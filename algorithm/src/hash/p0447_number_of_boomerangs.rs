use std::collections::HashMap;
use crate::Solution;

impl Solution {
    pub fn number_of_boomerangs(points: Vec<Vec<i32>>) -> i32 {
        let mut ret = 0;
        for i in 0..points.len() {
            let mut map = HashMap::new();
            for j in 0..points.len() {
                if i == j {
                    continue;
                }
                let distance = (points[j][0] - points[i][0]).pow(2) + (points[j][1] - points[i][1]).pow(2);
                let v = map.entry(distance).or_insert(0);
                *v+=1;
            }
            //println!("{:?}", map);
            for (_,v) in map {
                if v > 1 {
                    ret+= v * (v-1)
                }
            }
        }
        ret
    }
}