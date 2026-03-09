---
title: "0044 Wildcard Matching - EN"
problemUrl: "https://leetcode.com/problems/wildcard-matching/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["string", "greedy", "two-pointers"]
complexity:
  time: "O(M * N)"
  space: "O(1)"
---

# Taming the Wildcard

## The Problem
Given a string `s` and a pattern `p`, implement wildcard pattern matching with support for `'?'` (matches any single character) and `'*'` (matches any sequence of characters, including the empty sequence). The matching must cover the **entire** input string, not just a part of it.

## The First Instinct

When I first approached this problem, I instinctively reached for dynamic programming. After all, it's a matching problem with two strings and branching decisions, just like Regular Expression Matching. A DP table of size M x N would solve it cleanly. But then I asked myself: does the `'*'` in wildcard matching really require that much machinery?

In regular expressions, `'*'` is tied to the preceding character: `a*` means "zero or more `a`s." That coupling creates genuinely overlapping subproblems. But in wildcard matching, `'*'` is independent: it can match any sequence of characters on its own. That independence is what opens the door to a **greedy** approach that uses only constant space.

## The Greedy Strategy

The idea is to walk through both strings simultaneously with two pointers, `s_idx` and `p_idx`, and handle each situation as it arises:

1. **Direct match or `?`:** If the current pattern character matches the current string character, or the pattern has `'?'`, we advance both pointers. This is the straightforward case.

2. **Star encountered:** When we hit a `'*'` in the pattern, we don't immediately decide how many characters it will consume. Instead, we record the position of this star (`star_idx`) and the current position in the string (`s_tmp_idx`). Then we advance only the pattern pointer, effectively trying to match the star with zero characters first.

3. **Mismatch with a star to fall back on:** If neither of the above cases applies but we have a previously recorded star, we **backtrack**. We return the pattern pointer to just after the star, increment `s_tmp_idx` by one (letting the star consume one more character), and set `s_idx` to `s_tmp_idx`. This is where the greedy retry happens.

4. **Mismatch with no star:** If there's no star to fall back on, the match fails entirely.

After the string is fully consumed, there might be trailing `'*'` characters in the pattern. These can all match the empty sequence, so we skip over them. If the pattern pointer has reached the end, the match succeeds.

### Why This Works

The key insight is that a `'*'` only needs to "remember" its most recent occurrence. If we encounter a second `'*'`, it subsumes the first one: any characters the first star needed to match are now covered by the second star's expanded reach. This means we never need to track more than one star at a time, and a single backtrack pointer suffices.

### A Step-by-step Example

For `s = "adceb"`, `p = "*a*b"`:
- `p_idx=0` is `'*'`: record `star_idx=0`, `s_tmp_idx=0`. `p_idx=1`
- `s_idx=0` is `'a'`, `p_idx=1` is `'a'`: match. `s_idx=1`, `p_idx=2`
- `p_idx=2` is `'*'`: record `star_idx=2`, `s_tmp_idx=1`. `p_idx=3`
- `s_idx=1` is `'d'`, `p_idx=3` is `'b'`: mismatch. Backtrack: `s_tmp_idx=2`, `s_idx=2`, `p_idx=3`
- `s_idx=2` is `'c'`, `p_idx=3` is `'b'`: mismatch. Backtrack: `s_tmp_idx=3`, `s_idx=3`, `p_idx=3`
- `s_idx=3` is `'e'`, `p_idx=3` is `'b'`: mismatch. Backtrack: `s_tmp_idx=4`, `s_idx=4`, `p_idx=3`
- `s_idx=4` is `'b'`, `p_idx=3` is `'b'`: match. `s_idx=5`, `p_idx=4`
- `s_idx=5`: string exhausted. Pattern also at end. Result: **true**

The second `'*'` tried matching zero characters, then one, then two, then three, until the remaining pattern `"b"` lined up with the end of the string. That incremental expansion is the heart of the greedy approach.

## Rust Solution

```rust
impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let s_bytes = s.as_bytes();
        let p_bytes = p.as_bytes();

        let (mut s_idx, mut p_idx) = (0, 0);
        let (mut star_idx, mut s_tmp_idx) = (None, 0);

        while s_idx < s_bytes.len() {
            if p_idx < p_bytes.len() && (p_bytes[p_idx] == b'?' || p_bytes[p_idx] == s_bytes[s_idx])
            {
                s_idx += 1;
                p_idx += 1;
            } else if p_idx < p_bytes.len() && p_bytes[p_idx] == b'*' {
                star_idx = Some(p_idx);
                s_tmp_idx = s_idx;
                p_idx += 1;
            } else if let Some(star_p) = star_idx {
                p_idx = star_p + 1;
                s_tmp_idx += 1;
                s_idx = s_tmp_idx;
            } else {
                return false;
            }
        }

        while p_idx < p_bytes.len() && p_bytes[p_idx] == b'*' {
            p_idx += 1;
        }

        p_idx == p_bytes.len()
    }
}
```

The Rust implementation is lean and expressive. Converting the strings to `&[u8]` with `as_bytes()` lets us work with byte comparisons directly, avoiding any Unicode overhead in a problem that only deals with lowercase letters and two special characters. The use of `Option<usize>` for `star_idx` is idiomatic: `None` means no star has been seen yet, and the `if let Some(star_p)` pattern in the backtracking branch reads naturally as "if there's a star to fall back on." The entire algorithm runs with just four scalar variables and no heap allocations beyond the input strings themselves.

## Conclusion

This problem is a beautiful example of how understanding the structure of a problem can lead to a dramatically simpler solution. The wildcard `'*'` is fundamentally different from the regex `'*'`: it's self-contained, not coupled to a preceding character. That independence means we don't need a DP table to explore all possibilities. A single pass with a greedy backtracking strategy, remembering only the most recent star, is enough to cover every case. The result is O(1) space and code that fits in a single loop, a reminder that sometimes the best optimization is not a clever data structure, but a deeper look at the problem itself.
