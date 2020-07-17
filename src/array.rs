
#[test]
fn new_test() {
    // 类型和长度可以省略
    let arr = ["a".to_string(), "b".to_string(), "c".to_string()];
    // 默认值初始化
    let arr = [0; 2];
    for (i,x) in arr.iter().enumerate() {
        println!("index is: {} & value is : {}", i, x);
    }
}

#[test]
fn change_test() {
    let mut arr = ["a".to_string(), "b".to_string(), "c".to_string()];
    change_str_arr(& mut arr);
    for v in arr.iter() {
        println!("{}", v);
    }
    let mut i32_arr = [1,2];
    change_arr(&mut i32_arr);
    for index in 0..2 {
        println!("index is: {} & value is : {}", index, i32_arr[index]);
    }
}

fn change_arr(arr: &mut [i32; 2]) {
    arr[1] = 10;
}
fn change_str_arr(arr: &mut [String; 3]) {
    //两种方式
    arr[1] = "e".to_string();
    arr[1].push_str("rust");
}