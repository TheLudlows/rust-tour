use std::fs::read;

struct Bitset {
    set: Vec<u64>,
    size:usize,
    one_count:usize
}

fn find_idx(idx:usize) -> usize {
    idx as usize / 64
}
fn find_bit(idx:usize) -> usize {
    63 - (idx as usize % 64)
}

impl Bitset {
    fn new(rel_size: i32) -> Self {
        let size = ((rel_size as usize - 1) / 64 + 1);
        let mut set = Vec::with_capacity(size);

        for _ in 0.. size {
            set.push(0);
        }
        Self {
            set,
            size:rel_size as usize,
            one_count:0
        }
    }

    fn fix(&mut self, idx: i32) {
        let bkt = find_idx(idx as usize);
        let bkt_idx = find_bit(idx as usize);
        if self.set[bkt] >> bkt_idx & 1 == 0 {
            self.one_count+=1;
            self.set[bkt]  |= 1 << bkt_idx;
        }
    }


    fn unfix(&mut self, idx: i32) {
        let bkt = find_idx(idx as usize);
        let bkt_idx = find_bit(idx as usize);
        if self.set[bkt] >> bkt_idx & 1 == 1 {
            self.one_count -= 1;
            self.set[bkt] &= !(1 << bkt_idx);
        }
    }

    fn flip(&mut self) {
        for i in 0..self.set.len() {
            self.set[i] = !self.set[i]
        }
        self.one_count = self.size - self.one_count;

    }

    fn all(&self) -> bool {
       self.one_count == self.size
    }



    fn one(&self) -> bool {
        self.count() > 0
    }

    fn count(&self) -> i32 {
       self.one_count as i32
    }

    fn to_string(&self) -> String {
        let mut str = String::new();
        for u in &self.set {
            //println!("{}", format!("{u:064b}"));
            str.push_str(&format!("{u:064b}"));
        }
        //println!("{}", str);
        for i in self.size..self.set.len() * 64 {
            str.pop().unwrap();
        }
        str
    }
}
#[test]
fn main() {
   let mut b = Bitset::new(65);
    b.flip();
    println!("{}", b.to_string());
    println!("{}", b.one());
    println!("{}", b.all());
}