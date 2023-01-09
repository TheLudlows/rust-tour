use std::collections::{BinaryHeap, BTreeSet, HashMap, HashSet};

struct NumberContainers {
    map:HashMap<i32, i32>,
    idx_map:HashMap<i32, BTreeSet<i32>>
}

impl NumberContainers {

    fn new() -> Self {
        Self {
            map: Default::default(),
            idx_map: Default::default(),
        }
    }

    fn change(&mut self, index: i32, number: i32) {
        match self.map.insert(index, number) {
            None => {}
            Some(old_num) => {
                self.idx_map.entry(old_num).or_default().remove(&index);
            }
        }
        self.idx_map.entry(number).or_default().insert(index);
    }

    fn find(&self, number: i32) -> i32 {
        if let Some(set) = self.idx_map.get(&number) {
            return  set.range(..).next().map_or(-1, |e| *e);
        }
        -1
    }
}

#[test]
fn test() {
    let mut n = NumberContainers::new();
    n.change(1, 2);
    n.find(2);
}