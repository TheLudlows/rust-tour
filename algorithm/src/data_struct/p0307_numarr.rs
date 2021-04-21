// 线段树
struct NumArray {
    nums: Vec<i32>,
    tree: Vec<i32>,
}
#[allow(dead_code)]

impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let mut tree = vec![0; nums.len() * 4];
        Self::build_tree(0, nums.len() - 1, &nums, 0, &mut tree);
        Self {
            nums,
            tree,
        }
    }
    fn update(&mut self, index: i32, val: i32) {
        self.nums[index as usize] = val;
        Self::update_tree(index as usize, 0, self.nums.len() - 1, 0, &mut self.tree, val);
    }
    fn update_tree(idx: usize, left: usize, right: usize, tree_idx: usize, tree: &mut Vec<i32>, val: i32) {
        if left == right {
            tree[tree_idx] = val;
            return;
        }
        let mid = (left + right) / 2;
        if idx <= mid {
            Self::update_tree(idx, left, mid, left_child(tree_idx), tree, val);
        } else {
            Self::update_tree(idx, mid + 1, right, right_child(tree_idx), tree, val);
        }
        tree[tree_idx] = tree[left_child(tree_idx)] + tree[right_child(tree_idx)];
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        Self::sum(0, 0, self.nums.len() - 1, left as usize, right as usize, &self.tree)
    }

    fn sum(tree_idx: usize, tree_left: usize, tree_right: usize, left: usize, right: usize, tree: &Vec<i32>) -> i32 {
        if tree_left == left && tree_right == right {
            return tree[tree_idx];
        }
        let mid = (tree_left + tree_right) / 2;
        if left > mid {
            Self::sum(right_child(tree_idx), mid + 1, tree_right, left, right, tree)
        } else if right <= mid {
            Self::sum(left_child(tree_idx), tree_left, mid, left, right, tree)
        } else {
            Self::sum(right_child(tree_idx), mid + 1, tree_right, mid + 1, right, tree) +
                Self::sum(left_child(tree_idx), tree_left, mid, left, mid, tree)
        }
    }

    fn build_tree(left: usize, right: usize, arr: &Vec<i32>, tree_idx: usize, tree: &mut Vec<i32>) {
        if left == right {
            tree[tree_idx] = arr[left];
        } else {
            let mid = (left + right) / 2;
            Self::build_tree(left, mid, arr, left_child(tree_idx), tree);
            Self::build_tree(mid + 1, right, arr, right_child(tree_idx), tree);
            tree[tree_idx] = tree[left_child(tree_idx)] + tree[right_child(tree_idx)];
        }
    }
}

#[inline]
fn left_child(parent: usize) -> usize {
    parent * 2 + 1
}

#[inline]
fn right_child(parent: usize) -> usize {
    parent * 2 + 2
}

#[test]
fn test() {
    let arr = NumArray::new(vec![0, 1, 2, 3]);
    let r = arr.sum_range(1, 3);
    println!("{}", r);
}
