---
title: "0005 Longest Palindromic Substring - EN"
problemUrl: "https://leetcode.com/problems/longest-palindromic-substring/"
difficulty: "Medium"
pubDate: "2026-03-08"
tags: ["string", "two-pointers", "expand-around-center"]
complexity:
  time: "O(n^2)"
  space: "O(n)"
---

# Longest Palindromic Substring: Expanding From the Center

## The Problem
Given a string `s`, find the longest palindromic substring contained within it.

A palindrome is a string that reads the same forwards and backwards. For example, in `"babad"` the answer could be `"bab"` or `"aba"`, and in `"cbbd"` the answer is `"bb"`.

The brute force approach would be to check every possible substring and verify which ones are palindromes, but that would give us $O(n^3)$. We need something better.

## The Intuition: Every Palindrome Has a Center
When I saw this problem, the key observation was that **every palindrome expands symmetrically from its center**. Instead of generating all substrings and checking each one, we can do the reverse: for each possible center, expand outward while the characters match.

There is a subtle detail: palindromes can have odd or even length. An odd palindrome like `"aba"` has a single character as its center (the `b`). An even palindrome like `"abba"` has its center between the two `b` characters. This means that for each position `i` we must perform **two expansions**: one centered on a single character (`i, i`) and another centered between two characters (`i, i+1`).

## The Algorithm
1. Convert the string into a vector of characters for index-based access.
2. For each position `i` from 0 to `n-1`:
   - Expand from `(i, i)` for odd-length palindromes.
   - Expand from `(i, i+1)` for even-length palindromes.
3. In each expansion, move pointers `l` and `r` outward while `chars[l] == chars[r]`.
4. Keep track of the start position and maximum length found.
5. At the end, extract the corresponding substring.

The `expand` function is the heart of the solution. It takes the character array and two initial indices, and returns the start position and length of the palindrome found. The special case is when `l` reaches 0: we need to return immediately because we cannot decrement further (we are working with `usize`, and subtracting from 0 would cause an underflow).

### Implementation in Rust
In Rust, we work with `Vec<char>` because Rust strings are UTF-8 encoded and do not allow direct indexing by position. The `expand` function carefully handles the array boundaries, especially when the left pointer reaches position 0.

```rust
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        if s.len() < 2 {
            return s;
        }

        let chars: Vec<char> = s.chars().collect();
        let mut start = 0;
        let mut max_len = 0;

        for i in 0..chars.len() {
            let (s1, l1) = Self::expand(&chars, i, i);

            let (s2, l2) = Self::expand(&chars, i, i + 1);

            if l1 > max_len {
                max_len = l1;
                start = s1;
            }

            if l2 > max_len {
                max_len = l2;
                start = s2;
            }
        }

        chars[start..start + max_len].iter().collect()
    }

    fn expand(chars: &[char], mut l: usize, mut r: usize) -> (usize, usize) {
        let n = chars.len();

        while r < n && chars[l] == chars[r] {
            if l == 0 && chars[l] == chars[r] {
                if l == 0 {
                    return (0, r + 1);
                }
            }
            l -= 1;
            r += 1;

            if r >= n || chars[l] != chars[r] {
                return (l + 1, r - (l + 1));
            }
        }

        (l + 1, r - (l + 1))
    }
}
```

## Conclusion
The expand-around-center technique turns a problem that seems to require $O(n^3)$ into one solvable in $O(n^2)$. For each of the $n$ possible centers, the expansion can traverse at most $O(n)$ characters, giving us $O(n^2)$ in time and $O(n)$ in space (due to the `Vec<char>`).

The lesson from this problem is that sometimes **inverting the perspective** simplifies everything. Instead of checking whether each substring is a palindrome, we build palindromes from the inside out. It is an elegant idea that shows up in many other string problems.
