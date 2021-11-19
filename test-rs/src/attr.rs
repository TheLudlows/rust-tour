// `#[allow(dead_code)]` 属性可以禁用 `dead_code` lint
#[allow(dead_code)]
fn unused_function() {}

fn used_function() {}

#[cfg(some_condition)]
fn conditional_function() {
    println!("condition met!")
}

#[test]
fn test() {
    used_function();
}