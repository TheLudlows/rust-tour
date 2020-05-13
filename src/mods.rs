mod my_mod {
    pub struct person {
        pub name: String
    }

    impl person {
        pub fn new(name_str: String) -> person {
            person {
                name: name_str
            }
        }
    }

    pub enum sex {
        m,
        f,
    }
}

//use crate::mods::my_mod::person;
//use crate::mods::my_mod::sex;
//use crate::mods::my_mod::*;
use my_mod::*;
//use my_mod as m;


fn main() {
    let p = person::new(String::from("four"));
    println!("{}", p.name);
    let sex = sex::m;
}