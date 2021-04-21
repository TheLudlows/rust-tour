use crate::Solution;

impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut max_area = 0;
        if heights.len() == 1 {
            return heights[0];
        }
        for i in 0..heights.len() {
            let h = heights[i];

            let mut left = i;
            let mut right = i;
            while left > 0 && heights[left - 1] >= h {
                left -= 1;
            }
            while right < heights.len() - 1 && heights[right + 1] >= h {
                right += 1;
            }
            max_area = std::cmp::max(max_area, h * (right - left + 1) as i32);
        }
        max_area
    }
}

#[test]
fn test() {
    let v = vec![0, 3, 2, 5];
    let r = largest_rectangle_area3(v);
    println!("{}", r)
}

pub fn largest_rectangle_area2(heights: Vec<i32>) -> i32 {
    let mut max_area = 0;
    if heights.len() == 1 {
        return heights[0];
    }
    let mut stack = vec![];
    for i in 0..=heights.len() {
        let h = if i == heights.len() { 0 } else {
            heights[i]
        };
        while !stack.is_empty() && heights[stack[stack.len() - 1]] >= h {
            let left = stack.pop().unwrap();

            if stack.is_empty() {
                max_area = std::cmp::max(max_area, i as i32 * heights[left]);
            } else {
                max_area = std::cmp::max(max_area, (i - 1 - stack.last().unwrap()) as i32 * heights[left]);
            }
        }
        stack.push(i);
    }
    max_area
}

pub fn largest_rectangle_area3(heights: Vec<i32>) -> i32 {
    let mut stack = vec![];
    let mut res = 0;
    for i in 0..=heights.len() {
        let cur_height = if i == heights.len() { 0 } else { heights[i] };

        while !stack.is_empty() && cur_height < heights[*stack.last().unwrap()] {
            let h = stack.pop().unwrap();
            if stack.is_empty() {
                res = res.max(i as i32 * heights[h])
            } else { res = res.max((i - *stack.last().unwrap() - 1) as i32 * heights[h]); }
        }
        stack.push(i)
    }
    res
}