use std::{
    pin::Pin,
    sync::{Arc, Mutex},
};

use tokio_stream::{
    Stream,
    wrappers::{ReceiverStream, TcpListenerStream},
};
use tonic::{Request, Response, Status, transport::Server};

pub use crate::protobuf::playground as proto;
use crate::protobuf::{example_user, playground::test_service_client::TestServiceClient};

#[derive(Debug, Default, Clone)]
struct TestServiceImplementation {
    users: Arc<Mutex<Vec<proto::User>>>,
}

impl TestServiceImplementation {
    fn with_users(users: Vec<proto::User>) -> Self {
        Self {
            users: Arc::new(Mutex::new(users)),
        }
    }
}

type ResponseStream<T> = Pin<Box<dyn Stream<Item = Result<T, Status>> + Send>>;

#[tonic::async_trait]
impl proto::test_service_server::TestService for TestServiceImplementation {
    type GetUsersStream = ResponseStream<proto::GetUsersResponse>;
    type LoopBackStream = ResponseStream<proto::LoopBackResponse>;

    async fn add_user(
        &self,
        request: Request<proto::AddUserRequest>,
    ) -> Result<Response<proto::AddUserResponse>, Status> {
        let user = request
            .into_inner()
            .user
            .ok_or_else(|| Status::invalid_argument("missing user"))?;

        let mut users = self.users.lock().unwrap();
        users.push(user);

        Ok(Response::new(proto::AddUserResponse {
            id: users.len() as i32,
        }))
    }

    async fn get_user(
        &self,
        request: Request<proto::GetUserRequest>,
    ) -> Result<Response<proto::GetUserResponse>, Status> {
        let id = request.into_inner().id;
        let user = self
            .users
            .lock()
            .unwrap()
            .get((id - 1) as usize)
            .cloned()
            .ok_or_else(|| Status::not_found(format!("user {id} not found")))?;

        Ok(Response::new(proto::GetUserResponse { user: Some(user) }))
    }

    async fn add_users(
        &self,
        request: Request<tonic::Streaming<proto::AddUsersRequest>>,
    ) -> Result<Response<proto::AddUsersResponse>, Status> {
        let mut stream = request.into_inner();
        let mut ids = Vec::new();

        while let Some(request) = stream.message().await? {
            let Some(user) = request.user else {
                return Err(Status::invalid_argument("missing user"));
            };

            let mut users = self.users.lock().unwrap();
            users.push(user);
            ids.push(users.len() as i32);
        }

        Ok(Response::new(proto::AddUsersResponse { ids }))
    }

    async fn get_users(
        &self,
        request: Request<proto::GetUsersRequest>,
    ) -> Result<Response<Self::GetUsersStream>, Status> {
        let ids = request.into_inner().ids;
        let users = self.users.clone();
        let (sender, receiver) = tokio::sync::mpsc::channel(16);

        tokio::spawn(async move {
            for id in ids {
                tokio::time::sleep(std::time::Duration::from_millis(500)).await;

                let user = users.lock().unwrap().get((id - 1) as usize).cloned();
                let Some(user) = user else {
                    continue;
                };

                let response = Ok(proto::GetUsersResponse { users: Some(user) });
                if sender.send(response).await.is_err() {
                    break;
                }
            }
        });

        Ok(Response::new(Box::pin(ReceiverStream::new(receiver))))
    }

    async fn loop_back(
        &self,
        request: Request<tonic::Streaming<proto::LoopBackRequest>>,
    ) -> Result<Response<Self::LoopBackStream>, Status> {
        let mut stream = request.into_inner();
        let (sender, receiver) = tokio::sync::mpsc::channel(16);

        tokio::spawn(async move {
            loop {
                let message = match stream.message().await {
                    Ok(Some(request)) => Ok(proto::LoopBackResponse { user: request.user }),
                    Ok(None) => break,
                    Err(status) => Err(status),
                };

                if sender.send(message).await.is_err() {
                    break;
                }
            }
        });

        Ok(Response::new(Box::pin(ReceiverStream::new(receiver))))
    }
}

