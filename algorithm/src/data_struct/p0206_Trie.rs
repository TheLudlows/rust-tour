use std::borrow::Borrow;

#[derive(Debug)]
#[derive(Default)]
struct Trie {
    children: [Option<Box<Trie>>; 26],
    end: bool,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {
    fn new() -> Self {
        Trie::default()
    }

    fn insert(&mut self, word: String) {
        let mut cur = self;
        let mut i = 0;
        for c in word.bytes() {
            i = (c - b'a') as usize;
            cur = cur.children[i].get_or_insert(Box::new(Trie::new()));
        }
        cur.end = true;
    }

    fn search(&self, word: String) -> bool {
        let mut cur = self;
        let mut i = 0;
        for c in word.bytes() {
            i = (c - b'a') as usize;
            if cur.children[i].is_none() {
                return false;
            } else {
                cur = cur.children[i].as_ref().unwrap().borrow();
            }
        }
        cur.end
    }

    fn starts_with(&self, prefix: String) -> bool {
        let mut cur = self;
        let mut i = 0;
        for c in prefix.bytes() {
            i = (c - b'a') as usize;
            if cur.children[i].is_none() {
                return false;
            } else {
                cur = cur.children[i].as_ref().unwrap().borrow();
            }
        }
        true
    }
}
#[test]
fn test() {
  let mut t = Trie::new();
    t.insert(String::from("abc"));
    println!("{:?}", t);
}
