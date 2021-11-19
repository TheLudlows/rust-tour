use std::collections::{BTreeMap, HashMap};

#[test]
fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let field_name = String::from("Favorite color");
    let mut map = HashMap::new();
    map.insert(field_name, "Blue");
    //println!("{}",field_name)

    let o = map.get(&("Favorite color".to_string()));
    println!("{}", o.unwrap());

    for (k, v) in map {
        println!("{},{}", k, v)
    }

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    let v = scores.entry(String::from("Yellow")).or_insert(50);
    *v += 100;
    println!("{}", scores.get(&String::from("Yellow")).unwrap());
}

#[test]
fn test_mut() {
    let mut map = HashMap::new();
    let key = "key";
    match map.get(key) {
        Some(v) => {
            println!("{}", v);
        }
        None => {
            map.insert(key, "v");
        }
    }
}
#[test]
fn test_mem() {
    let map = HashMap::new();
    let mut map = explain("empty", map);

    map.insert('a', 1);
    let mut map = explain("added 1", map);
    map.insert('b', 2);
    map.insert('c', 3);

    let mut map = explain("added 3", map);

    map.insert('d', 4);

    let mut map = explain("added 4", map);

    map.remove(&'a');

    explain("final", map);
}

// HashMap 结构有两个 u64 的 RandomState，然后是四个 usize，
// 分别是 bucket_mask, ctrl, growth_left 和 items
// 我们 transmute 打印之后，再 transmute 回去
fn explain<K, V>(name: &str, map: HashMap<K, V>) -> HashMap<K, V> {
    let arr: [usize; 6] = unsafe { std::mem::transmute(map) };
    println!(
        "{}: bucket_mask 0x{:x}, ctrl 0x{:x}, growth_left: {}, items: {}",
        name, arr[2], arr[3], arr[4], arr[5]
    );
    unsafe { std::mem::transmute(arr) }
}
#[derive(Debug, Eq)]
struct Name {
    pub name: String,
    pub flags: u32,
}

impl Ord for Name {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
       (&self.name, &self.flags).cmp(&(&other.name, &other.flags))
    }
}
impl PartialOrd for Name {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
       Some(self.name.cmp(&other.name))
    }
}
impl PartialEq for Name {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.flags == other.flags
    }
}

impl Name {
    pub fn new(name: impl AsRef<str>, flags: u32) -> Self {
        Self {
            name: name.as_ref().to_string(),
            flags,
        }
    }
}
#[test]
fn main2() {
    let mut map = BTreeMap::new();
    map.insert(Name::new("/etc/password", 0x1), 12);
    map.insert(Name::new("/etc/hosts", 0x1), 4);
    map.insert(Name::new("/home/tchen", 0x0), 28);
    
    for item in map.iter() {
        println!("{:?}", item);
    }
}
