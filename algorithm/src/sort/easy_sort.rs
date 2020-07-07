
fn exchange_sort(arr: &mut [i32]) {
    for i in 0..arr.len() {
        for j in i + 1..arr.len() {
            if arr[j] > arr[i] {
                let tmp = arr[i];
                arr[i] = arr[j];
                arr[j] = tmp;
            }
            //println!("{:?}",arr)
        }
    }
}

fn bubble_sort(arr: &mut [i32]) {
    for i in 0..arr.len() {
        for j in 0..arr.len() - i - 1 {
            if arr[j] < arr[j+1] {
               arr.swap(j,j+1)
            }
        }
    }
}


#[cfg(test)]
mod test {
    use crate::sort::easy_sort::{exchange_sort, bubble_sort};

    #[test]
    fn test() {
        let mut a = [1, 4, 5, 6, 3];
        exchange_sort(&mut a);
        println!("{:?}", a);

        let mut a = [1, 4, 5, 6, 3];
        bubble_sort(&mut a);
        println!("{:?}", a)
    }
}