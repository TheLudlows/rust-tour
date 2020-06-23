fn main(){
    let arr1:[u32;4] = [1,2,3,4];
    // 类型和长度可以省略
    let arr2 = ["a".to_string(),"b".to_string(),"c".to_string()];
    // 默认值初始化
    let mut arr3 = [0;2];
    println!("{}",arr3.len());
    for index in 0..2 {
        println!("index is: {} & value is : {}",index,arr3[index]);
    }
    for v in arr2.iter(){
        println!("{}",v);
    }
    let changed = change_arr2(arr2);

    for v in changed.iter(){
        println!("{}",v);
    }
}
fn change_arr( arr: & mut [i32;2]) {
    arr[1] = 10;

}

fn change_arr2( mut arr: [String;3]) -> [String;3] {
    arr[1] = "e".to_string();
    arr
}