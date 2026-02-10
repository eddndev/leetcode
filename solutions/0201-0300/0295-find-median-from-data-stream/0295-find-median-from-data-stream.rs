use std::collections::BinaryHeap;
use std::cmp::Reverse;

struct MedianFinder {
    small: BinaryHeap<i32>,
    large: BinaryHeap<Reverse<i32>>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {

    fn new() -> Self {
        MedianFinder {
            small: BinaryHeap::new(),
            large: BinaryHeap::new(),
        }
    }
    
    fn add_num(&mut self, num: i32) {
        self.small.push(num);

        if let Some(max_of_small) = self.small.pop() {
            self.large.push(Reverse(max_of_small));
        }


        if self.large.len() > self.small.len() {
            if let Some(Reverse(min_of_large)) = self.large.pop() {
                self.small.push(min_of_large);
            }
        }
    }
    
    fn find_median(&self) -> f64 {
        if self.small.len() > self.large.len() {
            return *self.small.peek().unwrap() as f64;
        }

        let s = *self.small.peek().unwrap();
        let l = self.large.peek().unwrap().0;

        (s as f64 + l as f64) / 2.0
    }
}

/**
 * Your MedianFinder object will be instantiated and called as such:
 * let obj = MedianFinder::new();
 * obj.add_num(num);
 * let ret_2: f64 = obj.find_median();
 */