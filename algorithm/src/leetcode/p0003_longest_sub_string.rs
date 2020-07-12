struct Solution;
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut max_len  = 0;
        0
    }
    fn indexOf(arr:&[char], c:char, start:usize,end:usize) -> i32 {
        for i in start..=end {
            if arr[i] == c {
               return  i as i32
            }
        }
        -1 as i32
    }
}
