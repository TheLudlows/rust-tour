#![allow(unused_variables)]

fn main() {
    struct User {
        username: String ,
        email:  String,
        sign_in_count: u64,
        active: bool,
    }

    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.username = String::from("four");
    println!("{}{}",user1.username,user1.email)

}