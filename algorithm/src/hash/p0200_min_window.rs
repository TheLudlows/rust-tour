use std::collections::HashMap;

use crate::Solution;

// slide windows
impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let s = s.as_bytes();
        let t = t.as_bytes();
        let mut cnt = HashMap::new();
        let mut map = HashMap::new();
        for b in t {
            *cnt.entry(*b).or_insert(0) += 1;
        }
        let mut min_window = i32::MAX;
        let mut start = 0;
        let (mut left, mut right) = (0, 0);
        let mut matches: usize = 0;
        while right < s.len() {
            if cnt.contains_key(&s[right]) {
                *map.entry(s[right]).or_insert(0) += 1;
                if cnt.get(&s[right]).unwrap() >= map.get(&s[right]).unwrap() {
                    matches += 1;
                }
            }
            while matches == t.len() && left <= right {
                let windows = (right - left + 1) as i32;
                if min_window > windows {
                    start = left;
                    min_window = windows
                }
                if let Some(n) = map.get_mut(&s[left]) {
                    *n -= 1;
                    if *n < *cnt.get(&s[left]).unwrap() {
                        matches -= 1;
                    }
                }
                left += 1;
            }
            right += 1;
        }
        if start + min_window as usize > s.len() {
            return String::new();
        }
        String::from_utf8(s[start..start + min_window as usize].to_owned()).unwrap()
    }
}


#[test]
fn test() {
    let r = min_window("ADOBECODEBANC".to_owned(), "ABC".to_owned());
    println!("{}", r);
}

pub fn min_window(s: String, t: String) -> String {
    let s = s.as_bytes();
    let t = t.as_bytes();
    let mut cnt = vec![0; 128];
    let mut window_cnt = vec![0; 128];
    for b in t {
        cnt[*b as usize] += 1;
    }
    let mut min_window = i32::MAX;
    let mut start = 0;
    let (mut left, mut right) = (0, 0);
    let mut matches: usize = 0;
    while right < s.len() {
        let idx = s[right] as usize;
        if cnt[idx] == 0 {
            right += 1;
            continue;
        }
        window_cnt[idx] += 1;
        if cnt[idx] >= window_cnt[idx] {
            matches += 1;
        }
        while matches == t.len() && left <= right {
            let windows = (right - left + 1) as i32;
            if min_window > windows {
                start = left;
                min_window = windows
            }
            let left_idx = s[left] as usize;
            if window_cnt[left_idx] != 0 {
                window_cnt[left_idx] -= 1;
                if window_cnt[left_idx] < cnt[left_idx] {
                    matches -= 1;
                }
            }
            left += 1;
        }
        right += 1;
    }
    if start + min_window as usize > s.len() {
        return String::new();
    }
    String::from_utf8(s[start..start + min_window as usize].to_owned()).unwrap()
}