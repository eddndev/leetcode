---
title: "0066 Plus One - EN"
problemUrl: "https://leetcode.com/problems/plus-one/"
difficulty: "Easy"
pubDate: "2026-03-08"
tags: ["array", "math"]
complexity:
  time: "O(n)"
  space: "O(1)"
---

# Plus One

## Problem
Given a large integer represented as an array of digits `digits`, where each element is a single digit of the number, add one to the number and return the resulting array. The digits are stored left to right, from most significant to least significant.

## Solution
We iterate through the array from right to left. If the current digit is less than 9, we simply increment it by 1 and return: there is no carry to propagate. If the digit is 9, it becomes 0 and we continue with the next digit to the left.

If we exit the loop without returning, it means every digit was 9 (e.g., `999`). In that case, all digits have been set to 0 and we just need to insert a `1` at the beginning of the array.

### Implementation in Rust

```rust
impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        for digit in digits.iter_mut().rev() {
            if *digit < 9 {
                *digit += 1;
                return digits;
            }
            *digit = 0;
        }
        digits.insert(0, 1);
        digits
    }
}
```
