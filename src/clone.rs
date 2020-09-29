#[derive(Eq, PartialEq, Debug)]
struct Foo {
    name: String
}

impl Clone for Foo {
    fn clone(&self) -> Self {
        Foo {
            name: String::from(&self.name)
        }
    }
}


#[derive(Clone, Eq, PartialEq, Debug)]
struct Boo {
    name: String
}

#[test]
fn clone_test() {
    let f = Foo { name: "rust".to_string() };
    let b = Boo { name: "rust".to_string() };
    assert_eq!(f, f.clone());
    assert_eq!(b, b.clone())
}