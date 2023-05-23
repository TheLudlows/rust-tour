use std::collections::{HashMap, HashSet};

struct WordFilter {
    pre_map: HashMap<String, HashSet<usize>>,
    suff_map: HashMap<String, HashSet<usize>>,
    words:Vec<String>
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl   WordFilter  {
    fn new (words: Vec<String>) -> Self {
        let mut pre_map: HashMap<String, HashSet<usize>> = HashMap::new();
        let mut suff_map: HashMap<String, HashSet<usize>> = HashMap::new();
        for i in 0..words.len() {
            let s = &words[i];
            for j in 1..=s.len() {
                let sub = s[0..j].to_string();
                let suff = s[s.len() - j..].to_string();
                pre_map.entry(sub).or_default().insert(i);
                suff_map.entry(suff).or_default().insert(i);
            }
        }
        Self { pre_map, suff_map, words }
    }

    fn f(&self, pref: String, suff: String) -> i32 {
        let set1 = self.pre_map.get(&pref[..]);
        let set2 = self.suff_map.get(&suff[..]);
        if set1.is_none() || set2.is_none() {
            return -1;
        }
        let set1 = set1.unwrap();
        let set2 = set2.unwrap();
        let mut res = set1.iter().filter(|&e| set2.contains(e)).map(|e| *e).max();
        res.map_or(-1, |i| i as i32)
    }
}
#[test]
fn test() {

}


