struct NumArray {
    sum: Vec<i32>
}
/// 缓存0-i的和
impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let mut sum = vec![0];
        for i in nums {
            sum.push(i + sum.last().unwrap());
        }
        Self{
            sum
        }
    }

    fn sum_range(&self, i: i32, j: i32) -> i32 {
        self.sum[(j+1) as usize] - self.sum[i as usize]
    }
}