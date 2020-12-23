use std::fs::File;
#[test]
fn match_error() {
    let f = File::open("hello.txt");
    match f {
        Ok(file) => {
            println!("File opened successfully.");
        }
        Err(err) => {
            println!("Failed to open the file.");
        }
    };
}

fn f(i: i32) -> Result<i32, bool> {
    if i >= 0 {
        Ok(i)
    } else {
        Err(false)
    }
}

fn g(i: i32) -> Result<i32, bool> {
    let t = f(i)?;
    println!("go ok");
    Ok(t)
}

#[test]
fn zero() {
    let r = g(-1);
    if let Ok(v) = r {
        println!("Ok: g(10000) = {}", v);
    } else {
        println!("Err");
    }
}
