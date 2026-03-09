---
title: "0126 Word Ladder II - EN"
problemUrl: "https://leetcode.com/problems/word-ladder-ii/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["breadth-first-search", "hash-table", "string", "backtracking"]
complexity:
  time: "O(N * M * 26)"
  space: "O(N * M)"
---

# Charting Every Shortcut Between Words

## The Problem
Given a `beginWord`, an `endWord`, and a `wordList`, return all the shortest transformation sequences from `beginWord` to `endWord`, where each step changes exactly one letter and every intermediate word must exist in `wordList`. If no such transformation exists, return an empty list.

## The Hidden Complexity

This problem is the ruthless sibling of Word Ladder I. In the first problem we only need the *length* of the shortest transformation. Here we need *all* sequences that achieve that minimum length. That changes everything.

My first impulse was to run a classic BFS and store full paths in the queue. But storing entire paths at each BFS node is a memory disaster -- each path grows with every level, and the number of paths can be exponential. The key is to separate exploration from reconstruction: first use BFS to map the structure of the shortest-path graph, then use backtracking to traverse it.

## Two Phases: BFS to Discover, Backtracking to Reconstruct

### Phase 1: Layer-by-Layer BFS

The fundamental idea is to perform BFS level by level, not word by word. At each level, we process *all* words in the current layer before advancing. This has a crucial consequence: if a word appears at level 3, we know its distance from `beginWord` is exactly 3. If we encounter it again at level 4, we no longer care -- that won't produce a shorter path.

For each new word we discover, instead of recording "where I came from" as a single parent, we record *all* parents from the current level. If both "hot" and "dot" can reach "dog" at the same level, then "dog" has two parents. That's the information we need to reconstruct multiple paths.

A subtle detail: we remove words from the `word_set` *at the start* of each level, not when we discover them individually. If we removed "dog" upon finding it from "hot," then "dot" would never register it as a child. By removing entire layers at once, we allow all words in the current level to contribute their connections before closing the door.

### Phase 2: Backtracking from the End

Once BFS finds `endWord`, we have a parent map that encodes a DAG (directed acyclic graph) of shortest paths. We reconstruct paths by walking backward from `endWord` to `beginWord` using the parent map, accumulating each path and reversing it upon reaching the origin.

### A Concrete Example

With `beginWord = "hit"`, `endWord = "cog"`, `wordList = ["hot","dot","dog","lot","log","cog"]`:

```
Level 0: {hit}
Level 1: {hot}          parents: hot <- [hit]
Level 2: {dot, lot}     parents: dot <- [hot], lot <- [hot]
Level 3: {dog, log}     parents: dog <- [dot], log <- [dot, lot]
Level 4: {cog}          parents: cog <- [dog, log]
```

Backtracking from "cog" reconstructs:
- cog -> dog -> dot -> hot -> hit
- cog -> log -> dot -> hot -> hit
- cog -> log -> lot -> hot -> hit

Each one gets reversed to give the final left-to-right path.

## Why Remove by Layer, Not Individually?

This is the most delicate point of the algorithm. If we remove each word from the global set the moment we discover it, we prevent other words *at the same level* from discovering it too. But two shortest paths can pass through the same word at the same level. By processing removals in complete layers, we guarantee that all edges in the shortest-path graph get captured in the parent map.

## Rust Solution

```rust
use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn find_ladders(
        begin_word: String,
        end_word: String,
        word_list: Vec<String>,
    ) -> Vec<Vec<String>> {
        let mut word_set: HashSet<String> = word_list.into_iter().collect();

        if !word_set.contains(&end_word) {
            return vec![];
        }

        let mut layer: HashSet<String> = HashSet::new();
        layer.insert(begin_word.clone());

        let mut parents: HashMap<String, Vec<String>> = HashMap::new();

        let mut found = false;

        while !layer.is_empty() && !found {
            for w in &layer {
                word_set.remove(w);
            }

            let mut next_layer: HashSet<String> = HashSet::new();

            for word in &layer {
                let mut chars: Vec<char> = word.chars().collect();

                for i in 0..chars.len() {
                    let old_char = chars[i];

                    for c in 'a'..='z' {
                        if c == old_char {
                            continue;
                        }

                        chars[i] = c;
                        let new_word: String = chars.iter().collect();

                        if word_set.contains(&new_word) {
                            if new_word == end_word {
                                found = true;
                            }

                            next_layer.insert(new_word.clone());

                            parents
                                .entry(new_word)
                                .or_insert(Vec::new())
                                .push(word.clone());
                        }
                    }
                    chars[i] = old_char;
                }
            }
            layer = next_layer;
        }

        let mut result = Vec::new();
        if found {
            let mut current_path = vec![end_word.clone()];
            Self::backtrack(
                &end_word,
                &begin_word,
                &parents,
                &mut current_path,
                &mut result,
            );
        }

        result
    }

    fn backtrack(
        current: &String,
        target: &String,
        parents: &HashMap<String, Vec<String>>,
        path: &mut Vec<String>,
        result: &mut Vec<Vec<String>>,
    ) {
        if current == target {
            let mut full_path = path.clone();
            full_path.reverse();
            result.push(full_path);
            return;
        }

        if let Some(parent_list) = parents.get(current) {
            for parent in parent_list {
                path.push(parent.clone());
                Self::backtrack(parent, target, parents, path, result);
                path.pop();
            }
        }
    }
}
```

The Rust implementation separates the two phases cleanly. The main `while` loop implements the layer-by-layer BFS: at the start of each iteration, all words in the current layer are removed from `word_set`, ensuring they won't be revisited in later levels while still allowing connections within the same level. The `HashMap<String, Vec<String>>` of parents accumulates *all* valid connections along shortest paths. The `found` flag lets the BFS complete the current level before stopping -- this is vital because multiple shortest paths can reach `endWord` from different words at the same level. The `backtrack` function traverses the parent map in reverse, building paths from `endWord` toward `beginWord` and reversing them at the end. The classic `push`/`pop` backtracking pattern reuses the same path vector, avoiding unnecessary allocations.

## Conclusion

Word Ladder II illustrates a fundamental lesson about BFS in graphs: when we need *all* shortest paths and not just one, exploration and reconstruction must live in separate phases. The layer-by-layer BFS builds an implicit DAG of parent-child relationships that respects minimum distances, and the subsequent backtracking exhaustively traverses it. The critical trick -- removing words from the set by complete layers rather than individually -- is what allows us to capture the entirety of optimal paths without sacrificing the guarantee of minimality.
