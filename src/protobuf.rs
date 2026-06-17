pub mod playground {
    pub mod objects {
        include!(concat!(env!("OUT_DIR"), "/playground.objects.rs"));
    }

    include!(concat!(env!("OUT_DIR"), "/playground.rs"));
}

#[allow(dead_code)]
pub fn example_user() -> playground::User {
    playground::User {
        name: "Martin".to_string(),
        age: 20,
        email: Some("martin@example.com".to_string()),
        main_address: Some(playground::Adress {
            street: "Example Street 1".to_string(),
            city: "Berlin".to_string(),
            zip: "10115".to_string(),
        }),
        additional_addresses: Vec::new(),
        sex: playground::Sex::Male as i32,
        cars: vec![playground::objects::Car {
            make: "Volkswagen".to_string(),
            model: "Golf".to_string(),
            year: 2024,
        }],
    }
}

#[allow(dead_code)]
pub fn encode_user(user: &playground::User) -> Vec<u8> {
    prost::Message::encode_to_vec(user)
}

#[allow(dead_code)]
pub fn decode_user(bytes: &[u8]) -> Result<playground::User, prost::DecodeError> {
    prost::Message::decode(bytes)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn encode_decode_user() {
        let user = example_user();
        let encoded = encode_user(&user);
        let decoded = decode_user(&encoded).unwrap();

        assert_eq!(decoded, user);
    }
}
