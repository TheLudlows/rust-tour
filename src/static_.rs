static BYTES: [u8; 3] = [1, 2, 3];
static mut MUT_BYTES: [u8; 3] = [1, 2, 3];
#[test]
fn main() {
    //MUT_BYTES[0] = 99; // 编译错误，修改静态变量是 unsafe 的
    println!("{:?}",BYTES);
    //println!("{:?}",MUT_BYTES);

    unsafe {
        MUT_BYTES[0] = 99;
        assert_eq!(99, MUT_BYTES[0]);
    }
}

use rand;
use std::fmt::Debug;

// 在运行时生成随机 &'static str
fn rand_str_generator() -> &'static str {
    let rand_string = rand::random::<u64>().to_string();
    Box::leak(rand_string.into_boxed_str())
}



#[derive(Debug)]
struct Stru;
trait Fuck<T> {
    fn fuck(&self,t:T);
}

impl <T: Debug> Fuck<T> for Stru {
    fn fuck(&self, t: T) {
        println!("{:?}",t)
    }
}

#[test]
fn test() {
    Stru.fuck("123");
}