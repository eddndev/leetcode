---
title: "0065 Valid Number - EN"
problemUrl: "https://leetcode.com/problems/valid-number/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["string", "simulation"]
complexity:
  time: "O(N)"
  space: "O(1)"
---

# The Grammar of Numbers

## The Problem
Given a string `s`, determine if it represents a valid number. A valid number can be an integer or a decimal, optionally followed by an exponent part (`e` or `E`). Integers and decimals may be preceded by a sign (`+` or `-`). Decimals must contain at least one digit and exactly one dot. The exponent part consists of `e` or `E` followed by an integer (which itself may be signed).

## The Deceptive Simplicity

At first glance, this problem seems trivial: just parse the string and see if it looks like a number. But then you start listing the edge cases, and the list never ends. Is `".1"` valid? Yes. What about `"."` alone? No. Can an exponent have a decimal? No. Can a sign appear in the middle of a number? Only right after `e` or `E`. Every rule has its nuance, and a careless implementation will fail on some obscure corner case.

I considered using a regex or a finite state machine with explicitly defined states and transitions. Both are valid, but I wanted something more direct: a single pass through the string with a handful of boolean flags that track what we've seen so far. The logic becomes a careful `match` on each character, where the flags tell us whether what we're seeing is still legal.

## The Single-Pass Strategy

The idea is to walk through the string character by character, maintaining three flags:

- **`seen_digit`**: Have we seen at least one digit? This is crucial because strings like `"."`, `"e5"`, or `"+"` are invalid -- a number must contain digits.
- **`seen_exponent`**: Have we already encountered an `e` or `E`? We can't have two exponents.
- **`seen_dot`**: Have we already encountered a dot? A number can have at most one, and it can't appear in the exponent part.

For each character, we apply a specific rule:

1. **Digit (`0-9`):** Always valid. We set `seen_digit = true`.

2. **Sign (`+` or `-`):** Only valid at the very start of the string or immediately after `e`/`E`. If it appears anywhere else, the string is invalid.

3. **Exponent (`e` or `E`):** Invalid if we've already seen one, or if no digit has appeared before it (because `"e5"` is not a number). When we accept it, we set `seen_exponent = true` and **reset `seen_digit` to `false`**. This last detail is essential: the exponent part must contain its own digits, so `"1e"` is invalid.

4. **Dot (`.`):** Invalid if we've already seen a dot or if we're in the exponent part (because exponents must be integers). Otherwise, we accept it and set `seen_dot = true`.

5. **Anything else:** Immediately invalid.

At the end, we return `seen_digit`. This final check catches cases like `"1e"` (exponent with no digits after it) or `"."` (dot with no digits at all). The reset of `seen_digit` when we encounter an exponent is what makes this work: it forces the exponent part to prove it has digits of its own.

### Walking Through an Example

For `s = "-3.14e2"`:
- `'-'`: index 0, sign at start is allowed.
- `'3'`: digit, `seen_digit = true`.
- `'.'`: no prior dot, no exponent, `seen_dot = true`.
- `'1'`: digit, `seen_digit = true`.
- `'4'`: digit, `seen_digit = true`.
- `'e'`: `seen_digit` is true, no prior exponent. Set `seen_exponent = true`, reset `seen_digit = false`.
- `'2'`: digit, `seen_digit = true`.
- End: `seen_digit` is `true`. **Valid.**

For `s = "1e"`:
- `'1'`: digit, `seen_digit = true`.
- `'e'`: valid (digit seen, no prior exponent). `seen_exponent = true`, `seen_digit = false`.
- End: `seen_digit` is `false`. **Invalid.** The exponent has no digits.

## Rust Solution

```rust
impl Solution {
    pub fn is_number(s: String) -> bool {
        let bytes = s.as_bytes();
        let n = bytes.len();

        let mut seen_digit = false;
        let mut seen_exponent = false;
        let mut seen_dot = false;

        for (i, &b) in bytes.iter().enumerate() {
            match b {
                b'0'..=b'9' => {
                    seen_digit = true;
                }

                b'+' | b'-' => {
                    if i > 0 && bytes[i - 1] != b'e' && bytes[i - 1] != b'E' {
                        return false;
                    }
                }

                b'e' | b'E' => {
                    if seen_exponent || !seen_digit {
                        return false;
                    }
                    seen_exponent = true;

                    seen_digit = false;
                }

                b'.' => {
                    if seen_dot || seen_exponent {
                        return false;
                    }
                    seen_dot = true;
                }

                _ => return false,
            }
        }

        seen_digit
    }
}
```

The Rust implementation works directly on bytes with `as_bytes()`, which is both efficient and natural since all valid characters in a number are ASCII. The `match` expression maps cleanly onto the rules we described: each arm handles one category of character, and the flags enforce the constraints. There's a subtle elegance in how the sign validation works -- instead of tracking a separate `seen_sign` flag, we simply check the previous character. If the sign isn't at position 0 and the character before it isn't `e` or `E`, it's illegal. This avoids extra state while remaining perfectly correct.

## Conclusion

This problem rewards precision over cleverness. There's no algorithmic trick here, no dynamic programming table, no graph traversal. It's about understanding a grammar and encoding it faithfully into a series of simple checks. The three boolean flags act as a minimal state machine, and the single pass through the string ensures we never do more work than necessary. What makes the solution elegant is not its complexity but its restraint: just enough state to capture every rule, and not a bit more.
