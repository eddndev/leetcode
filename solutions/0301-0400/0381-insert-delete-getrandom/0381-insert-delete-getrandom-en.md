---
title: "0381 Insert Delete GetRandom O(1) Duplicates Allowed - EN"
problemUrl: "https://leetcode.com/problems/insert-delete-getrandom-o1-duplicates-allowed/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["hash-table", "design", "array", "randomized"]
complexity:
  time: "O(1) amortized per insert, remove, and getRandom"
  space: "O(N), where N is the total number of elements in the collection"
---

# Juggling Duplicates in Constant Time

## The Problem
Design a `RandomizedCollection` data structure that supports three operations in average O(1) time: `insert(val)` which inserts an element `val` and returns `true` if the element was not already present, `remove(val)` which removes one instance of `val` and returns `true` if the element was present, and `getRandom()` which returns a random element from the collection where the probability of each element is proportional to the number of times it appears.

## The Trap of Duplicates

If this problem only required unique values, the classic solution would be a vector plus a HashMap mapping each value to its index. But duplicates change everything. A single value can appear multiple times, and I need to track all of its indices so I can remove any one of them in O(1). Moreover, `getRandom` must respect frequency: if the number 5 appears three times and the number 2 appears once, then 5 must be three times more likely to be selected.

## The Strategy of a Vector with Index Sets

My approach combines two structures:

1. **A vector `nums`** that stores all elements, including duplicates. This makes `getRandom` trivial: I just pick a uniform random index.
2. **A `HashMap<i32, HashSet<usize>>`** that for each value stores the set of all indices where it appears in the vector.

Insertion is straightforward: I append the value to the end of the vector and record its new index in the corresponding set. The trick lies in removal.

## Removal Without Gaps

To remove in O(1), I cannot simply pull an element from the middle of the vector, because that would shift all subsequent indices. Instead, I use the swap-with-last technique:

1. I pick any index of the value to remove (I take the first one the `HashSet` iterator yields).
2. I remove it from that value's index set.
3. If that index is not the last in the vector, I copy the last element of the vector into the position I want to remove, and update the last element's index set to reflect its new position.
4. I pop the vector to discard the last position.

This dance of swaps guarantees that the vector is always compact, with no gaps, and that every operation is amortized O(1).

## Walking Through an Example

Consider the operations: `insert(1)`, `insert(1)`, `insert(2)`, `remove(1)`, `getRandom()`.

- **insert(1)**: `nums = [1]`, indices: `{1: {0}}`. Returns `true` (first time).
- **insert(1)**: `nums = [1, 1]`, indices: `{1: {0, 1}}`. Returns `false` (already existed).
- **insert(2)**: `nums = [1, 1, 2]`, indices: `{1: {0, 1}, 2: {2}}`. Returns `true`.
- **remove(1)**: I pick index 0 (or 1, depends on the iterator). Suppose I pick 0. The last element is `2` at index 2. I copy `2` into position 0: `nums = [2, 1]`. Update indices: `{1: {1}, 2: {0}}`. Returns `true`.
- **getRandom()**: With `nums = [2, 1]`, there is a 50% probability for each.

## Rust Solution

```rust
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
```

The `RandomizedCollection` structure maintains a vector `nums` where duplicates coexist naturally, and a `HashMap` of sets that tracks where each value lives. In `insert`, I first check whether the value already exists to decide the boolean return, then append it to the end of the vector and record its index. In `remove`, I take any index of the value to delete, remove it from the set, and if it was not the last element in the vector, I move the last element into that position while updating its indices. Finally, `get_random` simply generates a uniform random index over the vector, which automatically respects duplicate frequencies since each instance occupies its own position.

## Conclusion

Insert Delete GetRandom O(1) with duplicates is a problem that looks straightforward until duplicates shatter the assumptions of the classic approach. The key insight is that a `HashSet` of indices per value, combined with the swap-with-last technique, preserves all constant-time guarantees. The vector gives us `getRandom` for free with correct probabilities, and the map of sets gives us the ability to locate and remove any instance without scanning the entire structure. It is an elegant exercise in how multiple data structures can collaborate to achieve what none could accomplish alone.
