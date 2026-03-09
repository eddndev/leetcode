---
title: "0212 Word Search II - EN"
problemUrl: "https://leetcode.com/problems/word-search-ii/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["trie", "backtracking", "depth-first-search", "matrix"]
complexity:
  time: "O(M * N * 4 * 3^(L-1)) where L is the maximum word length"
  space: "O(W * L) where W is the number of words"
---

# Hunting Words Through a Maze of Letters

## The Problem
Given an `m x n` `board` of characters and a list of `words`, return all words from the list that can be found in the grid. Each word must be formed from letters in horizontally or vertically adjacent cells, and the same cell may not be used more than once within a single word.

## The Brute Force Trap

The naive approach would be to take each word, attempt to form it starting from every cell in the grid via DFS, and check if it completes. With `W` words of maximum length `L` on an `M x N` grid, that gives us `W * M * N * 4 * 3^(L-1)` operations. With thousands of words, this is unacceptable.

My first observation was that many words share prefixes. If "oath" and "oat" are both in the list, the path o-a-t gets traversed twice with the naive approach. What I needed was a structure that would let me search for *all* words simultaneously as I traverse the grid. That structure is a **Trie**.

## The Strategy: Trie + Simultaneous DFS

### The Trie as a Navigation Map

The idea is to build a Trie from all the words in the list. Then, for each cell in the grid, I start a DFS -- but instead of looking for a specific word, I *descend through the Trie in parallel*. If the current cell contains 'o' and the current Trie node has a child 'o', I advance simultaneously through the grid and the Trie. If that child doesn't exist, I prune the branch immediately -- no word in the list can begin with that prefix, so there's no reason to keep exploring.

This transforms the problem from "search W words independently" to "traverse the grid once, guided by the Trie." Each cell is explored in the context of prefixes that are still viable, and dead branches are eliminated instantly.

### Collection and Deduplication in a Single Pass

When the DFS reaches a Trie node that contains a complete word (stored directly in the node), I collect it into the result. But here's a subtlety: the same word could be found via multiple paths in the grid. To avoid duplicates without using a `HashSet`, I use `node.word.take()` -- I extract the word from the node and replace it with `None`. The first time I find it, I collect it; any subsequent attempt finds the node empty. Free deduplication baked right into the structure.

### In-Place Marking to Prevent Revisits

During the DFS, I need to mark visited cells to avoid reusing them within the same word. Instead of maintaining a separate boolean matrix, I temporarily replace the cell's character with `'#'`. When backtracking, I restore the original character. This saves memory and simplifies the code -- I only need to check `board[r][c] != '#'` to know whether a cell is available.

### A Concrete Example

With `board = [["o","a","a","n"],["e","t","a","e"],["i","h","k","r"],["i","f","l","v"]]` and `words = ["oath","pea","eat","rain"]`:

```
Trie built:
      root
      / \
     o   p    e    r
     |   |    |    |
     a   e    a    a
     |   |    |    |
     t   a    t    i
     |               |
     h               n

DFS from (0,0)='o': child 'o' exists -> advance
  (1,0)='e': no child 'e' from 'o' -> prune
  (0,1)='a': child 'a' exists -> advance
    (1,1)='t': child 't' exists -> advance
      (1,0)='e': no child 'e' from 't' -> prune
      (2,1)='h': child 'h' exists -> node has word="oath" -> collect!
```

The word "oath" is found in a single traversal. "eat" is found starting from (1,1) or (0,2). "pea" and "rain" are not found in the grid.

## Navigation by Wrapping

An interesting detail of this implementation is how it handles grid boundaries. Instead of checking `new_r >= 0` separately (which would require signed indices), I use `wrapping_add` to add the directions. If `r = 0` and `dr = -1`, then `0_usize.wrapping_add(-1_isize as usize)` produces `usize::MAX`, which will always fail the comparison `new_r < board.len()`. This eliminates the need for casts or additional checks -- bounds are verified with a single comparison per dimension.

## Rust Solution

```rust
use std::collections::HashMap;

#[derive(Default)]
struct TrieNode {
    children: [Option<Box<TrieNode>>; 26],
    word: Option<String>,
}

impl TrieNode {
    fn new() -> Self {
        Self::default()
    }

    fn insert(&mut self, word: String) {
        let mut node = self;
        for b in word.bytes() {
            let idx = (b - b'a') as usize;
            node = node.children[idx].get_or_insert_with(|| Box::new(TrieNode::new()));
        }
        node.word = Some(word);
    }
}
impl Solution {
    pub fn find_words(mut board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        let mut root = TrieNode::new();

        for word in words {
            root.insert(word);
        }

        let rows = board.len();
        let cols = board[0].len();
        let mut result = Vec::new();

        for r in 0..rows {
            for c in 0..cols {
                if let Some(next_node) = &mut root.children[char_to_idx(board[r][c])] {
                    dfs(&mut board, r, c, next_node, &mut result);
                }
            }
        }

        result
    }
}

#[inline(always)]
fn char_to_idx(c: char) -> usize {
    (c as u8 - b'a') as usize
}

fn dfs(
    board: &mut Vec<Vec<char>>,
    r: usize,
    c: usize,
    node: &mut TrieNode,
    result: &mut Vec<String>,
) {
    if let Some(w) = node.word.take() {
        result.push(w);
    }

    let original_char = board[r][c];
    board[r][c] = '#';

    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    for (dr, dc) in directions {
        let new_r = r.wrapping_add(dr as usize);
        let new_c = c.wrapping_add(dc as usize);

        if new_r < board.len() && new_c < board[0].len() {
            let next_char = board[new_r][new_c];
            if next_char != '#' {
                if let Some(next_node) = &mut node.children[char_to_idx(next_char)] {
                    dfs(board, new_r, new_c, next_node, result);
                }
            }
        }
    }
    board[r][c] = original_char;
}
```

The Rust implementation leverages the ownership system elegantly. The Trie is built with `Box<TrieNode>` for child nodes, providing pointer stability on the heap. The `insert` method navigates the Trie with `get_or_insert_with`, creating nodes only when needed. The `dfs` function takes `&mut TrieNode` as a mutable reference, which allows using `node.word.take()` to extract the found word without cloning or auxiliary structures -- `take()` replaces the `Option<String>` with `None` in the node itself, deduplicating results naturally. The fixed-size array `children: [Option<Box<TrieNode>>; 26]` is more efficient than a `HashMap` for the limited lowercase alphabet, since each access is `O(1)` without hashing overhead. The `#[inline(always)]` annotation on `char_to_idx` ensures the character-to-index conversion resolves without function call overhead.

## Conclusion

Word Search II demonstrates how an auxiliary data structure can transform a multi-search problem into a unified traversal. The Trie acts as a real-time prefix filter: every DFS step that doesn't correspond to any viable prefix is discarded immediately, drastically reducing the search space. The combination of in-place marking, destructive extraction with `take()`, and wrapping-based navigation produces a solution that is both efficient and concise -- no auxiliary structures for deduplication, no boolean matrices for marking, and no casts for boundary handling.
