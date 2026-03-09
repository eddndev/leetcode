---
title: "0214 Shortest Palindrome - EN"
problemUrl: "https://leetcode.com/problems/shortest-palindrome/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["string", "kmp", "palindrome", "rolling-hash"]
complexity:
  time: "O(N) where N is the length of the string"
  space: "O(N) for the combined string and the LPS array"
---

# The Broken Mirror: Building Palindromes with KMP

## The Problem
Given a string `s`, find the shortest palindrome that can be formed by adding characters **only to the beginning** of `s`. In other words, we must prepend the fewest characters possible to turn `s` into a palindrome.

## The Initial Intuition

My first reaction was to think about brute force: try each possible palindromic prefix of `s`, from longest to shortest. The longest palindromic prefix I find tells me exactly which characters are left over at the end -- those characters, reversed, are what I need to prepend. But checking whether each prefix is a palindrome costs `O(N)`, and there are `N` possible prefixes, so the naive approach is `O(N^2)`. For a string of up to 50,000 characters, I needed something better.

The key question is: *what is the longest palindromic prefix of `s`?* If I find it efficiently, the rest of the problem is trivial -- I just need to take the characters that remain after that prefix, reverse them, and prepend them.

## The Strategy: KMP to the Rescue

### The Connection to Pattern Matching

This is where the solution becomes elegant. I consider the string `s` and its reverse `rev(s)`. If I concatenate `s + '#' + rev(s)`, I can use the KMP prefix table (also known as the LPS table -- Longest Proper Prefix which is also Suffix) to find exactly what I need.

The value at the last position of the LPS table tells me the length of the longest prefix of `s` that matches a suffix of `rev(s)`. But a suffix of `rev(s)` *is* a prefix of `s` read backwards. So this match indicates precisely the length of the longest palindromic prefix of `s`.

### The Critical Separator

The `'#'` character between `s` and `rev(s)` is essential. Without it, false matches could occur that cross the boundary between the two strings, producing an LPS value larger than the correct one. The separator guarantees that any detected match genuinely corresponds to a prefix of `s` that is a palindrome.

### A Concrete Example

With `s = "aacecaaa"`:
```
s       = "aacecaaa"
rev(s)  = "aaacecaa"
combined = "aacecaaa#aaacecaa"

LPS table:
a a c e c a a a # a a a c e c a a
0 1 0 0 0 1 1 1 0 1 1 1 0 0 0 1 2  <-- last value = 7
```

The last value is 7, meaning "aacecaa" (the first 7 characters of `s`) form the longest palindromic prefix. Only the last character 'a' remains, which reversed and prepended gives `"aaacecaaa"`.

Another example with `s = "abcd"`:
```
combined = "abcd#dcba"

LPS table:
a b c d # d c b a
0 0 0 0 0 0 0 0 1  <-- last value = 1
```

Only the first character 'a' is palindromic by itself. We need to prepend `rev("bcd") = "dcb"`, resulting in `"dcbabcd"`.

## The Algorithm Step by Step

1. Compute `rev(s)` -- the reverse of the string.
2. Build the combined string `s + '#' + rev(s)`.
3. Compute the LPS table over the combined string.
4. The value `lps[last]` gives us `palindrome_len`, the length of the longest palindromic prefix.
5. Take the first `n - palindrome_len` characters of `rev(s)` (the suffix of `s` that is not part of the palindrome, reversed).
6. Concatenate that fragment with the original `s`.

The beauty of this approach is that it reduces a palindrome problem to a pattern matching problem, solved in linear time with the KMP machinery.

## Rust Solution

```rust
impl Solution {
    pub fn shortest_palindrome(s: String) -> String {
        let n = s.len();
        if n == 0 {
            return s;
        }

        let s_bytes = s.as_bytes();
        let mut rev_s_bytes = s_bytes.to_vec();
        rev_s_bytes.reverse();

        let mut combined = Vec::with_capacity(2 * n + 1);
        combined.extend_from_slice(s_bytes);
        combined.push(b'#');
        combined.extend_from_slice(&rev_s_bytes);

        let m = combined.len();
        let mut lps = vec![0; m];
        let mut j = 0;

        for i in 1..m {
            while j > 0 && combined[i] != combined[j] {
                j = lps[j - 1];
            }
            if combined[i] == combined[j] {
                j += 1;
            }
            lps[i] = j;
        }

        let palindrome_len = lps[m - 1];

        let suffix_to_add =
            unsafe { String::from_utf8_unchecked(rev_s_bytes[0..n - palindrome_len].to_vec()) };

        suffix_to_add + &s
    }
}
```

The implementation operates entirely at the byte level with `as_bytes()`, avoiding the cost of manipulating multibyte UTF-8 characters when we know the input contains only ASCII. The combined string is built with `Vec::with_capacity(2 * n + 1)` to prevent reallocations during insertions. The KMP loop is classic: on a mismatch, it falls back using `lps[j - 1]` until it finds a compatible prefix or reaches the start. The use of `unsafe { String::from_utf8_unchecked(...) }` is safe here because the original bytes come from a valid `String` -- we are simply taking a subset of bytes that we already know are correct UTF-8. Finally, the concatenation `suffix_to_add + &s` leverages Rust's `Add<&str>` implementation for `String`, which takes ownership of the left side and appends the right side without additional copies.

## Conclusion

Shortest Palindrome is a fascinating example of how a problem seemingly about palindromes is best solved with pattern matching tools. The KMP LPS table, originally designed for substring searching, turns out to be exactly what we need to find the longest palindromic prefix. The construction `s + '#' + rev(s)` is the conceptual bridge connecting both worlds -- it transforms "find the largest prefix of `s` that is a palindrome" into "find the largest match between a prefix of `s` and a suffix of `rev(s)`", and that is precisely what KMP does in linear time.
