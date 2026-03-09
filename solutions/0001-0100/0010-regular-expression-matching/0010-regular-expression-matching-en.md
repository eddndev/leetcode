---
title: "0010 Regular Expression Matching - EN"
problemUrl: "https://leetcode.com/problems/regular-expression-matching/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["string", "dynamic-programming", "recursion"]
complexity:
  time: "O(M * N)"
  space: "O(M * N)"
---

# When Patterns Come Alive

## The Problem
Given a string `s` and a pattern `p`, implement regular expression matching with support for `'.'` (matches any single character) and `'*'` (matches zero or more of the preceding element). The matching must cover the **entire** input string, not just a part of it.

## The First Impression

When I first saw this problem, I thought it would just be a matter of walking through both strings character by character. But then the `'*'` showed up, and everything changed. The asterisk doesn't act alone: it depends on the character that precedes it, and it can mean "ignore me completely" or "repeat me as many times as you need." That duality is what turns a seemingly simple problem into a genuinely hard one.

My first instinct was recursion. If the pattern has `a*`, I can choose to consume no characters (zero occurrences) or consume one and keep trying. But pure recursion explodes exponentially. That's where **dynamic programming** enters the scene.

## Building the DP Table

My approach was to build a table `dp[i][j]` where each cell answers the question: "Do the first `i` characters of `s` match the first `j` characters of `p`?"

### The base case

Cell `dp[0][0]` is `true`: an empty string matches an empty pattern. But there's a subtle detail: a pattern like `a*b*c*` can also match an empty string, because each `x*` can represent zero occurrences. That's why we need to initialize the first row by walking through the pattern and propagating `dp[0][j-2]` every time we encounter a `'*'`.

### The transitions

For each cell `dp[i][j]`, there are three possible scenarios:

1. **Direct match or dot:** If `p[j-1]` equals `s[i-1]` or is `'.'`, we simply inherit the diagonal result: `dp[i][j] = dp[i-1][j-1]`.

2. **Asterisk:** Here lies the real complexity. The `'*'` gives us two paths:
   - **Zero occurrences:** We ignore the last two characters of the pattern (`x*`), so we look at `dp[i][j-2]`.
   - **One or more occurrences:** If the character preceding the `*` matches `s[i-1]` (or is `'.'`), we can "consume" one character from `s` and stay at the same position in the pattern: `dp[i-1][j]`.

3. **No match:** The cell stays `false`.

The elegance of this approach is that it captures all the complexity of backtracking in a two-dimensional table, avoiding redundant work.

## C Solution

```c
#include <stdbool.h>
#include <string.h>

bool isMatch(char *s, char *p) {
    int m = strlen(s);
    int n = strlen(p);

    // DP Table
    bool dp[m + 1][n + 1];

    for (int i = 0; i <= m; i++) {
        for (int j = 0; j <= n; j++) {
            dp[i][j] = false;
        }
    }

    // Initial State
    // Empty String VS Empty pattern is TRUE
    dp[0][0] = true;

    for (int j = 1; j <= n; j++) {
        if (p[j - 1] == '*') {
            dp[0][j] = dp[0][j - 2];
        }
    }

    for (int i = 1; i <= m; i++) {
        for (int j = 1; j <= n; j++) {
            if (p[j - 1] == '.' || p[j - 1] == s[i - 1]) {
                dp[i][j] = dp[i - 1][j - 1];
            }

            else if (p[j - 1] == '*') {
                bool zero_match = dp[i][j - 2];

                bool char_match = (p[j - 2] == s[i - 1] || p[j - 2] == '.');

                bool one_plus_match = char_match && dp[i - 1][j];

                dp[i][j] = zero_match || one_plus_match;
            }

            else {
                dp[i][j] = false;
            }
        }
    }

    return dp[m][n];
}
```

In C, the implementation is fairly straightforward. We declare the DP table on the stack using VLAs (Variable Length Arrays), which saves us the complexity of managing dynamic memory. The variables `zero_match`, `char_match`, and `one_plus_match` make the asterisk logic easy to follow, instead of cramming everything into a single cryptic line.

## Rust Solution

```rust
impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let s = s.as_bytes();
        let p = p.as_bytes();
        let (m, n) = (s.len(), p.len());
        let mut dp = vec![vec![false; n + 1]; m + 1];

        dp[0][0] = true;

        for j in 2..=n {
            if p[j - 1] == b'*' {
                dp[0][j] = dp[0][j - 2];
            }
        }

        for i in 1..=m {
            for j in 1..=n {
                if p[j - 1] == b'*' {
                    dp[i][j] = dp[i][j - 2];
                    if p[j - 2] == b'.' || p[j - 2] == s[i - 1] {
                        dp[i][j] = dp[i][j] || dp[i - 1][j];
                    }
                } else if p[j - 1] == b'.' || p[j - 1] == s[i - 1] {
                    dp[i][j] = dp[i - 1][j - 1];
                }
            }
        }

        dp[m][n]
    }
}
```

The Rust version is more compact. We convert the strings to `&[u8]` with `as_bytes()` so we can compare byte by byte efficiently, avoiding the complexity of working with Unicode characters where we don't need them. The empty-pattern initialization loop starts at `j = 2` because a `'*'` can never appear at position 0 (it always needs a preceding character).

## Conclusion

This problem is a classic dynamic programming exercise that teaches something fundamental: when a problem has branching decisions (consume or not consume, match zero or more times), DP lets us explore all branches without repeating work. The `dp` table acts as shared memory of all the subproblems we've already solved, and the final answer is simply waiting for us at `dp[m][n]`.
