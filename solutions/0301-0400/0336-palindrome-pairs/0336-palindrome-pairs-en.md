---
title: "0336 Palindrome Pairs - EN"
problemUrl: "https://leetcode.com/problems/palindrome-pairs/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["hash-map", "string", "palindrome"]
complexity:
  time: "O(N * K^2)"
  space: "O(N * K)"
---

# When Two Words Complete Each Other

## The Problem
Given a list of unique words, find all pairs of distinct indices `(i, j)` such that the concatenation of `words[i] + words[j]` forms a palindrome.

## The Brute Force Trap

The naive approach screams at you: try every pair `(i, j)`, concatenate the strings, and check if the result is a palindrome. With up to 5000 words each up to 300 characters long, that's O(N^2 * K) checks, which can reach billions of operations. We need something smarter.

## The Key Observation

What makes a concatenation `words[i] + words[j]` a palindrome? There are really only a few structural cases to consider, and all of them revolve around splitting a word at every possible position and asking two questions:

1. **If the left portion of a word is a palindrome**, then the reverse of its right portion, placed before the word, would complete a full palindrome. So we look up `reverse(right portion)` in our dictionary and, if found, that word goes on the left.

2. **If the right portion of a word is a palindrome**, then the reverse of its left portion, placed after the word, would complete a full palindrome. So we look up `reverse(left portion)` in our dictionary and, if found, that word goes on the right.

By iterating over every possible split point `j` from `0` to `len` for each word, we cover all cases, including the important edge case where one of the two portions is the entire word (and the other is empty). When the split is at position `0`, the left portion is empty (which is trivially a palindrome), and we're looking for the full reverse of the word. When the split is at position `len`, the right portion is empty, and we're looking for the full reverse again, but placed differently.

The extra condition `s2.len() > 0` in the second check prevents counting the same pair twice when both portions reduce to the same empty-string scenario.

## Walking Through an Example

Consider `words = ["abcd", "dcba", "lls", "s", "sssll"]`:

- For `"abcd"`: at split `j=4`, the right portion is empty (palindrome), the left portion is `"abcd"`, its reverse `"dcba"` is in the map. So `("abcd", "dcba")` is a valid pair. At split `j=0`, the left portion is empty (palindrome), right portion is `"abcd"`, its reverse `"dcba"` is in the map. So `("dcba", "abcd")` is also a valid pair.
- For `"lls"`: at split `j=2`, the right portion is `"s"` (palindrome), the left portion is `"ll"`, its reverse `"ll"` is not in the map. At split `j=1`, the left portion is `"l"` (palindrome), right portion is `"ls"`, its reverse `"sl"` is not in the map. But at split `j=0`, left is empty (palindrome), right is `"lls"`, its reverse `"sll"` is not in the map. At split `j=3`, right is empty (palindrome), left is `"lls"`, its reverse `"sll"` is not in the map. However, for `"s"` at split `j=0`, left is empty, right is `"s"`, reverse is `"s"`, which is `"s"` itself, no match (same index). But for `"sssll"` at split `j=3`, left is `"sss"` (palindrome), right is `"ll"`, reverse `"ll"` is not in the map. At split `j=2`, left is `"ss"` (palindrome), right is `"sll"`, reverse `"lls"` is in the map. So `("lls", "sssll")` is valid.

Result: `[[0,1], [1,0], [2,4], [3,2]]`.

## Rust Solution

```rust
use std::collections::HashMap;

impl Solution {
    pub fn palindrome_pairs(words: Vec<String>) -> Vec<Vec<i32>> {
        let mut map = HashMap::new();
        // Construimos un mapa de palabra -> índice para búsquedas rápidas
        for (i, word) in words.iter().enumerate() {
            map.insert(word.as_str(), i);
        }

        let mut res = Vec::new();

        for (i, word) in words.iter().enumerate() {
            let n = word.len();
            let chars = word.as_bytes();

            for j in 0..=n {
                let s1 = &chars[0..j];
                let s2 = &chars[j..n];

                if is_palindrome(s1) {
                    let mut rev_s2 = s2.to_vec();
                    rev_s2.reverse();
                    if let Ok(target) = std::str::from_utf8(&rev_s2) {
                        if let Some(&k) = map.get(target) {
                            if k != i {
                                res.push(vec![k as i32, i as i32]);
                            }
                        }
                    }
                }

                if s2.len() > 0 && is_palindrome(s2) {
                    let mut rev_s1 = s1.to_vec();
                    rev_s1.reverse();
                    if let Ok(target) = std::str::from_utf8(&rev_s1) {
                        if let Some(&k) = map.get(target) {
                            if k != i {
                                res.push(vec![i as i32, k as i32]);
                            }
                        }
                    }
                }
            }
        }

        res
    }
}

fn is_palindrome(chars: &[u8]) -> bool {
    let mut left = 0;
    let mut right = chars.len();
    if right == 0 {
        return true;
    }
    right -= 1;

    while left < right {
        if chars[left] != chars[right] {
            return false;
        }
        left += 1;
        right -= 1;
    }
    true
}
```

The Rust implementation starts by building a `HashMap` from each word to its index for O(1) lookups. For every word, we iterate through all possible split positions `j` from `0` to `n` inclusive, producing a left portion `s1 = word[0..j]` and a right portion `s2 = word[j..n]`. When `s1` is a palindrome, we check if the reverse of `s2` exists in the map, which would mean prepending that word creates a full palindrome. Symmetrically, when `s2` is a palindrome, we check if the reverse of `s1` exists, which would mean appending that word creates a palindrome. The `s2.len() > 0` guard in the second branch prevents double-counting the case where `j = n` (empty right portion), since the `j = 0` case in the first branch already handles the full-reverse lookup. Working at the byte level with `as_bytes()` avoids UTF-8 overhead for these ASCII-only inputs, and `std::str::from_utf8` safely converts back for the map lookup.

## Conclusion

Palindrome Pairs is a problem where brute force is tantalizingly close to feasible but not quite there. The insight that saves us is decomposing the palindrome condition into structural cases: for every possible split of a word, if one side is already a palindrome, the reverse of the other side is exactly the partner we need. A hash map turns each partner lookup into constant time, bringing the total complexity down to O(N * K^2), where K is the maximum word length. It's a beautiful example of how understanding the structure of the output you're looking for can dramatically reduce the search space.
