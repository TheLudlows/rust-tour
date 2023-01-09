use rand::Rng;
use std::collections::HashMap;

struct RandomizedSet {
    nums: Vec<i32>,
    indices: HashMap<i32, usize>,
}

impl RandomizedSet {
    fn new() -> Self {
        Self {
            nums: Vec::new(),
            indices: HashMap::new(),
        }
    }

    fn insert(&mut self, val: i32) -> bool {
        if self.indices.contains_key(&val) {
            return false;
        }

        let index = self.nums.len();
        self.nums.push(val);
        self.indices.insert(val, index);
        true
    }

    fn remove(&mut self, key: i32) -> bool {
        if let Some(index) = self.indices.remove(&key) {
            let last_index = self.nums.len() - 1;
            if last_index != index {
                self.nums[index] = self.nums[last_index];
                self.indices.insert(self.nums[last_index], index);
            }
            self.nums.pop();
            return true;
        }

        false
    }

    fn get_random(&self) -> i32 {
        let mut rng = rand::thread_rng();
        let index: usize = rng.gen_range(0..self.nums.len());
        self.nums[index]
    }
}

#[test]
fn test() {
    let mut  r = RandomizedSet::new();
    r.insert(1);
    r.remove(1);
    let res = r.insert(1);
    println!("{}", res)
}
