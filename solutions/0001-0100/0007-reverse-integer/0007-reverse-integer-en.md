---
title: "0007 Reverse Integer - EN"
problemUrl: "https://leetcode.com/problems/reverse-integer/"
difficulty: "Medium"
pubDate: "2026-03-08"
tags: ["math", "overflow-handling"]
complexity:
  time: "O(log x)"
  space: "O(1)"
---

# Reverse Integer: Flipping Digits Without Blowing Up

## The Problem
Given a signed 32-bit integer, return the number with its digits reversed. If the result overflows beyond the range of a signed 32-bit integer (`[-2^31, 2^31 - 1]`), return `0`.

At first glance it looks like a trivial numeric manipulation exercise. Extract digits with modulo, rebuild the number by multiplying by 10... something anyone can do in a couple of minutes. But the trap lies in what you do not see: **overflow**.

## The Overflow Trap

The range of an `i32` goes from `-2,147,483,648` to `2,147,483,647`. Reversing a number like `1,999,999,999` produces `9,999,999,991`, which exceeds the maximum. If we do not handle this, the program crashes or returns garbage.

The key question is: **how do we detect overflow before it happens?**

### The Classic Approach (C/C++)
In C, the typical strategy is to manually compare against `INT_MAX / 10` before multiplying. It works but is verbose and prone to sign-related errors.

### The Idiomatic Approach (Rust)
Rust, by design, forces us to think about overflow. In debug mode, arithmetic operations that overflow trigger a `panic`. In release mode, they silently wrap around. Neither is what we want here.

The elegant solution: use `checked_mul` and `checked_add`, which return `Option<i32>`. If the operation is safe, we get `Some(value)`. If it overflows, we get `None`. Chaining them with `and_then` gives us a clean and safe pipeline.

What in C requires manual comparisons against limits becomes a single declarative expression inside a `match` in Rust.

## The Solution

The algorithm is straightforward:
1. Extract the last digit with `num % 10`.
2. Shrink the number with `num /= 10`.
3. Accumulate into `rev` by multiplying by 10 and adding the digit, but **guarding** every operation against overflow.
4. If at any step the multiplication or addition overflows, return `0` immediately.

A subtle detail: in Rust, the `%` operator preserves the sign of the dividend. If `num` is negative, `digit` will be negative too. This means we do not need to handle the sign separately: `checked_add` with a negative digit effectively subtracts, and overflow detection works correctly for both ends of the range.

```rust
impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut num = x;
        let mut rev = 0i32;

        while num != 0 {
            let digit = num % 10;
            num /= 10;

            match rev.checked_mul(10).and_then(|v| v.checked_add(digit)) {
                Some(val) => rev = val,
                None => return 0,
            }
        }

        rev
    }
}
```

## Conclusion

This problem is a reminder that integer arithmetic is not as innocent as it seems. The difference between a correct program and one with undefined behavior can be a single unguarded multiplication.

Rust turns what in other languages is programmer discipline into a **type system guarantee**. The `checked_*` methods are not a luxury: they are the correct way to work with arithmetic that can overflow.
