---
title: "0140 Word Break II - EN"
problemUrl: "https://leetcode.com/problems/word-break-ii/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["dynamic-programming", "backtracking", "memoization", "hash-table", "string"]
complexity:
  time: "O(N * 2^N)"
  space: "O(N * 2^N)"
---

# Splitting a String into Every Possible Sentence

## The Problem
Given a string `s` and a dictionary of strings `wordDict`, add spaces in `s` to construct sentences where each word is a valid dictionary word. Return all such possible sentences in any order.

## Why This Problem Bites

Word Break I asks a yes-or-no question: *can* the string be segmented? That's a clean dynamic programming problem with O(N^2) behavior. Word Break II asks for *all* valid segmentations. That single word -- "all" -- transforms the problem from polynomial to potentially exponential. A string like `"aaa...a"` with `wordDict = ["a", "aa", "aaa", ...]` can produce a combinatorial explosion of valid sentences.

My first instinct was to use the same DP table from Word Break I and then somehow reconstruct paths. But the reconstruction itself is the hard part -- it's not enough to know *that* a position is reachable; I need to remember *which words* led there and follow every branching path. The natural tool for this is DFS with memoization: explore from each position, try every dictionary word that matches starting there, recursively solve the rest, and cache the results.

## The Strategy: DFS with Memoization

The idea is simple in spirit. Starting from index 0 of the string, I try every possible prefix. If that prefix exists in the dictionary, I recursively solve the remaining suffix. The results from the suffix get combined with the current word to form complete sentences.

Without memoization, this would revisit the same suffixes exponentially many times. Consider the string `"catsanddog"` with words `["cat", "cats", "and", "sand", "dog"]`. Both `"cat" + "sand..."` and `"cats" + "and..."` eventually need to solve the suffix `"dog"`. Memoization ensures that once I've computed all sentences starting from a given index, I never recompute them.

### Walking Through an Example

With `s = "catsanddog"` and `wordDict = ["cat", "cats", "and", "sand", "dog"]`:

```
dfs(0): try prefixes of "catsanddog"
  "cat" matches -> dfs(3): try prefixes of "sanddog"
    "sand" matches -> dfs(7): try prefixes of "dog"
      "dog" matches -> dfs(10): base case, return [""]
      -> returns ["dog"]
    -> returns ["sand dog"]
  -> returns ["cat sand dog"]

  "cats" matches -> dfs(4): try prefixes of "anddog"
    "and" matches -> dfs(7): CACHED -> ["dog"]
    -> returns ["and dog"]
  -> returns ["cats and dog"]

Final: ["cat sand dog", "cats and dog"]
```

Notice how `dfs(7)` is called twice but computed only once. The memo at index 7 stores `["dog"]` and returns it immediately the second time.

### The Base Case

When `start == s.len()`, we've consumed the entire string. Returning a vector containing a single empty string is the right move -- it signals "the rest of the string is empty, so the sentence is complete." The caller then appends nothing after the current word, producing a sentence that ends cleanly.

### Why a HashSet for the Dictionary?

Each call to `dfs` iterates over all possible end positions and checks if the substring `s[start..end]` exists in the dictionary. Using a `HashSet` makes each lookup O(1) amortized, instead of scanning the entire dictionary for each candidate prefix. This matters because the inner loop runs up to N times per call.

## Rust Solution

```rust
use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
        let word_set: HashSet<&str> = word_dict.iter().map(|w| w.as_str()).collect();
        let mut memo: HashMap<usize, Vec<String>> = HashMap::new();

        Self::dfs(0, &s, &word_set, &mut memo)
    }

    fn dfs(
        start: usize,
        s: &str,
        word_set: &HashSet<&str>,
        memo: &mut HashMap<usize, Vec<String>>,
    ) -> Vec<String> {
        if let Some(res) = memo.get(&start) {
            return res.clone();
        }

        if start == s.len() {
            return vec![String::new()];
        }

        let mut results = Vec::new();

        for end in start + 1..=s.len() {
            let word = &s[start..end];

            if word_set.contains(word) {
                let sub_sentences = Self::dfs(end, s, word_set, memo);

                for sub in sub_sentences {
                    let mut sentence = String::from(word);
                    if !sub.is_empty() {
                        sentence.push(' ');
                        sentence.push_str(&sub);
                    }
                    results.push(sentence);
                }
            }
        }

        memo.insert(start, results.clone());
        results
    }
}
```

The Rust implementation leans into the language's strengths with careful borrowing. The `word_set` is a `HashSet<&str>` that borrows from the original `word_dict`, avoiding cloning the dictionary strings. The `memo` is a `HashMap<usize, Vec<String>>` keyed by the starting index -- each entry stores every valid sentence that can be formed from `s[start..]`. The `dfs` function checks the memo first, returns immediately on a cache hit, and otherwise iterates over all possible end positions. When a prefix matches a dictionary word, it recurses on the remainder and assembles each sub-sentence by prepending the current word. The `if !sub.is_empty()` check handles the base case gracefully: when we're at the end of the string, we don't append a trailing space. After computing all sentences for a given start index, the results are cloned into the memo before returning -- a necessary cost since Rust's ownership model doesn't allow us to return and store the same vector without cloning.

## Conclusion

Word Break II is the kind of problem where the naive approach (generate all possible splits, check each one) and the optimal approach share the same worst-case complexity -- the output itself can be exponential. The value of memoization here isn't in changing the asymptotic worst case; it's in eliminating redundant computation for the many inputs where the number of valid sentences is manageable. The DFS-with-memo pattern is the natural fit: it explores the search space top-down, branches at every valid dictionary word, and caches results so that shared suffixes are never recomputed. It's backtracking with memory -- the best of both worlds.
