---
title: "0224 Basic Calculator - EN"
problemUrl: "https://leetcode.com/problems/basic-calculator/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["stack", "string", "math", "recursion"]
complexity:
  time: "O(N) where N is the length of the string"
  space: "O(N) for the stack in the worst case of nested parentheses"
---

# Untangling Parentheses: A Calculator From Scratch

## The Problem
Given a string `s` representing a valid mathematical expression with digits, `+`, `-`, parentheses `(` and `)`, and spaces, evaluate the expression and return its result. Using built-in evaluation functions like `eval()` is not allowed.

## The Trap of Nested Parentheses

At first glance, adding and subtracting numbers seems trivial. But parentheses change everything. An expression like `1 - (2 + 3 - (4 + 5))` requires remembering the outer context while evaluating the inner subexpression. Each opening parenthesis creates a new "world" with its own partial result, and closing it requires merging that result back into the context we left behind.

A naive approach would be to find the innermost parentheses, evaluate them, replace the subexpression with its result, and repeat. But this involves multiple passes over the string and costly string manipulation. The key realization is that we can process the string in a single pass if we have a mechanism to *pause* and *resume* the outer computation when we encounter parentheses.

## The Strategy: A Stack as Context Memory

### The Natural Flow

My central idea was to treat the stack as context memory. As I scan the string from left to right, I maintain three variables: `result` (the current accumulated result), `current_number` (the number I am building digit by digit), and `sign` (the sign preceding the next number, `+1` or `-1`).

When I encounter a digit, I append it to the number I am building. When I encounter `+` or `-`, I update the sign. When the number is complete (because an operator or parenthesis arrived), I fold it into the result with `result += sign * current_number`.

### Parentheses as Save Points

The elegance emerges with parentheses:

1. **On opening `(`**: I save the current state onto the stack -- first `result`, then `sign` -- and reset both. It is like taking a "snapshot" of the outer computation before diving into the subexpression. The `result` resets to `0` and `sign` to `+1` because inside the parentheses I start a fresh calculation.

2. **On closing `)`**: I recover the previous sign and result from the stack. The subexpression result I just computed gets multiplied by the saved sign and added to the previous result. In essence, `outer_result + saved_sign * subexpression_result`.

### A Concrete Example

With `s = "1 - (2 + 3 - (4 + 5))"`:

```
Character '1': current_number=1
Character ' ': (ignored)
Character '-': result += 1*1 = 1. sign = -1
Character ' ': (ignored)
Character '(': push result=1, push sign=-1. result=0, sign=1
  Character '2': current_number=2
  Character '+': result += 1*2 = 2. sign = 1
  Character '3': current_number=3
  Character ' ': (ignored)
  Character '-': result += 1*3 = 5. sign = -1
  Character ' ': (ignored)
  Character '(': push result=5, push sign=-1. result=0, sign=1
    Character '4': current_number=4
    Character '+': result += 1*4 = 4. sign = 1
    Character '5': current_number=5
  Character ')': result += 1*5 = 9. pop sign=-1, pop result=5. result = 5 + (-1)*9 = -4
Character ')': pop sign=-1, pop result=1. result = 1 + (-1)*(-4) = 5

Final result: 5
```

Verification: `1 - (2 + 3 - (4 + 5)) = 1 - (5 - 9) = 1 - (-4) = 5`.

### Why the Stack Is Sufficient

Each pair of nested parentheses adds exactly two elements to the stack (result and sign), so the stack depth is proportional to the maximum nesting level. Since parentheses are guaranteed to be balanced by the problem constraints, we never end up with orphaned data on the stack. And because we process the string character by character without backtracking, total time is linear.

## Rust Solution

```rust
impl Solution {
    pub fn calculate(s: String) -> i32 {
        let mut stack = Vec::new();
        let mut result = 0;
        let mut current_number = 0;
        let mut sign = 1;

        let chars: Vec<char> = s.chars().collect();
        let mut i = 0;

        while i < chars.len() {
            let c = chars[i];

            if c.is_ascii_digit() {
                current_number = 0;
                while i < chars.len() && chars[i].is_ascii_digit() {
                    current_number = current_number * 10 + (chars[i] as i32 - '0' as i32);
                    i += 1;
                }
                result += sign * current_number;
                i -= 1;
            } else if c == '+' {
                sign = 1;
            } else if c == '-' {
                sign = -1;
            } else if c == '(' {
                stack.push(result);
                stack.push(sign);
                result = 0;
                sign = 1;
            } else if c == ')' {
                let prev_sign = stack.pop().unwrap();
                let prev_result = stack.pop().unwrap();
                result = prev_result + (prev_sign * result);
            }
            i += 1;
        }

        result
    }
}
```

The implementation traverses the string with an index `i` that advances manually to allow consuming multi-digit numbers in an inner loop. When it encounters a digit, it enters a sub-loop that accumulates the full number by multiplying by 10 and adding each digit. The `i -= 1` at the end of the digit block compensates for the general `i += 1` in the outer loop, preventing it from skipping the next character. The `+` and `-` operators simply update the `sign` variable without touching `result` -- the number will be folded into the result when fully read. The stack stores `(result, sign)` pairs as two separate integers in reverse order: first `result`, then `sign`, so that `pop()` retrieves the sign first and the accumulated result second. The formula `prev_result + (prev_sign * result)` elegantly merges the evaluated subexpression with the outer context, regardless of how many nesting levels deep we are.

## Conclusion

The Basic Calculator problem is a classic exercise in using a stack to manage nested contexts. By recognizing that parentheses act as save and restore points, the solution reduces to a linear scan where the stack preserves the outer state while we evaluate inner subexpressions. The result is an O(N) time and space algorithm that processes the string in a single pass, with no need for explicit recursion or complex parsing machinery.
