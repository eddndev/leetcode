---
title: "0003 Longest Substring Without Repeating Characters - EN"
problemUrl: "https://leetcode.com/problems/longest-substring-without-repeating-characters/"
difficulty: "Medium"
pubDate: "2026-03-08"
tags: ["hash-table", "string", "sliding-window"]
complexity:
  time: "O(n)"
  space: "O(1)"
---

# Longest Substring Without Repeating Characters: The Sliding Window and the Static Array Trick

## The Problem
Given a string `s`, find the length of the longest substring without repeating characters.

It sounds simple at first, but the key lies in how we efficiently track which characters are already inside our current "window" and how we adjust it without backtracking unnecessarily.

## The First Intuition: HashSet and Brute Force
My first instinct was the naive approach: for each position in the string, expand rightward while there are no repetitions, using a `HashSet` to detect duplicates. This works, but has $O(n^2)$ complexity in the worst case. Every time we find a duplicate, we move the start one step forward and scan again. Unacceptable.

## The Evolution: Sliding Window with HashMap
The natural next step was the **sliding window** technique. We maintain two pointers, `left` and `right`, that define the edges of our window. When `right` advances and finds a repeated character, instead of resetting from scratch, we move `left` just past the last occurrence of that character.

Using a `HashMap` we could store the last position of each character. But a HashMap has overhead: hashing, collision handling, internal dynamic allocation. For a problem where the character universe is limited (128 ASCII characters), there is a more elegant solution.

## The Optimization: Static Array of 128 Positions
Instead of a HashMap, I used a static array `[usize; 128]`. Each index in the array corresponds to the ASCII value of a character, and stores the **position + 1** of the last time we saw that character.

Why position + 1? Because we initialize everything to 0, and we need to distinguish "never seen" (value 0) from "seen at position 0". By storing `right + 1`, the comparison `last_seen[idx] > left` directly tells us whether the character is inside the current window.

This trick eliminates the need for a HashMap and gives us guaranteed $O(1)$ access with no collisions and no hashing overhead. Furthermore, the 128-element array lives on the stack, not the heap, which is excellent for cache locality.

### Implementation in Rust
In Rust, the `.as_bytes()` method on a `String` is $O(1)$ because internally `String` already stores data as a UTF-8 byte buffer. Combined with `.enumerate()`, we traverse the string idiomatically and efficiently.

```rust
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut last_seen = [0usize; 128];
        let mut max_len = 0;
        let mut left = 0;

        // .as_bytes() is O(1) in Rust 'cause String already saves the buffer internaly
        // .enumerate() gives us the index (right) and the value (byte)
        for (right, &byte) in s.as_bytes().iter().enumerate() {
            let idx = byte as usize;

            // If the character has been seen in the current range of the window
            if last_seen[idx] > left {
                left = last_seen[idx];
            }

            // We update the position of the character (idx + 1)
            last_seen[idx] = right + 1;

            let current_len = (right - left + 1) as i32;
            if current_len > max_len {
                max_len = current_len;
            }
        }

        max_len
    }
}
```

## Conclusion

I went from the naive $O(n^2)$ approach to an $O(n)$ solution with constant $O(1)$ space (the 128-element array is fixed, regardless of input size).

The key lesson from this problem is that when the key universe is small and known, **a static array always beats a HashMap**. No hashing, no collisions, no dynamic allocations. Just index arithmetic and contiguous memory on the stack.
