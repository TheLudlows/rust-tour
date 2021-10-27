use crate::Solution;

impl Solution {
    pub fn valid_utf8(data: Vec<i32>) -> bool {

        match data.len() {
            0 => false,
            1 => (data[0] >> 7) == 0,
            2 => (data[0] >> 5) == 6 && (data[1] >> 6) == 2,
            3 => (data[0] >> 4) == 14 && (data[1] >> 6) == 2 && (data[2] >> 6) == 2,
            4 => (data[0] >> 3) == 30 && (data[1] >> 6) == 2 && (data[2] >> 6) == 2 && (data[3] >> 6) == 2,
            _ => false
        }
    }
}
#[test]
fn test() {
   let r = Solution::valid_utf8(vec![197,130,1]);
    println!("{}", r)
}