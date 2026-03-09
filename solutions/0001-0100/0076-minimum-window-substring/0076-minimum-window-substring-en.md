---
title: "0076 Minimum Window Substring - EN"
problemUrl: "https://leetcode.com/problems/minimum-window-substring/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["string", "sliding-window", "hash-table"]
complexity:
  time: "O(N + M)"
  space: "O(1)"
---

# The Smallest Window That Holds Everything

## The Problem
Given two strings `s` and `t`, find the minimum window substring of `s` such that every character in `t` (including duplicates) is included in the window. If no such window exists, return an empty string.

## The Initial Intuition

When I first approached this problem, the brute force path was obvious: check every possible substring of `s` and verify whether it contains all characters of `t`. That's O(N^2 * M) at best, and clearly not going to fly for large inputs.

The mental leap comes from recognizing that this is a classic **sliding window** problem. Instead of generating all substrings, I can maintain a window defined by two pointers, `left` and `right`, and slide it across `s`. The right pointer expands the window to include more characters, and the left pointer contracts it to find the minimum valid window. The question becomes: how do I efficiently know when my window contains all the characters of `t`?

## The Frequency Map Approach

The answer is a frequency map. I use an array of size 128 (covering all ASCII characters) to store the count of each character needed from `t`. As the right pointer advances, I decrement the count for each character it encounters. When a character's count goes from positive to zero or below, it means we've satisfied that character's requirement. I track this with a single variable `count`, initialized to `t.len()`, which represents the total number of characters still "owed."

The key insight: when `count` reaches zero, the current window contains all characters of `t`. At that point, I try to shrink the window from the left, looking for a smaller valid window.

When I move the left pointer forward, I increment the count for the character being removed. If that count goes above zero, it means we've lost a character that `t` actually needs, so `count` goes back up and the window is no longer valid. We stop contracting and resume expanding with the right pointer.

### Why This Works

The subtlety is in how the frequency map handles characters not in `t`. Those characters start with a count of 0 in the map. When the right pointer encounters them, their count goes negative. When the left pointer releases them, their count goes back up toward zero but never above it. So `count` (which only increments when a map value goes above zero) is never affected by irrelevant characters. The map naturally separates "needed" characters from "noise."

### A Step-by-step Example

For `s = "ADOBECODEBANC"`, `t = "ABC"`:
- Initialize map: `A:1, B:1, C:1`, `count = 3`
- Right moves through `A`: map `A:0`, count = 2. Window: `"A"`
- Right continues through `D`, `O`, `B`: at `B`, map `B:0`, count = 1. Window: `"ADOB"`
- Right through `E`, `C`: at `C`, map `C:0`, count = 0. Window: `"ADOBEC"` (length 6)
- Now shrink from left. Release `A`: map `A:1`, count = 1. Window invalid. Record `start=0, len=6`
- Right continues... Eventually finds window `"BANC"` (length 4), which is the answer.

The beauty is that both pointers only move forward, so each character is visited at most twice: once by `right` and once by `left`.

## Rust Solution

```rust
impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let s_bytes = s.as_bytes();
        let t_bytes = t.as_bytes();

        let mut map = [0; 128];

        for &b in t_bytes {
            map[b as usize] += 1;
        }

        let mut left = 0;
        let mut min_len = usize::MAX;
        let mut start_index = 0;
        let mut count = t.len();
        for (right, &char_right) in s_bytes.iter().enumerate() {
            if map[char_right as usize] > 0 {
                count -= 1;
            }
            map[char_right as usize] -= 1;

            while count == 0 {
                let current_len = right - left + 1;

                if current_len < min_len {
                    min_len = current_len;
                    start_index = left;
                }

                let char_left = s_bytes[left];
                map[char_left as usize] += 1;

                if map[char_left as usize] > 0 {
                    count += 1;
                }

                left += 1;
            }
        }

        if min_len == usize::MAX {
            "".to_string()
        } else {
            String::from_utf8_lossy(&s_bytes[start_index..start_index + min_len]).to_string()
        }
    }
}
```

The Rust implementation leverages working directly with byte slices via `as_bytes()`, which avoids the overhead of character-level iteration on UTF-8 strings. The frequency map is a fixed-size array `[0; 128]` rather than a HashMap, giving us O(1) lookups with zero allocation overhead. The use of `usize::MAX` as a sentinel for `min_len` is idiomatic Rust for "no valid answer yet," and the final slice extraction `s_bytes[start_index..start_index + min_len]` cleanly reconstructs the answer without any unnecessary copies during the search itself.

## Conclusion

This problem is a textbook demonstration of the sliding window technique at its finest. The central idea is that we never need to backtrack: both pointers march forward, and the frequency map acts as a compact ledger that tells us exactly when we have enough and when we've lost too much. The space is O(1) because the map has a fixed size of 128 regardless of input, and the time is O(N + M) because we traverse `s` with both pointers and `t` once for initialization. Sometimes the most elegant solution comes not from complex data structures, but from a careful accounting of what we need and what we have.
