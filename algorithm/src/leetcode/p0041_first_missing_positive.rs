

/// 数组内的元素按照值的大小进行归位，0..len之间的数字放入对应的索引位置，小于和大于len的不处理。
impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        nums.push(0);//长度加1
        let len = nums.len() as i32;
        for i in 0..nums.len() {
            let mut j = nums[i];
            while 0 <= j && j < len && nums[j as usize] != j {
                std::mem::swap(&mut nums[j as usize], &mut j);
                nums[i] = j;
            }
        }

        for (i, n) in nums.into_iter().enumerate() {
            if n != i as i32 {
                return i as i32;
            }
        }
        len
    }
}

#[test]
fn test() {
    let v = vec![8, 10, 4, 3, 1, 2];
    Solution::first_missing_positive(v);
}