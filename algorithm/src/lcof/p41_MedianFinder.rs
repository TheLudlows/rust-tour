struct MedianFinder {
    data:Vec<i32>
}

impl MedianFinder {

    fn new() -> Self {
        Self{
            data:Vec::new()
        }
    }

    fn add_num(&mut self, num: i32) {
        self.data.push(num);
        self.data.sort();
    }

    fn find_median(&self) -> f64 {
        let n1 = self.data[self.data.len()/2] as f64;
        let n2 = self.data[(self.data.len() + 1)/2] as f64;
        (n1 + n2)/(2  as f64)
    }
}