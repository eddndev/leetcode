---
title: "0420 Strong Password Checker - EN"
problemUrl: "https://leetcode.com/problems/strong-password-checker/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["greedy", "string"]
complexity:
  time: "O(N) where N is the length of the password"
  space: "O(N) for storing the repeating sequences"
---

# The Locksmith's Three-Way Balancing Act

## The Problem
Given a string `password`, return the minimum number of steps required to make it a strong password. A strong password has at least 6 characters, at most 20 characters, contains at least one lowercase letter, at least one uppercase letter, at least one digit, and does not contain three repeating characters in a row. In one step, you can insert a character, delete a character, or replace a character.

## The Initial Intuition

At first glance, this problem looks like it should yield to a clean dynamic programming formulation. But the interplay between three different constraints -- length requirements, character type requirements, and repeating character restrictions -- makes DP state spaces explode. What makes this problem truly hard is that the three operations (insert, delete, replace) interact with the constraints in fundamentally different ways, and the optimal strategy shifts dramatically depending on whether the password is too short, just right, or too long.

The key insight that unlocks the solution is to split the analysis into three regimes based on length, then within each regime figure out how the operations work together.

## The Three Regimes

### Too Short (length < 6)

When the password has fewer than 6 characters, I need to add characters. Every insertion simultaneously does two things: it moves me toward the minimum length of 6, and it can fix a missing character type. It can also break a repeating sequence by inserting a different character in the middle. So the answer is simply the maximum of `missing_types` and `6 - n`. Both constraints need to be satisfied, and since each insertion can address both, I just take whichever demands more operations.

### Just Right (6 <= length <= 20)

When the length is already within bounds, I never need to insert or delete. The only operation that matters is replacement. A sequence of `k` identical characters requires `floor(k / 3)` replacements to break it up -- I place a different character at every third position, which prevents any three consecutive characters from matching. The answer is the maximum of `missing_types` and the total replacements needed. Again, replacements can simultaneously satisfy both goals: replacing a character can introduce a missing type while also breaking a repeating run.

### Too Long (length > 20)

This is where the real complexity lives. I definitely need `n - 20` deletions to bring the length down. But deletions can also reduce the number of replacements needed for repeating sequences, and I want to use them as efficiently as possible.

Here is the critical observation: for a repeating sequence of length `k`, I need `floor(k / 3)` replacements. But if I delete characters from that sequence, I can reduce `k` and thus reduce the replacement count. The efficiency of deletion depends on `k mod 3`:

- If `k mod 3 == 0`: deleting just 1 character reduces the replacement count by 1. These are the most efficient targets for deletions.
- If `k mod 3 == 1`: deleting 2 characters reduces the replacement count by 1. Still worthwhile but less efficient.
- If `k mod 3 == 2` (or any other case): I need to delete 3 characters to reduce the replacement count by 1. This is the least efficient use of deletions.

So I prioritize deletions greedily: first spend deletions on sequences where `k mod 3 == 0` (1 deletion saves 1 replacement), then on sequences where `k mod 3 == 1` (2 deletions save 1 replacement), then use remaining deletions at a rate of 3-to-1 on everything else.

After applying deletions optimally, whatever replacements remain must still be done. And the final answer is `deletions + max(missing_types, remaining_replacements)`.

## Why the Greedy Works

The greedy priority for deletions is correct because we have a fixed deletion budget (`n - 20`) and we want to maximize the reduction in replacements. Each "mod 0" sequence gives us the best exchange rate: one deletion for one fewer replacement. Each "mod 1" sequence is next best: two deletions for one fewer replacement. Everything else costs three deletions per saved replacement. By spending our budget in this order, we minimize the leftover replacements.

The `max(missing_types, ...)` appears in every regime because missing character types and structural fixes are independent constraints that can sometimes be satisfied by the same operation but never interfere with each other.

## Rust Solution

```rust
use std::cmp::max;
use std::cmp::min;

impl Solution {
    pub fn strong_password_checker(password: String) -> i32 {
        let chars: Vec<char> = password.chars().collect();
        let n = chars.len() as i32;

        let has_lower = chars.iter().any(|c| c.is_ascii_lowercase());
        let has_upper = chars.iter().any(|c| c.is_ascii_uppercase());
        let has_digit = chars.iter().any(|c| c.is_ascii_digit());

        let missing_types = !has_lower as i32 + !has_upper as i32 + !has_digit as i32;

        let mut replace = 0;
        let mut one_seq = Vec::new();
        let mut two_seq = Vec::new();

        let mut i = 2;
        while i < n {
            if chars[i as usize] == chars[(i - 1) as usize]
                && chars[i as usize] == chars[(i - 2) as usize]
            {
                let mut length = 2;
                while i < n && chars[i as usize] == chars[(i - 1) as usize] {
                    length += 1;
                    i += 1;
                }
                replace += length / 3;
                if length % 3 == 0 {
                    one_seq.push(length);
                } else if length % 3 == 1 {
                    two_seq.push(length);
                }
            } else {
                i += 1;
            }
        }

        if n < 6 {
            max(missing_types, 6 - n)
        } else if n <= 20 {
            max(missing_types, replace)
        } else {
            let delete_needed = n - 20;
            let mut delete_left = delete_needed;

            replace -= min(delete_left, one_seq.len() as i32 * 1) / 1;
            delete_left = max(0, delete_left - one_seq.len() as i32 * 1);

            replace -= min(delete_left, two_seq.len() as i32 * 2) / 2;
            delete_left = max(0, delete_left - two_seq.len() as i32 * 2);

            replace -= delete_left / 3;

            delete_needed + max(missing_types, replace)
        }
    }
}
```

The solution starts by scanning the password for character type presence. The boolean-to-integer cast `!has_lower as i32` elegantly counts missing types. Then it walks through the string detecting repeating sequences of three or more identical characters. For each sequence, it computes `length / 3` replacements needed and classifies the sequence by `length % 3` into `one_seq` (mod 0, where 1 deletion saves 1 replacement) or `two_seq` (mod 1, where 2 deletions save 1 replacement). Sequences where `length % 3 == 2` do not need special tracking because they only benefit from deletions at the generic 3-to-1 rate.

In the too-long branch, the deletion budget is spent in priority order. The expression `min(delete_left, one_seq.len() as i32 * 1) / 1` caps the deletions at either the budget or the number of mod-0 sequences. The division by 1 is a no-op but maintains the pattern: for mod-1 sequences the division is by 2, reflecting the 2-deletions-per-saved-replacement exchange rate. Finally, `delete_left / 3` handles any remaining budget at the 3-to-1 rate applied uniformly to the leftover sequences.

## Conclusion

Strong Password Checker is one of those rare problems where the difficulty lies not in any single algorithmic technique but in the careful orchestration of multiple interacting constraints. There is no fancy data structure, no deep recursion, no clever mathematical identity -- just a thorough case analysis combined with a greedy priority scheme for allocating deletions. The three-regime decomposition keeps the logic manageable, and the mod-3 classification of repeating sequences transforms an apparently intractable optimization into a linear-time greedy scan. What makes this problem beautiful, and frustrating, is that the answer is conceptually simple once you see it, but discovering the right decomposition demands patience and precision.
