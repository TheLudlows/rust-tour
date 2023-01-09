use std::collections::HashMap;
use std::hash::Hash;
use std::sync::atomic::AtomicI32;
use std::sync::atomic::Ordering::Release;

struct Codec {
    map:HashMap<String,i32>,
    rev_map:HashMap<i32, String>,
    atomic:AtomicI32
}

/**
  * `&self` means the method takes an immutable reference
  * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    fn new() -> Self {
        Self {
            map: Default::default(),
            rev_map: Default::default(),
            atomic: Default::default(),
        }
        
    }

    // Encodes a URL to a shortened URL.
    fn encode(&mut self, longURL: String) -> String {
        if let Some(url)  = self.map.get(&longURL) {
            return  format!("{}", url);
        }
        let i = self.atomic.fetch_add(1, Release);
        self.map.insert(longURL.clone(), i.clone());
        self.rev_map.insert(i, longURL);
        return format!("{}", i)
    }

    // Decodes a shortened URL to its original URL.
    fn decode(&self, shortURL: String) -> String {
        self.rev_map.get(&shortURL.parse().unwrap()).map_or("".to_string(), |e| e.clone())
    }
}
#[test]
fn test() {
    
}
