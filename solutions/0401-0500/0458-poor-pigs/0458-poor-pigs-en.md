---
title: "0458 Poor Pigs - EN"
problemUrl: "https://leetcode.com/problems/poor-pigs/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["math", "combinatorics"]
complexity:
  time: "O(log(buckets) / log(states)), where states = (minutesToTest / minutesToDie) + 1"
  space: "O(1)"
---

# Piglets, Poison, and the Geometry of Information

## The Problem
There are `buckets` buckets of liquid, exactly one of which is poisoned. To figure out which one is poisoned, you can feed pigs the liquid. A pig dies if it drinks the poison, and it takes exactly `minutesToDie` minutes to die. Given a total of `minutesToTest` minutes to conduct the tests, return the minimum number of pigs needed to determine with certainty which bucket is poisoned.

## The Illusion of Linear Search

My first instinct was to think of this as an elimination problem. A pig can test one bucket per round, so with a single round I could eliminate buckets one at a time. But that would potentially require as many pigs as buckets, which is absurd. The key is that pigs are not limited to testing a single bucket: they can drink from multiple buckets simultaneously, and the information they provide by dying (or surviving) is much richer than it first appears.

## The Time Dimension

What transforms this problem is realizing that time is not merely a constraint: it is an additional dimension of information. If I have `minutesToTest / minutesToDie` rounds available, each pig can end up in one of several states at the conclusion of the experiment: died in round 1, died in round 2, ..., died in round R, or survived. That is `R + 1` possible states per pig.

With a single pig and `R + 1` states, I can distinguish among `R + 1` buckets: I give it a different bucket in each round, and the moment it dies (or the fact that it survives) tells me which one was poisoned.

## Multiplying Dimensions

Now comes the conceptual leap. If I have two pigs, each with `R + 1` states, I can arrange the buckets in a grid of `(R+1) x (R+1)`. The first pig identifies the row and the second the column. In round k, the first pig drinks from all buckets in row k, and the second from all buckets in column k. Their combined death/survival patterns identify the exact cell.

This extends naturally to more dimensions. With `P` pigs, I can arrange the buckets in a `P`-dimensional hypercube, each axis of size `(R+1)`. The total capacity is `(R+1)^P`. I just need to find the smallest `P` such that `(R+1)^P >= buckets`.

## The Elegance of the Algorithm

The algorithm is almost trivially simple once the theory is understood:

1. Compute `rounds = minutesToTest / minutesToDie`.
2. Compute `states = rounds + 1` (the possible states per pig).
3. Multiply `states` by itself repeatedly, counting how many pigs are needed until the capacity meets or exceeds the number of buckets.

There is no search, no simulation, no DP. It is pure combinatorial mathematics translated into a three-line loop.

## Walking Through an Example

Consider `buckets = 1000`, `minutesToDie = 15`, `minutesToTest = 60`.

- Available rounds: `60 / 15 = 4`.
- States per pig: `4 + 1 = 5`.
- With 1 pig: capacity = 5. Not enough.
- With 2 pigs: capacity = 25. Not enough.
- With 3 pigs: capacity = 125. Not enough.
- With 4 pigs: capacity = 625. Not enough.
- With 5 pigs: capacity = 3125. More than enough.

Answer: 5 pigs. Each pig represents an axis in a 5-dimensional space with 5 positions per axis. The 3125 cells comfortably cover the 1000 buckets.

## Rust Solution

```rust
impl Solution {
    pub fn poor_pigs(buckets: i32, minutes_to_die: i32, minutes_to_test: i32) -> i32 {
        let rounds = minutes_to_test / minutes_to_die;
        let states = rounds + 1;
        let mut pigs = 0;
        let mut capacity = 1;

        while capacity < buckets {
            capacity *= states;
            pigs += 1;
        }

        pigs
    }
}
```

The solution first computes how many testing rounds are available and from that the number of states per pig. It then enters a loop that multiplies the capacity by `states` on each iteration, incrementing the pig counter. The loop terminates as soon as the capacity meets or exceeds the number of buckets. The operation is O(log(buckets) / log(states)) in time and O(1) in space, which is practically instantaneous for any valid input.

## Conclusion

Poor Pigs is a problem that disguises an information-theoretic argument behind a seemingly whimsical scenario. The key is not in simulating tests or optimizing assignments, but in recognizing that each pig is an axis of information and that time adds states to each axis. The number of buckets that can be distinguished is the product of every pig's states, and finding the minimum number of pigs reduces to a simple power comparison. It is one of those problems where the most elegant solution is also the shortest.
