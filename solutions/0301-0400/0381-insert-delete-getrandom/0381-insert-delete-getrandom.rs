use rand::Rng;
use std::collections::{HashMap, HashSet};

struct RandomizedCollection {
    nums: Vec<i32>,
    indices: HashMap<i32, HashSet<usize>>,
}

impl RandomizedCollection {
    fn new() -> Self {
        RandomizedCollection {
            nums: Vec::new(),
            indices: HashMap::new(),
        }
    }

    fn insert(&mut self, val: i32) -> bool {
        let is_present = self.indices.contains_key(&val) && !self.indices[&val].is_empty();

        self.nums.push(val);

        let new_idx = self.nums.len() - 1;
        self.indices
            .entry(val)
            .or_insert_with(HashSet::new)
            .insert(new_idx);

        !is_present
    }

    fn remove(&mut self, val: i32) -> bool {
        if let Some(idxs) = self.indices.get_mut(&val) {
            if idxs.is_empty() {
                return false;
            }

            let remove_idx = *idxs.iter().next().unwrap();

            idxs.remove(&remove_idx);

            if idxs.is_empty() {
                self.indices.remove(&val);
            }

            let last_idx = self.nums.len() - 1;

            if remove_idx != last_idx {
                let last_val = self.nums[last_idx];

                self.nums[remove_idx] = last_val;

                if let Some(last_val_idxs) = self.indices.get_mut(&last_val) {
                    last_val_idxs.remove(&last_idx);
                    last_val_idxs.insert(remove_idx);
                }
            }

            self.nums.pop();

            true
        } else {
            false
        }
    }

    fn get_random(&self) -> i32 {
        let mut rng = rand::thread_rng();
        let random_idx = rng.gen_range(0..self.nums.len());
        self.nums[random_idx]
    }
}
