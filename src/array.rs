fn main() {
    // 类型和长度可以省略
    let mut arr2 = ["a".to_string(), "b".to_string(), "c".to_string()];
    // 默认值初始化
    let mut arr3 = [0; 2];
    for index in 0..2 {
        println!("index is: {} & value is : {}", index, arr3[index]);
    }
    for v in arr2.iter() {
        println!("{}", v);
    }
    change_arr2(& mut arr2);

    for v in arr2.iter() {
        println!("{}", v);
    }

    change_arr(&mut arr3);
    for index in 0..2 {
        println!("index is: {} & value is : {}", index, arr3[index]);
    }
}

fn change_arr(arr: &mut [i32; 2]) {
    arr[1] = 10;
}

fn change_arr2(arr: &mut [String; 3]) {
    arr[1] = "e".to_string();
}