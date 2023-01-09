use std::collections::BTreeMap;

struct Fancy {
    vec: Vec<i32>,
    tree: BTreeMap<usize, Vec<Op>>,
}

struct Op {
    op_type: Op_type,
    val: i32,
}
#[derive(PartialEq)]
enum Op_type {
    ADD,
    MUL,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Fancy {
    fn new() -> Self {
        Self {
            vec: vec![],
            tree: Default::default(),
        }
    }

    fn append(&mut self, val: i32) {
        self.vec.push(val)
    }

    fn add_all(&mut self, inc: i32) {
        let mut v = self.tree.entry(self.vec.len()).or_default();
        let op = Op{ op_type: Op_type::ADD, val: inc };
        match v.last_mut() {
            None => {
                v.push(op);
            }
            Some(last) => {
                let res =  ((last.val as i64 + inc as i64) % 1000000007) as i32;
                if last.op_type == Op_type::ADD {
                    last.val = res as i32;
                } else {
                    v.push(op);
                }
            }
        }
    }

    fn mult_all(&mut self, m: i32) {

       let mut v = self.tree.entry(self.vec.len()).or_default();
        let op = Op{ op_type: Op_type::MUL, val: m };
        match v.last_mut() {
            None => {
                v.push(op);
            }
            Some(last) => {
                let res =  ((last.val as i64 * m as i64) % 1000000007) as i32;
                if last.op_type == Op_type::MUL {
                    last.val = res as i32;
                } else {
                    v.push(op);
                }
            }
        }
    }

    fn get_index(&self, idx: i32) -> i32 {
        let idx = idx as usize;
        if idx >= self.vec.len() {
            return -1;
        }
        let mut n = self.vec[idx] as i64;
        for (_, ops) in self.tree.range(idx+1..) {
            for op in ops {
                if op.op_type == Op_type::ADD {
                    n += op.val as i64;
                } else {
                    n *= op.val as i64;
                }
                n %= 1000_000_007
            }
        }

        n as i32
    }
}
#[test]
fn test() {
    let mut f = Fancy::new();
    f.append(2);
    f.add_all(3);
    f.append(7);
    f.mult_all(2);

    let r = f.get_index(0);
    println!("{}", r);
}