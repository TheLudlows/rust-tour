fn main(){
    let arr1:[u32;4] = [1,2,3,4];
    // 类型和长度可以省略
    let arr2 = ["a","b","c"];
    // 默认值初始化
    let arr3 = [2,5];
    println!("{}",arr3.len());
    for index in 0..4 {
        println!("index is: {} & value is : {}",index,arr1[index]);
    }
}