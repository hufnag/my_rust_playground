mod c_lib_wrapper;
mod cpp_lib_wrapper;

fn main() {
    let result = c_lib_wrapper::add(3, 5);
    println!("Result: {}", result);

    let mut user_ages = cpp_lib_wrapper::UserAgeTable::new();
    user_ages.add("Martin", 20).unwrap();
    user_ages.add("Jens", 36).unwrap();

    if let Some(age) = user_ages.get("Martin") {
        println!("Martin is {age} years old");
    }
}
