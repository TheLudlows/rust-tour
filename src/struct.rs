#![allow(unused_variables)]

struct User {
    username: String ,
    email:  String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let mut a = "a";

    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    println!("{}{}",user1.username,user1.email);

    let user2 = build_user(String::from("abc.com"),String::from("four"));

    let user3 = User{
        ..user1
    };
    println!("{}", user3.username);

    struct person (string,string,usize);
    let p1 = person(String::from("four"),String::from("ai"),12);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

