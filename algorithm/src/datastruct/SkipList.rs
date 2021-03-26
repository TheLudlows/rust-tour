use std::cell::RefCell;
use std::rc::Rc;
use std::ptr::NonNull;

#[derive(Debug)]
struct Node {
    next: Vec<Option<Rc<RefCell<Node>>>>,
    key: i32,
}

impl Node {
    pub fn new(key: i32, n: usize) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            key,
            next: vec![None; n],
        }))
    }
}

struct SkipList {
    max_level: usize,
    head: Vec<Option<Rc<RefCell<Node>>>>,
    size: usize,
    cur_level: usize,
}

impl SkipList {
    pub fn new(max_level: usize) -> Self {
        SkipList {
            max_level: max_level - 1,
            head: vec![None; max_level],
            size: 0,
            cur_level: 0,
        }
    }

    fn rand_level(&self) -> usize {
        let mut level = 0;
        while rand::random::<bool>() && level < self.max_level {
            level += 1;
        }
        level
    }

    pub fn put(&mut self, key: i32, v: String) {
        let level = self.rand_level();
        let newNode = Node::new(key, level + 1);
        let updateNode = &self.head;
        for i in (0..=self.cur_level).rev() {}
        for i in 0..level {}
    }

    pub fn get(&self, key: i32) -> Option<String> {
        Some(String::new())
    }

    fn find_level(mut node: &Option<Rc<RefCell<Node>>>, level: usize, key: i32) -> Option<Rc<RefCell<Node>>> {
        while let Some(realNode) = node {
            if realNode.borrow().key < key {
                node = (realNode.borrow().next[level]);
            }
            NonNull::new
        }
        None
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::datastruct::SkipList::SkipList;

    #[test]
    fn test_rand() {
        let mut t = 0;
        let mut f = 0;
        for i in 0..1000 {
            if rand::random::<bool>() {
                t += 1;
            } else {
                f += 1;
            }
        }
        println!("true:{} false:{}", t, f);
    }

    #[test]
    fn test_rand_level() {
        let sl = SkipList::new(10);
        let mut map = HashMap::new();
        for i in 0..1000 {
            let l = sl.rand_level();
            let mut count = map.entry(l.clone()).or_insert(0);
            *count += 1;
        }
        println!("{:?}", map)
    }
}