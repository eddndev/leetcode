---
title: "0032 Longest Valid Parentheses - EN"
problemUrl: "https://leetcode.com/problems/longest-valid-parentheses/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["string", "stack", "dynamic-programming"]
complexity:
  time: "O(N)"
  space: "O(N)"
---

# The Art of Closing What You Open

## The Problem
Given a string containing only the characters `'('` and `')'`, find the length of the longest valid (well-formed) parentheses substring. A valid substring is one where every opening parenthesis has a corresponding closing parenthesis, in the correct order.

## The Deception of Simplicity

When I first saw this problem, I thought it would be enough to just count opening and closing parentheses. If they match, we have a valid substring. But I quickly realized that's not sufficient. Consider `"())()"`: it has the same count of `(` and `)`, but it's not a complete valid sequence. Order matters, and what we really need is to find the **longest contiguous segment** where parentheses are perfectly balanced.

My first attempt was dynamic programming, but then I thought of a more elegant solution: using a **stack**. The idea is that the stack doesn't just track open parentheses -- it also gives us a reference point for calculating lengths.

## The Stack with Sentinel Strategy

The key trick is initializing the stack with `-1`. This value acts as a **sentinel**: it represents the position just before the start of any valid substring. Without it, we'd have to handle a bunch of special cases.

As we walk through the string:

1. **If we find `(`:** We simply push its index onto the stack. It's an opening promise we hope to close later.

2. **If we find `)`:** First we pop the top of the stack. After popping:
   - **If the stack is empty:** There was no matching `(`. This `)` becomes the new boundary, so we push its index as the new sentinel.
   - **If the stack is not empty:** We have a match. The length of the current valid substring is `i - stack.top()`. We update the maximum if needed.

The brilliance of this approach is that the top of the stack always represents **the position just before the start of the current valid substring**. That makes computing the length a simple subtraction.

### A Step-by-step Example

For `"(()":
- Start with stack = `[-1]`
- `i=0`, `(`: stack = `[-1, 0]`
- `i=1`, `(`: stack = `[-1, 0, 1]`
- `i=2`, `)`: pop `1`, stack = `[-1, 0]`, length = `2 - 0 = 2`
- Result: `2`

For `")()())`:
- Start with stack = `[-1]`
- `i=0`, `)`: pop `-1`, stack empty, push `0`. Stack = `[0]`
- `i=1`, `(`: stack = `[0, 1]`
- `i=2`, `)`: pop `1`, stack = `[0]`, length = `2 - 0 = 2`
- `i=3`, `(`: stack = `[0, 3]`
- `i=4`, `)`: pop `3`, stack = `[0]`, length = `4 - 0 = 4`
- `i=5`, `)`: pop `0`, stack empty, push `5`. Stack = `[5]`
- Result: `4`

## Rust Solution

```rust
use std::cmp;

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let n = s.len();
        if n == 0 {
            return 0;
        }

        let mut stack: Vec<i32> = Vec::with_capacity(n + 1);

        stack.push(-1);

        let mut max_len = 0;

        for (i, &byte) in s.as_bytes().iter().enumerate() {
            if byte == b'(' {
                stack.push(i as i32);
            } else {
                stack.pop();

                if stack.is_empty() {
                    stack.push(i as i32);
                } else {
                    let current_len = (i as i32) - stack.last().unwrap();
                    max_len = cmp::max(max_len, current_len);
                }
            }
        }

        max_len
    }
}
```

In the Rust implementation, we convert the string to bytes with `as_bytes()` to compare directly against `b'('` and `b')'`, avoiding the overhead of working with Unicode characters. The stack is initialized with capacity `n + 1` to prevent unnecessary reallocations. We use `i32` instead of `usize` so we can store the sentinel `-1` without complications, and `stack.last().unwrap()` gives us the top without consuming it, which is exactly what we need after the `pop`.

## Conclusion

This problem is a perfect example of how a stack can solve problems that seem to require dynamic programming. The `-1` sentinel is the key to everything: it eliminates special cases and turns length calculations into a trivial operation. Instead of tracking intervals or building tables, we let the stack do all the bookkeeping. Sometimes the right data structure is all you need for an O(N) solution to fall into place naturally.