async fn spawn_test_server(
    service: TestServiceImplementation,
) -> TestServiceClient<tonic::transport::Channel> {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();

    let server = Server::builder()
        .add_service(proto::test_service_server::TestServiceServer::new(service))
        .serve_with_incoming(TcpListenerStream::new(listener));

    tokio::spawn(server);

    TestServiceClient::connect(format!("http://{address}"))
        .await
        .unwrap()
}

#[tokio::test]
async fn add_user_rpc_adds_one_user() {
    let mut client = spawn_test_server(TestServiceImplementation::default()).await;
    let user = example_user();

    let add_response = client
        .add_user(proto::AddUserRequest { user: Some(user) })
        .await
        .unwrap()
        .into_inner();

    assert_eq!(add_response.id, 1);
}

#[tokio::test]
async fn get_user_rpc_returns_existing_user() {
    let user = example_user();
    let mut client =
        spawn_test_server(TestServiceImplementation::with_users(vec![user.clone()])).await;

    let get_response = client
        .get_user(proto::GetUserRequest { id: 1 })
        .await
        .unwrap()
        .into_inner();

    assert_eq!(get_response.user, Some(user));
}

#[tokio::test]
async fn add_users_rpc_accepts_client_stream() {
    let user = example_user();

    let mut anna = example_user();
    anna.name = "Anna".to_string();

    let mut jens = example_user();
    jens.name = "Jens".to_string();

    let mut client = spawn_test_server(TestServiceImplementation::with_users(vec![user])).await;
    let add_users_stream = tokio_stream::iter([
        proto::AddUsersRequest {
            user: Some(anna.clone()),
        },
        proto::AddUsersRequest {
            user: Some(jens.clone()),
        },
    ]);

    let add_users_response = client
        .add_users(add_users_stream)
        .await
        .unwrap()
        .into_inner();

    assert_eq!(add_users_response.ids, vec![2, 3]);
}

#[tokio::test]
async fn get_users_rpc_returns_server_stream() {
    let user = example_user();

    let mut anna = example_user();
    anna.name = "Anna".to_string();

    let mut jens = example_user();
    jens.name = "Jens".to_string();

    let mut client = spawn_test_server(TestServiceImplementation::with_users(vec![
        user.clone(),
        anna.clone(),
        jens.clone(),
    ]))
    .await;

    let started_at = std::time::Instant::now();
    let mut get_users_stream = client
        .get_users(proto::GetUsersRequest { ids: vec![1, 2, 3] })
        .await
        .unwrap()
        .into_inner();

    let mut streamed_users = Vec::new();
    while let Some(response) = get_users_stream.message().await.unwrap() {
        streamed_users.push(response.users.unwrap());
    }

    assert!(started_at.elapsed() >= std::time::Duration::from_millis(1_500));

    assert_eq!(
        streamed_users,
        vec![user.clone(), anna.clone(), jens.clone()]
    );
}

#[tokio::test]
async fn loop_back_rpc_returns_bidirectional_stream() {
    let user = example_user();

    let mut anna = example_user();
    anna.name = "Anna".to_string();

    let mut jens = example_user();
    jens.name = "Jens".to_string();

    let mut client = spawn_test_server(TestServiceImplementation::default()).await;

    let (sender, receiver) = tokio::sync::mpsc::channel(1);
    sender
        .send(proto::LoopBackRequest {
            user: Some(user.clone()),
        })
        .await
        .unwrap();

    let mut loop_back_response_stream = client
        .loop_back(ReceiverStream::new(receiver))
        .await
        .unwrap()
        .into_inner();

    let mut looped_back_users = Vec::new();
    let first_response = loop_back_response_stream.message().await.unwrap().unwrap();
    looped_back_users.push(first_response.user.unwrap());

    sender
        .send(proto::LoopBackRequest {
            user: Some(anna.clone()),
        })
        .await
        .unwrap();
    sender
        .send(proto::LoopBackRequest {
            user: Some(jens.clone()),
        })
        .await
        .unwrap();
    drop(sender);

    while let Some(response) = loop_back_response_stream.message().await.unwrap() {
        looped_back_users.push(response.user.unwrap());
    }

    assert_eq!(looped_back_users, vec![user, anna, jens]);
}
