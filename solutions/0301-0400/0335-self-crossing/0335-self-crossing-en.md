---
title: "0335 Self Crossing - EN"
problemUrl: "https://leetcode.com/problems/self-crossing/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["geometry", "math", "array"]
complexity:
  time: "O(n) where n is the length of the distance array"
  space: "O(1)"
---

# The Spiral That Bites Its Own Tail

## The Problem
You are given an array of integers `distance`. Starting at the origin on a 2D plane, you move `distance[0]` meters north, then `distance[1]` meters west, then `distance[2]` meters south, then `distance[3]` meters east, and so on, cycling through directions. Return `true` if your path crosses itself at any point, and `false` otherwise.

## The Initial Intuition

When I first encountered this problem, I imagined tracing a spiral on graph paper. You walk north, turn left to go west, turn left again to go south, and so on -- always turning left. If the spiral keeps expanding outward, the path never crosses itself. If it shrinks consistently inward, the path also stays clean. The crossing happens when the spiral transitions from expanding to shrinking, or when the segments become just the right length to touch or overlap a previous edge.

The brute-force approach of tracking every point visited and checking for intersections would be expensive. But the constrained nature of the movement -- always turning left in a fixed cycle of four directions -- means the path's geometry is highly structured. I only need to check a small window of recent segments to detect a crossing.

## The Three Crossing Patterns

After sketching a few examples, I realized that a crossing at step `i` can only happen in one of three distinct geometric configurations. Each involves the current segment intersecting one of the recent previous segments.

**Pattern 1 -- Fourth edge crosses the first:** The current segment `i` crosses segment `i-2` when the current move is at least as long as the move two steps ago, AND the move one step ago is at most the move three steps ago. This is the classic case where the spiral tries to expand after shrinking. The fourth edge reaches far enough to cross back over the first.

**Pattern 2 -- Fifth edge lands exactly on the first:** Segment `i` meets segment `i-4` when the move one step ago is exactly equal to the move three steps ago (the parallel edges have the same length), AND the current move plus the move four steps ago is enough to reach the edge two steps ago. This creates a situation where the path folds back and the fifth edge exactly touches the first.

**Pattern 3 -- Sixth edge crosses the first:** This is the most subtle case. Segment `i` crosses segment `i-4` when the spiral partially shrinks but then a later segment reaches back. It requires five conditions to align: the move one step ago is shorter than three steps ago, the move one step ago plus five steps ago reaches at least as far as three steps ago, the move two steps ago exceeds four steps ago, and the current move plus four steps ago reaches at least as far as two steps ago.

## Why Only These Three?

The beauty of this approach is completeness. Because the path always turns left and each segment runs in one of four cardinal directions, any crossing must involve the current segment hitting one of the last few segments. Segments farther back are geometrically unreachable by the current segment -- the intervening turns guarantee they're too far away. So checking these three patterns at each step is both necessary and sufficient.

## Walking Through the Logic

For each index `i` starting from 3, I check pattern 1 first since it only needs four segments. If `i >= 4`, I additionally check pattern 2. If `i >= 5`, I check pattern 3. The moment any pattern matches, I return `true`. If I exhaust the entire array without a match, the path never crosses itself.

The conditions use only comparisons and additions on the distance values -- no coordinate tracking, no set lookups, no geometric intersection calculations. The entire solution runs in a single linear pass with constant extra space.

## Rust Solution

```rust
impl Solution {
    pub fn is_self_crossing(distance: Vec<i32>) -> bool {
        let n = distance.len();

        if n <= 3 {
            return false;
        }

        for i in 3..n {
            if distance[i] >= distance[i - 2] && distance[i - 1] <= distance[i - 3] {
                return true;
            }

            if i >= 4 {
                if distance[i - 1] == distance[i - 3]
                    && distance[i] + distance[i - 4] >= distance[i - 2]
                {
                    return true;
                }
            }

            if i >= 5 {
                if distance[i - 1] <= distance[i - 3]
                    && distance[i - 1] + distance[i - 5] >= distance[i - 3]
                    && distance[i - 2] > distance[i - 4]
                    && distance[i] + distance[i - 4] >= distance[i - 2]
                {
                    return true;
                }
            }
        }

        false
    }
}
```

The implementation begins with an early return for arrays of three or fewer elements, since four segments are the minimum required for a crossing. The main loop iterates from index 3, checking each pattern in order. Pattern 1 compares `distance[i]` against `distance[i-2]` and `distance[i-1]` against `distance[i-3]` to detect when the spiral reversal causes a direct intersection. Pattern 2 kicks in at index 4 and catches the exact-overlap case where parallel edges have identical lengths. Pattern 3, available from index 5 onward, handles the trickiest scenario where a partial contraction followed by a partial expansion causes the sixth edge to clip the first.

The solution's elegance lies in reducing a geometric intersection problem to a handful of arithmetic comparisons. No coordinates are ever computed -- the relative distances between parallel edges contain all the information needed to detect crossings.

## Conclusion

Self Crossing is a problem that rewards geometric reasoning over brute force. What initially appears to require tracking coordinates and computing intersections reduces to three simple patterns involving only the last few distances. The spiral's constrained left-turn structure means crossings can only occur in predictable configurations, and recognizing these patterns transforms a potentially quadratic simulation into an `O(n)` single-pass solution with `O(1)` space. The hardest part isn't the code -- it's convincing yourself that three patterns are truly exhaustive, which requires careful diagramming of how a left-turning spiral can fold back on itself.
