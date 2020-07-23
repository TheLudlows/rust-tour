const VAR:u32 = 10;
fn main() {
   // const VAR:u32 = 20;

    let mut x = 5;
    println!("The value of x is: {}", x);
    let mut x = "a";
    println!("The value of x is: {}", x);
    println!("The const is:{}", VAR);

    let y = 100;
    println!("The value of y is: {}", y);

    let y = "yyy";
    println!("The value of y is: {}", y);

    let t:(u32,char,bool) = (1,'a',false);
    println!("tuple is {},{},{}", t.0,t.1,t.2);

    let arr = [1,2,3,4,5];
    let str_arr = ["a","b"];
    println!("arr len is:{}", arr.len());
    println!("arr string  first is:{}", str_arr[0])
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
fn first_words(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}
#[test]
fn test() {
    let mut a = "a".to_string();
    //let b = &mut a;
    //let c = *b;
    //let c = &a;
    //let d = *c;
}



