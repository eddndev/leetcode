---
title: "0282 Expression Add Operators - EN"
problemUrl: "https://leetcode.com/problems/expression-add-operators/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["backtracking", "math", "string", "recursion"]
complexity:
  time: "O(N * 4^N) where N is the length of the string"
  space: "O(N) for the recursion depth"
---

# Injecting Operators into Numerical Chaos

## The Problem
Given a string `num` that contains only digits and an integer `target`, return all possibilities of inserting the binary operators `+`, `-`, and `*` between the digits of `num` so that the resulting expression evaluates to `target`. Operands in the returned expressions must not contain leading zeros.

## The Combinatorial Explosion

This problem looks simple on the surface: take a string of digits and insert operators between them. But the real complexity hides on two fronts. First, between each pair of consecutive digits I have four choices: insert nothing (concatenate the digits to form a longer number), insert `+`, insert `-`, or insert `*`. With `N` digits, that generates up to `4^N` possible combinations. Second, multiplication breaks the natural left-to-right associativity of evaluation: `2+3*4` is not `(2+3)*4 = 20` but `2+(3*4) = 14`, because multiplication has higher precedence.

My first instinct was to generate all possible expressions and evaluate them with a parser, but that would be inefficient. What I needed was to evaluate the expression *as I build it*, maintaining enough information to handle multiplication precedence correctly.

## The Strategy: Backtracking with Last Operand Memory

### The Multiplication Key

The central trick of this problem is tracking the last operand used. When I process `2 + 3` and the accumulated value is `5`, if the next operator is `*4`, I can't simply compute `5 * 4 = 20`. I need to *undo* the previous addition: `(5 - 3) + (3 * 4) = 2 + 12 = 14`. That is exactly what it means to respect operator precedence without a full parser.

That's why my backtracking function maintains two values:
- `current_val`: the accumulated result of the expression so far
- `last_operand`: the last operand applied, needed to undo the previous operation in case of multiplication

### Partitioning the String

At each recursion step, I consider all possible substrings starting at the current index. The substring `num[index..=i]` forms a candidate operand. If that operand has more than one digit and starts with '0', I discard it immediately -- leading zeros are not allowed.

For the first operand (when `index == 0`), there's no operator to insert: I simply use the number as the initial value and as the last operand. For subsequent operands, I try all three operations:

- **Addition**: `current_val + val`, with `last_operand = val`
- **Subtraction**: `current_val - val`, with `last_operand = -val`
- **Multiplication**: `(current_val - last_operand) + (last_operand * val)`, with `last_operand = last_operand * val`

Notice that for subtraction, the last operand is stored as `-val`. This is intentional: if multiplication follows, I need to undo the entire subtraction, including the sign.

### A Concrete Example

With `num = "232"` and `target = 8`:

```
Index 0: try "2" as first operand
  current_val = 2, last_operand = 2

  Index 1: try "3"
    +: current_val = 2+3 = 5, last = 3
      Index 2: try "2"
        +: 5+2 = 7 != 8
        -: 5-2 = 3 != 8
        *: (5-3)+(3*2) = 2+6 = 8 == 8 -> "2+3*2" collected!
    -: current_val = 2-3 = -1, last = -3
      Index 2: try "2"
        +: -1+2 = 1 != 8
        -: -1-2 = -3 != 8
        *: (-1-(-3))+(-3*2) = 2-6 = -4 != 8
    *: current_val = (2-2)+(2*3) = 6, last = 6
      Index 2: try "2"
        +: 6+2 = 8 == 8 -> "2*3+2" collected!
        -: 6-2 = 4 != 8
        *: (6-6)+(6*2) = 12 != 8

  Index 1: try "32" (concatenation)
    +: 2+32 = 34 != 8
    -: 2-32 = -30 != 8
    *: (2-2)+(2*32) = 64 != 8

Index 0: try "23" as first operand
  current_val = 23, last = 23
  ...no combination reaches 8

Index 0: try "232" as first operand
  current_val = 232 != 8
```

Result: `["2+3*2", "2*3+2"]`.

## The Overflow Guard

A subtle but critical detail: although `target` is received as `i32`, intermediate values can exceed that range. Consider `num = "999999999"` -- concatenations generate enormous numbers, and multiplications amplify them further. That's why the implementation immediately converts `target` to `i64` and performs all arithmetic in 64 bits, avoiding silent overflows that would produce incorrect results.

## Rust Solution

```rust
impl Solution {
    pub fn add_operators(num: String, target: i32) -> Vec<String> {
        let mut result = Vec::new();
        if num.is_empty() {
            return result;
        }

        let chars: Vec<char> = num.chars().collect();
        let target = target as i64;

        Self::backtrack(0, "", 0, 0, &chars, target, &mut result);

        result
    }

    fn backtrack(
        index: usize,
        path: &str,
        current_val: i64,
        last_operand: i64,
        chars: &Vec<char>,
        target: i64,
        result: &mut Vec<String>,
    ) {
        if index == chars.len() {
            if current_val == target {
                result.push(path.to_string());
            }
            return;
        }

        for i in index..chars.len() {
            if i > index && chars[index] == '0' {
                break;
            }

            let part_str: String = chars[index..=i].iter().collect();
            let val: i64 = part_str.parse().unwrap();

            if index == 0 {
                Self::backtrack(i + 1, &part_str, val, val, chars, target, result);
            } else {
                Self::backtrack(
                    i + 1,
                    &format!("{}+{}", path, part_str),
                    current_val + val,
                    val,
                    chars,
                    target,
                    result,
                );

                Self::backtrack(
                    i + 1,
                    &format!("{}-{}", path, part_str),
                    current_val - val,
                    -val,
                    chars,
                    target,
                    result,
                );

                Self::backtrack(
                    i + 1,
                    &format!("{}*{}", path, part_str),
                    (current_val - last_operand) + (last_operand * val),
                    last_operand * val,
                    chars,
                    target,
                    result,
                );
            }
        }
    }
}
```

The Rust implementation uses `&str` for the `path` parameter rather than `String`, which avoids transferring ownership on each recursive call. Each backtracking branch builds a new string with `format!`, which lives in the corresponding stack frame and is automatically discarded when backtracking. Using `chars: &Vec<char>` allows O(1) indexed access to characters, avoiding the complexity of iterating over UTF-8 bytes directly. The operand conversion with `part_str.parse().unwrap()` is safe because the string contains only digits -- guaranteed by the problem constraints. The condition `i > index && chars[index] == '0'` implements leading-zero pruning with a `break` instead of `continue`: once I detect that the first digit is '0', there's no point in trying longer substrings, since they would all have a leading zero.

## Conclusion

Expression Add Operators is a problem that blends combinatorial generation with real-time arithmetic evaluation. The difficulty lies not in the exhaustive search itself, but in the elegant handling of operator precedence without building a full expression tree. The trick of tracking the last operand allows undoing the previous operation when a multiplication appears, simulating correct precedence with just two state variables. The overflow protection via `i64` and the leading-zero pruning with `break` are details that separate a correct solution from one that fails on edge cases.
