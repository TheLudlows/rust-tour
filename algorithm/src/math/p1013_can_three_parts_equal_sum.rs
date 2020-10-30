use crate::Solution;

impl Solution {
    pub fn can_three_parts_equal_sum(a: Vec<i32>) -> bool {
        let total: i32 = a.iter().sum();
        if total % 3 != 0 {
            return false;
        }
        let sum = total / 3;

        let (mut i, mut j) = (0, a.len() - 1);
        let mut count1 = a[i];
        let mut count3 = a[j];
        while i < j && (count1 != sum || count3 != sum) {
            if count1 != sum {
                i += 1;
                count1 += a[i];
            }
            if count3 != sum {
                j -= 1;
                count3 += a[j];
            }
        }

        count1 == sum && count3 == sum && i + 1 < j
    }
}

#[test]
fn test() {
    let v = vec![1, -1, 1, -1];
    Solution::can_three_parts_equal_sum(v);
}
