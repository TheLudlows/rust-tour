use crate::Solution;

impl Solution {
    pub fn hammingWeight(mut n: u32) -> i32 {
        let mut ret = 0;
        while n != 0 {
            ret += 1;
            n = n & (n - 1);
        }
        ret
    }
}

#[test]
fn test() {
    println!("{}", 5 & 1);
    let r:u32 = !7;
    println!("{}", r);

}