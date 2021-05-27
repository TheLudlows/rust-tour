use std::collections::HashSet;

fn main () {
    let mut vec =vec![vec![]];
    loop {
        let mut set = HashSet::new();

        let mut line = String::new();
        let read = std::io::stdin().read_line(&mut line).unwrap();

        if read == 0 {
            break;
        }
        let n = line.trim().parse::<usize>().unwrap();
        for _ in 0..n {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();

            let s = line.trim();
            let i = s.parse::<i32>().unwrap();
            set.insert(i);
        }
        let mut v = set.into_iter().collect::<Vec<i32>>();
        v.sort();
        vec.push(v);
    }
    //println!("{:?}",vec);
    for i in 0..vec.len() {
        for j in 0..vec[i].len() {
            println!("{}", vec[i][j]);
        }
    }
}