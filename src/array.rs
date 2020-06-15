fn main(){
    let arr1:[u32;4] = [1,2,3,4];
    // 类型和长度可以省略
    let arr2 = ["a","b","c"];
    // 默认值初始化
    let mut arr3 = [0;2];
    println!("{}",arr3.len());
    for index in 0..2 {
        println!("index is: {} & value is : {}",index,arr3[index]);
    }
    change_arr(&mut arr3);
    for v in arr3.iter(){
        println!("{}",v);
    }
}
fn change_arr( arr: & mut [i32;2]) {
    arr[1] = 10;

}