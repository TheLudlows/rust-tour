use crate::Solution;

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let mut ret = vec![];
        let n = n as usize;
        let mut row = vec![false; n];
        let mut dia1 = vec![false; 2*n-1];
        let mut dia2 = vec![false; 2*n-1];
        let mut cur = vec![];
        find_queen(n,0,&mut cur, &mut row, &mut dia1, &mut dia2, &mut ret);
        ret
    }
}
fn find_queen(n: usize, line: usize, cur: &mut Vec<usize>, row:&mut Vec<bool>
              , dia1:&mut Vec<bool>, dia2:&mut Vec<bool>, ret: &mut Vec<Vec<String>>) {
        if line == n {
            let mut vec = vec![];
            for i in 0..n {
                let mut l =  vec!['.'; n];
                l[cur[i]] = 'Q';
                vec.push(String::from(l.into_iter().collect::<String>()));
            }
            ret.push(vec);
            return
        }

        for i in 0..n {
            if !row[i] && !dia1[line + i] && !dia2[i + n-line -1] {
                cur.push(i);
                row[i] = true;
                dia1[line + i] = true;
                dia2[i + n -line  -1] = true;
                find_queen(n,line+1,cur,row,dia1, dia2, ret);
                cur.pop();
                row[i] = false;
                dia1[line + i] = false;
                dia2[i + n-line  -1] = false;
            }
        }
}
#[test]
fn test() {
    Solution::solve_n_queens(4);
}