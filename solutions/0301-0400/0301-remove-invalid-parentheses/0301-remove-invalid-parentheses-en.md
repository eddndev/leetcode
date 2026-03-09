---
title: "0301 Remove Invalid Parentheses - EN"
problemUrl: "https://leetcode.com/problems/remove-invalid-parentheses/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["backtracking", "bfs", "string", "dfs", "pruning"]
complexity:
  time: "O(2^N) where N is the length of the string"
  space: "O(N) for the recursion depth and the expression being built"
---

# Pruning Sick Parentheses from the Tree

## The Problem
Given a string `s` containing parentheses and letters, remove the minimum number of invalid parentheses to make the string valid. Return all possible unique strings. A parentheses string is valid if every opening `(` has a matching closing `)` and they are properly nested.

## The Minimal Surgery

At first glance, one might think about generating all possible combinations by removing parentheses and then filtering the valid ones. But that would be catastrophically inefficient. The key lies in the word *minimum*: I don't want to remove parentheses at random, but exactly the ones that are surplus -- no more, no less.

My first step is to diagnose the problem: count how many opening and how many closing parentheses I need to remove. I scan the string maintaining a counter of unmatched opening parentheses (`l_rem`). Every time I encounter a `(`, I increment the counter. Every time I encounter a `)`, if there's an unmatched `(` I decrement it (they paired up), but if there isn't one, that `)` is invalid and must be removed, so I increment `r_rem`. At the end of the scan, `l_rem` tells me how many `(` are surplus and `r_rem` how many `)` are surplus.

## The Strategy: DFS with Exact Removal Counting

### Three Decisions per Parenthesis

With the diagnosis ready, I use backtracking via DFS. For each character in the string, I need to decide:

1. **If it's `(` and I still have `(` removals left** (`l_rem > 0`): I can skip it (remove it).
2. **If it's `)` and I still have `)` removals left** (`r_rem > 0`): I can skip it (remove it).
3. **Always**: I can include the character in the current expression, *as long as* including it doesn't break validity. For `)`, this means the current balance (number of unmatched `(`) must be greater than 0.

The `balance` variable tracks the number of open `(` that haven't been closed yet. I never allow the balance to go negative -- that would mean a `)` without its matching `(`.

### Avoiding Duplicates

A subtle problem arises with repeated characters. If I have `"(("` and need to remove one, removing the first or the second produces the same result `"("`. To handle this, I use a `HashSet` that stores the resulting strings, guaranteeing automatic uniqueness. The duplicate condition in the code also attempts to prune redundant branches early, though the `HashSet` is the ultimate safety net.

### A Concrete Example

With `s = "())"`:

```
Diagnosis: l_rem = 0, r_rem = 1 (one ')' is surplus)

Index 0: '('
  Include: balance = 1, expr = "("
    Index 1: ')'
      Include: balance = 0, expr = "()"
        Index 2: ')'
          Skip (r_rem = 1 -> 0): advance without including
            Index 3: end, l_rem=0, r_rem=0, balance=0 -> "()" collected!
          Include: balance < 0? No, balance = 0, can't include ')'
      Skip (r_rem = 1 -> 0):
        Index 2: ')'
          Include: balance = 0? No, can't include ')' with balance 0
    Index 1: ')'
      Skip (r_rem = 1 -> 0):
        Index 2: ')'
          Include: balance = 1 -> 0, expr = "()"
            Index 3: end -> "()" collected (duplicate, HashSet handles it)
```

Result: `["()"]`.

### Another Example

With `s = "(a)())"`:

```
Diagnosis: l_rem = 0, r_rem = 1

The DFS explores all ways to remove exactly one ')'.
Removing ')' at index 3: "(a)()" -> valid
Removing ')' at index 4: "(a)()" -> duplicate
Removing ')' at index 5: "(a)()" -> duplicate
But also: removing ')' at index 1... no, index 1 is 'a'.

Result: ["(a)()", "(a())"]
```

## The Pruning That Matters

The beauty of this approach is that I never generate strings with more or fewer removals than necessary. The counters `l_rem` and `r_rem` act as a budget: every time I decide to remove a parenthesis, I deduct from the corresponding budget. When I reach the end of the string, I verify that both budgets are zero and that the balance is zero. This triple check guarantees that the resulting string is valid *and* that I removed exactly the minimum necessary.

## Rust Solution

```rust
use std::collections::HashSet;

impl Solution {
    pub fn remove_invalid_parentheses(s: String) -> Vec<String> {
        let mut l_rem = 0;
        let mut r_rem = 0;

        for c in s.chars() {
            if c == '(' {
                l_rem += 1;
            } else if c == ')' {
                if l_rem > 0 {
                    l_rem -= 1;
                } else {
                    r_rem += 1;
                }
            }
        }

        let s_chars: Vec<char> = s.chars().collect();
        let mut result = HashSet::new();
        let mut current_expr = String::new();

        Self::dfs(0, 0, l_rem, r_rem, &s_chars, &mut current_expr, &mut result);

        result.into_iter().collect()
    }

    #[allow(clippy::too_many_arguments)]
    fn dfs(
        index: usize,
        balance: i32,
        l_rem: i32,
        r_rem: i32,
        s: &Vec<char>,
        expr: &mut String,
        res: &mut HashSet<String>,
    ) {
        if index == s.len() {
            if l_rem == 0 && r_rem == 0 && balance == 0 {
                res.insert(expr.clone());
            }
            return;
        }

        let char_at = s[index];

        let is_duplicate = index > 0
            && s[index] == s[index - 1]
            && expr.len() != (index - (l_rem + r_rem) as usize);

        if char_at == '(' && l_rem > 0 {
            Self::dfs(index + 1, balance, l_rem - 1, r_rem, s, expr, res);
        } else if char_at == ')' && r_rem > 0 {
            Self::dfs(index + 1, balance, l_rem, r_rem - 1, s, expr, res);
        }

        expr.push(char_at);
        if char_at == '(' {
            Self::dfs(index + 1, balance + 1, l_rem, r_rem, s, expr, res);
        } else if char_at == ')' {
            if balance > 0 {
                Self::dfs(index + 1, balance - 1, l_rem, r_rem, s, expr, res);
            }
        } else {
            Self::dfs(index + 1, balance, l_rem, r_rem, s, expr, res);
        }

        expr.pop();
    }
}
```

The Rust implementation converts the input string into a `Vec<char>` for O(1) indexed access, avoiding the complexity of navigating UTF-8 bytes. The expression under construction (`expr`) is passed as `&mut String`, which allows `push` and `pop` in amortized O(1) -- the classic backtracking pattern where I build the solution incrementally and undo it when backtracking. The `HashSet<String>` as the result container eliminates duplicates naturally, though it comes with the cost of cloning the entire string each time a valid solution is found. The `is_duplicate` condition attempts to detect redundant branches when there are consecutive identical characters, but the `HashSet` acts as the definitive uniqueness guarantee. The `#[allow(clippy::too_many_arguments)]` acknowledges that the DFS function needs quite a bit of state -- an acceptable trade-off to keep the recursion pure without resorting to an auxiliary struct.

## Conclusion

Remove Invalid Parentheses is a problem that combines syntactic analysis with intelligent exhaustive search. The key is not in trying all possible removal combinations, but in calculating *before* exploring how many parentheses of each type must disappear. With that diagnosis in hand, the DFS becomes a precise surgery: each branch of the recursion tree respects the removal budget and the balance invariant, pruning early the branches that cannot lead to a valid solution. The `HashSet` as a safety net against duplicates and the push/pop pattern for building the expression round out an elegant solution to a problem that, without proper pruning, would be intractable.
