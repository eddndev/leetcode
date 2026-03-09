---
title: "0127 Word Ladder - EN"
problemUrl: "https://leetcode.com/problems/word-ladder/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["breadth-first-search", "hash-table", "string"]
complexity:
  time: "O(M^2 * N)"
  space: "O(M^2 * N)"
---

# Crossing the Bridge One Letter at a Time

## The Problem
Given two words, `beginWord` and `endWord`, and a dictionary `wordList`, return the number of words in the **shortest transformation sequence** from `beginWord` to `endWord`, such that every adjacent pair of words differs by exactly one letter, and every intermediate word must exist in `wordList`. Return `0` if no such sequence exists. Note that `beginWord` does not need to be in `wordList`.

## The Hidden Graph

At first, this looks like a string manipulation puzzle, but there is a graph lurking beneath the surface. Each word in the dictionary is a node, and two nodes are connected by an edge if and only if they differ by exactly one character. The problem then becomes: find the shortest path from `beginWord` to `endWord` in this implicit graph.

The moment I recognized this as a shortest-path problem on an unweighted graph, the algorithm became obvious: **Breadth-First Search**. BFS guarantees that the first time we reach a node, we have found the shortest path to it. No need for Dijkstra, no need for dynamic programming -- just a simple queue and level-by-level expansion.

## The Character Swapping Strategy

The naive way to find neighbors would be to compare every word against every other word in the dictionary, checking if they differ by exactly one letter. That is O(N * M) per word, where N is the dictionary size and M is the word length. For a large dictionary, this gets expensive.

Instead, I generate neighbors on the fly: for each position in the current word, I try all 26 lowercase letters. If the resulting word exists in the dictionary, it is a valid neighbor. This is O(26 * M) per word, which is effectively O(M) -- independent of dictionary size. The HashSet makes the lookup O(1) amortized.

There is a subtle but critical detail: once a word is discovered as a neighbor, I **remove it from the set immediately**, before pushing it onto the queue. This serves the same purpose as a "visited" set but is cheaper -- we do not need a separate data structure, and we prevent the same word from being enqueued multiple times at different levels. It works because BFS processes nodes level by level, so the first time we encounter a word is guaranteed to be at the shortest distance.

## Walking Through an Example

Consider `beginWord = "hit"`, `endWord = "cog"`, `wordList = ["hot", "dot", "dog", "lot", "log", "cog"]`.

- **Level 1:** Start with `"hit"`. Try all single-character changes. `"hot"` is in the set. Remove it, enqueue `("hot", 2)`.
- **Level 2:** Process `"hot"`. Changing characters yields `"dot"` and `"lot"`. Both are in the set. Remove and enqueue both at level 3.
- **Level 3:** Process `"dot"`. Yields `"dog"`. Enqueue at level 4. Process `"lot"`. Yields `"log"`. Enqueue at level 4.
- **Level 4:** Process `"dog"`. Yields `"cog"`. That matches `endWord`. Return 5.

The answer is **5**: the sequence `"hit" -> "hot" -> "dot" -> "dog" -> "cog"`.

### Why Remove Instead of Mark Visited?

Using the word set as both dictionary and visited tracker is elegant for two reasons. First, it eliminates the need for a separate HashSet of visited words, saving memory. Second, it prevents a subtle bug: if we only check membership without removing, multiple words at the same BFS level could independently discover the same neighbor, leading to duplicate enqueue operations and wasted work. Removing on discovery ensures each word is processed exactly once.

## Rust Solution

```rust
use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        // Convert the vector to a hashset for search
        let mut word_set: HashSet<String> = word_list.into_iter().collect();

        // If the final word doesn't exist, return 0
        if !word_set.contains(&end_word) {
            return 0;
        }

        // Init the queue for BFS
        let mut queue: VecDeque<(String, i32)> = VecDeque::new();
        queue.push_back((begin_word, 1));

        while let Some((current_word, level)) = queue.pop_front() {
            // If end_word reached, return's the current level
            if current_word == end_word {
                return level;
            }

            let mut current_bytes = current_word.into_bytes();
            let len = current_bytes.len();

            for i in 0..len {
                let original_char = current_bytes[i];

                for c in b'a'..=b'z' {
                    if c == original_char {
                        continue;
                    }

                    current_bytes[i] = c;

                    if let Ok(next_word_str) = std::str::from_utf8(&current_bytes) {
                        if word_set.contains(next_word_str) {
                            word_set.remove(next_word_str);

                            queue.push_back((next_word_str.to_string(), level + 1));
                        }
                    }
                }

                current_bytes[i] = original_char;
            }
        }

        0
    }
}
```

The Rust implementation makes several choices that feel natural in the language. Converting the word into a byte array with `into_bytes()` lets us manipulate individual characters as `u8` values, iterating through `b'a'..=b'z'` without any casting ceremony. The `std::str::from_utf8` check is Rust being cautious -- since we only substitute ASCII lowercase letters into an originally valid UTF-8 string, this will always succeed, but Rust's type system insists on the validation. The `word_set` doubles as the dictionary and the visited set: each word is removed the instant it is discovered, which is both memory-efficient and correct. The `while let Some(...)` pattern drains the VecDeque idiomatically, and the early return when `current_word == end_word` short-circuits the search the moment the target is found.

## Conclusion

Word Ladder is one of those problems that teaches you to see graphs where there are none. The words are nodes, the single-character edits are edges, and the shortest transformation sequence is simply BFS on this implicit graph. The key engineering insight is the neighbor generation strategy -- swapping each position through the alphabet and checking against a HashSet rather than comparing against every word in the dictionary. Combined with the trick of removing words from the set on discovery to avoid revisits, the entire solution runs in O(M^2 * N) time, where M is the word length and N is the dictionary size, with BFS guaranteeing optimality by construction.
