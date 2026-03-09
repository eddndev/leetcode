---
title: "0403 Frog Jump - EN"
problemUrl: "https://leetcode.com/problems/frog-jump/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["dynamic-programming", "hash-map", "hash-set"]
complexity:
  time: "O(N^2) where N is the number of stones"
  space: "O(N^2)"
---

# The Frog That Calculates Before It Leaps

## The Problem
A frog is crossing a river. The river is divided into units and some of the units have stones. The frog can jump on a stone, but it must not land in the water. Given a list of stone positions sorted in ascending order, determine if the frog can cross the river by landing on the last stone. Initially, the frog is on the first stone and assumes the first jump must be 1 unit. If the frog's last jump was `k` units, its next jump must be either `k - 1`, `k`, or `k + 1` units. The frog can only jump in the forward direction.

## The Initial Intuition

At first glance, this looks like a search problem over a state graph, where each state is a combination of current stone and last jump size. I could try a depth-first or breadth-first search, but the number of possible states could explode. What I need is a way to track, for each stone, all the jump sizes with which the frog can reach it.

My first instinct is to use dynamic programming. If for each stone I store the set of possible jumps that brought the frog there, I can propagate forward: from each stone, with each recorded jump `k`, I try to reach positions `stone + k - 1`, `stone + k`, and `stone + k + 1`. If any of those positions corresponds to a real stone, I record the new jump at that destination stone.

## The HashMap as the Backbone

The key to the approach is using a `HashMap` where each stone maps to a `HashSet` of jump sizes. I initialize the map by inserting all stones with empty sets. Then I place a jump of `0` on the first stone as a seed -- the frog starts there without having jumped.

This design has a crucial advantage: checking whether a destination position is a valid stone reduces to an O(1) lookup in the map. I do not need to search linearly or binary-search through the stones array.

## The Forward Propagation

I iterate through the stones in order. For each stone, I clone its set of jumps (necessary in Rust to avoid borrow conflicts) and for each jump `k` in the set, I consider the three possible next steps: `k - 1`, `k`, and `k + 1`. I only propagate if the step is positive -- a jump of `0` or negative makes no sense because the frog only moves forward.

If the destination position `current_stone + step` exists as a key in the map, I insert `step` into that destination stone's set. At the end, the frog can cross the river if and only if the last stone's set of jumps is not empty, meaning at least one valid path reached it.

## Why the Complexity Works

Each stone can have at most O(N) different jumps in its set, since jumps grow in a controlled manner. For each stone, I iterate over its jumps and perform three lookup-and-insert operations on the map. In the worst case, the total number of operations is O(N^2), which is acceptable for the problem's constraints where N can be up to 2000.

## Rust Solution

```rust
use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn can_cross(stones: Vec<i32>) -> bool {
        let n = stones.len();

        let mut dp: HashMap<i32, HashSet<i32>> = HashMap::new();

        for &stone in &stones {
            dp.insert(stone, HashSet::new());
        }

        if let Some(start_set) = dp.get_mut(&stones[0]) {
            start_set.insert(0);
        }

        for i in 0..n {
            let current_stone = stones[i];

            let jumps = dp[&current_stone].clone();

            for &k in &jumps {
                for step in k - 1..=k + 1 {
                    if step > 0 {
                        let next_pos = current_stone + step;

                        if let Some(next_set) = dp.get_mut(&next_pos) {
                            next_set.insert(step);
                        }
                    }
                }
            }
        }

        !dp[&stones[n - 1]].is_empty()
    }
}
```

The `dp` map is built by first inserting all stones as keys with empty sets, ensuring that existence checks are straightforward. The seed `0` on the first stone allows the first jump to generate steps `k - 1 = -1` (discarded by the `step > 0` condition), `k = 0` (also discarded), and `k + 1 = 1` (the only valid initial jump). The `.clone()` on the jump set is necessary because Rust does not allow holding an immutable reference for iteration and a mutable reference for insertion on the same map simultaneously. Each successful insertion into `next_set` indicates the frog can reach that destination stone with that particular step size. The final check verifies whether the last stone was reached by any path.

## Conclusion

Frog Jump is an elegant example of how forward-propagation dynamic programming can solve reachability problems with constrained transitions. The combination of `HashMap` and `HashSet` in Rust provides amortized O(1) lookup and insertion operations, keeping the solution within O(N^2) in both time and space. What might appear to be an exponential search problem is tamed by observing that all that matters at each stone is not how we arrived, but with which jumps we arrived -- and that compact information is all that is needed to decide where the frog can go next.
