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
use std::fs::File;
use std::thread;
use std::io::Read;
use std::cmp::min;

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

// 只接受带有 'a 生命周期注解的引用类型
fn t_ref<'a,T>(t: &'a T) {}

// 接受满足 'a 生命周期约束的任何类型
fn t_bound<'a, T: 'a>(t: T) {}

// 内部含有引用的所有权类型
struct Ref<'a, T: 'a>(&'a T);

#[test]
fn test2() {
    let string = String::from("string");

    t_bound(&string); // 编译通过
    t_bound(Ref(&string)); // 编译通过
    t_bound(&Ref(&string)); // 编译通过

    t_ref(&string); // 编译通过
    //t_ref(Ref(&string)); // 编译失败，期望得到引用，实际得到 struct
    t_ref(&Ref(&string)); // 编译通过

    // 满足 'static 约束的字符串变量可以转换为 'a 约束
    t_bound(string); // 编译通过
}
#[test]
fn test3() {
    pub fn read_in_background(f: &'static mut File) {
        thread::spawn(move || {
            let mut buf = Vec::<u8>::new();
            if let Ok(count) = f.read_to_end(&mut buf) {
                println!("read {} bytes from file.", count);
            }
        });
    }

    let mut _f = File::open("/tmp/1.log").unwrap();
    //read_in_background(&mut f);
}

#[test]
fn test4() {
    struct MyCursor<'a> {
        data: &'a [u8],
    }

    impl<'a> Read for MyCursor<'a> {
        fn read(&mut self, buf: &mut [u8]) -> std::result::Result<usize, std::io::Error> {
            let size = min(buf.len(), self.data.len());
            buf[..size].copy_from_slice(&self.data[..size]);
            Ok(size)
        }
    }
}