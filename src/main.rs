mod c_lib_wrapper;

fn main() {
    let result = c_lib_wrapper::add(3, 5);
    println!("Result: {}", result);
}
