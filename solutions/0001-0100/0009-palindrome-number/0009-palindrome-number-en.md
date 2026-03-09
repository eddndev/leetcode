---
title: "0009 Palindrome Number - EN"
problemUrl: "https://leetcode.com/problems/palindrome-number/"
difficulty: "Easy"
pubDate: "2026-03-08"
tags: ["math"]
complexity:
  time: "O(log n)"
  space: "O(1)"
---

# Palindrome Number

## Problem
Given an integer `x`, return `true` if `x` is a palindrome, meaning it reads the same forwards and backwards.

Negative numbers are never palindromes (the `-` sign breaks symmetry). Neither are numbers ending in `0`, except `0` itself.

## Solution
The immediate temptation is to convert the number to a string and compare characters. It works, but uses unnecessary extra memory.

The trick is to **reverse only half of the number**. We extract digits from the end and accumulate them until the reversed half is greater than or equal to the remaining half. At that point, we compare both halves: if they are equal, it is a palindrome. If the number has an odd number of digits, we divide the reversed half by 10 to discard the middle digit.

The condition `input > reversed` as the `while` guard is the key: it guarantees we only process half the digits, which also gives us `O(log n)` complexity.

### Implementation in Rust

```rust
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 || (x % 10 == 0 && x != 0) {
            return false;
        }

        let mut input = x;
        let mut reversed = 0;

        while input > reversed {
            reversed = reversed * 10 + input % 10;
            input /= 10;
        }

        input == reversed || input == reversed / 10
    }
}
```
