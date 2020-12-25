use crate::Solution;

impl Solution {
    pub fn find_content_children(mut g: Vec<i32>, mut s: Vec<i32>) -> i32 {
        g.sort();
        s.sort();
        let mut res = 0;
        let mut s = s.iter();
        for i in &g {
            while let Some(n) = s.next() {
                if n >= i {
                    res+=1;
                    break;
                }
            }
        }
        res
    }
}