pub fn quick_sort(arr: &mut Vec<i32>, l: usize, r: usize) {
    if l < r {
        let pivot = partition(arr, l, r);
        quick_sort(arr, l, pivot - 1);
        quick_sort(arr, pivot + 1, r);
    }
}

fn partition(arr: &mut Vec<i32>, l: usize, r: usize) -> usize {
    let mut j = l+1;
    let p = arr[l];

    for i in l..=r {
        if arr[i] < p {
            arr.swap(i,j);
            j+=1;
        }
    }
    arr.swap(l, j-1);
    j-1
}

#[test]
fn test() {
    let mut v = vec![2,1,4,5,6,70,10];
    let len = v.len();
    quick_sort(&mut v, 0 , len-1);
    println!("{:?}", v)

}

