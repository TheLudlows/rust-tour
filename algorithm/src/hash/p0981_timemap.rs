use std::collections::{BTreeMap, HashMap};

struct TimeMap {
    map: BTreeMap<i32, HashMap<String, String>>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TimeMap {
    fn new() -> Self {
        Self {
            map: BTreeMap::new()
        }
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        self.map.entry(timestamp)
            .and_modify(|map| { map.insert(key.clone(), value.clone()); })
            .or_insert_with(|| {
                let mut map = HashMap::new();
                map.insert(key, value);
                map
            });
    }

    fn get(&self, key: String, timestamp: i32) -> String {
        let rag = self.map.range(..=timestamp).rev();

        for (_, map) in rag {
            match map.get(&key) {
                None => {}
                Some(v) => {
                    return v.clone();
                }
            }
        }
        return String::new();
    }
}

/**
 * Your TimeMap object will be instantiated and called as such:
 * let obj = TimeMap::new();
 * obj.set(key, value, timestamp);
 * let ret_2: String = obj.get(key, timestamp);
 */
#[test]
fn test() {
    let mut t = TimeMap::new();
    t.set("b".to_string(), "b".to_string(), 1);
    t.set("a".to_string(), "a".to_string(), 0);

    let ret = t.get("a".to_string(), 2);
    assert_eq!("a", ret)
}