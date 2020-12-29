use std::cell::Cell;

#[test]
fn cell_test() {
    struct Foo {
        b: Cell<i32>,
        c: Cell<String>,
    }

    let f = Foo {
        b: Cell::new(20),
        c: Cell::new("c".to_string()),
    };
    assert_eq!(20, f.b.get());
    f.b.set(30);
    assert_eq!(30, f.b.get());
    f.c.set("ccc".to_string())
}
