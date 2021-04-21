use crate::Solution;

impl Solution {
    pub fn can_place_flowers(mut flowerbed: Vec<i32>, mut n: i32) -> bool {
        let mut new_arr = vec![0];
        new_arr.append(&mut flowerbed);
        new_arr.push(0);

        for i in 1..new_arr.len() - 1 {
            if new_arr[i - 1] == 0 && new_arr[i + 1] == 0 && new_arr[i] == 0 {
                new_arr[i] = 1;
                n -= 1;
            }
        }
        n <= 0
    }
}