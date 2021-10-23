use crate::Solution;


/// a ^ b = b ^ a
/// a ^ 0 = a
/// s[i] ^ s[i+1] = t[i]
/// s[i+1] = t[i] ^ s[i];

impl Solution {
    pub fn decode(t: Vec<i32>, first: i32) -> Vec<i32> {
        let mut s = vec![first];
        for i in 1..t.len()+1 {
            s.push(t[i-1] ^ s[i-1]);
        }
        s
    }
}
#[test]
fn test() {

}