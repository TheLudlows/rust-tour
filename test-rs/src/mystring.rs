use std::{any::type_name_of_val, fmt, ops::Deref, str};

const MINI_STRING_MAX_LEN: usize = 30;

// MyString 里，String 有 3 个 word，供 24 字节，所以它以 8 字节对齐
// 所以 enum 的 tag + padding 最少 8 字节，整个结构占 32 字节。
// MiniString 可以最多有 30 字节（再加上 1 字节长度和 1字节 tag），就是 32 字节.
struct MiniString {
    len: u8,
    data: [u8; MINI_STRING_MAX_LEN],
}

impl MiniString {
    // 这里 new 接口不暴露出去，保证传入的 v 的字节长度小于等于 30
    fn new(v: impl AsRef<str>) -> Self {
        let bytes = v.as_ref().as_bytes();
        // 我们在拷贝内容时一定要要使用字符串的字节长度
        let len = bytes.len();
        let mut data = [0u8; MINI_STRING_MAX_LEN];
        data[..len].copy_from_slice(bytes);
        Self {
            len: len as u8,
            data,
        }
    }
}

impl Deref for MiniString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        // 由于生成 MiniString 的接口是隐藏的，它只能来自字符串，所以下面这行是安全的
        str::from_utf8(&self.data[..self.len as usize]).unwrap()
        // 也可以直接用 unsafe 版本
        // unsafe { str::from_utf8_unchecked(&self.data[..self.len as usize]) }
    }
}

impl fmt::Debug for MiniString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 这里由于实现了 Deref trait，可以直接得到一个 &str 输出
        write!(f, "{}", self.deref())
    }
}

#[derive(Debug)]
enum MyString {
    Inline(MiniString),
    Standard(String),
}

// 实现 Deref 接口对两种不同的场景统一得到 &str
impl Deref for MyString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        match *self {
            MyString::Inline(ref v) => v.deref(),
            MyString::Standard(ref v) => v.deref(),
        }
    }
}

impl<S> From<S> for MyString where S : AsRef<str> + {
    fn from(s: S) -> Self {
        match s.as_ref().len() > MINI_STRING_MAX_LEN {
            true => Self::Standard(String::from(s.as_ref())),
            _ => Self::Inline(MiniString::new(s)),
        }
    }
}

impl fmt::Display for MyString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.deref())
    }
}
impl MyString {
    pub fn push_str(&mut self, string: &str) {
        match self {
            MyString::Inline(mini) => {
                let idx = mini.len as usize;
                let len = string.len();
                if idx + len <= MINI_STRING_MAX_LEN {
                    mini.data[idx..idx+len].copy_from_slice(string.as_bytes());
                    mini.len += string.len() as u8;
                }else {
                    *self = MyString::Standard(String::from(self.to_string() + string));
                }
            }
            MyString::Standard(stad)  => stad.push_str(string),
        };
    }
}
#[test]
fn main() {
    let len1 = std::mem::size_of::<MyString>();
    let len2 = std::mem::size_of::<MiniString>();
    let len3 = std::mem::size_of::<String>();

    println!("Len: MyString {}, MiniString {} {}", len1, len2, len3);

    let s1: MyString = "hello world".into();
    let s2: MyString = "这是一个超过了三十个字节的很长很长的字符串".into();
    let s3 = MyString::from("aaa");
    let s4 = MyString::from(String::from("aa"));
    let s5 = MyString::from("b".to_owned());
    println!("{}", type_name_of_val(&s3));

    // debug 输出
    println!("s1: {:?}, s2: {:?}", s1, s2);
    // display 输出
    println!(
        "s1: {}({} bytes, {} chars), s2: {}({} bytes, {} chars)",
        s1,
        s1.len(),
        s1.chars().count(),
        s2,
        s2.len(),
        s2.chars().count()
    );

    // MyString 可以使用一切 &str 接口，感谢 Rust 的自动 Deref
    assert!(s1.ends_with("world"));
    assert!(s2.starts_with("这"));
}
