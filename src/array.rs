#[test]
fn new_test() {
    // 类型和长度可以省略
    let arr = ["a".to_string(), "b".to_string(), "c".to_string()];
    // 默认值初始化
    let arr = [0; 2];
    for (i, x) in arr.iter().enumerate() {
        println!("index is: {} & value is : {}", i, x);
    }
}

#[test]
fn change_test() {
    let mut arr = ["a".to_string(), "b".to_string(), "c".to_string()];
    change_str_arr(&mut arr);
    for v in arr.iter() {
        println!("{}", v);
    }
    let mut i32_arr = [1, 2];
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

#[test]
fn test() {
    let a = [1, 2, 3];
    let mut s: &[i32] = &a[1..2];
    println!("{}", s.len());
    let v = vec![1, 2, 3, 4];
    let vs = &v[0..2];
    println!("{}", vs.len());
}

#[test]
fn test_iter() {
    let mut arr = [1, 2, 3];
    for n in &arr {
        println!("{}", n)
    }

    for n in arr.iter() {
        println!("{}", n)
    }

    for n in (&arr).into_iter() {
        println!("{}", n)
    }
    for n in (&mut arr).iter_mut() {
        *n = *n * 2;
    }
    let p: &[i32] = &mut arr;

    arr[0] = 100;
    println!("{:?}", arr)
}

#[test]
fn test_any() {
    let arr = [1, 2, 3];
    assert_eq!(arr.iter().any(|x| *x > 3), false);
    assert_eq!((&arr).into_iter().any(|&x| x > 3), false);
}

#[test]
fn test_fold() {
    let arr = [1, 2, 3];
    assert_eq!(arr.iter().fold(0, |total, &x| x + total), 6);
}

#[test]
fn test_fold_vec() {
    let v = vec![1, 2, 3];
    assert_eq!(v.into_iter().fold(0, |total, x| x + total), 6);
}
