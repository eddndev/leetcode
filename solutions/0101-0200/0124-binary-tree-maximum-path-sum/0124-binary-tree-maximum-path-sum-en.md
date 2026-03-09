---
title: "0124 Binary Tree Maximum Path Sum - EN"
problemUrl: "https://leetcode.com/problems/binary-tree-maximum-path-sum/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["binary-tree", "depth-first-search", "dynamic-programming"]
complexity:
  time: "O(N)"
  space: "O(H)"
---

# The Golden Trail Hidden in the Tree

## The Problem
Given the `root` of a binary tree, return the maximum path sum of any **non-empty** path in the tree. A path is defined as a sequence of nodes where each pair of adjacent nodes has an edge connecting them. A node can only appear in the path at most once. The path does not need to pass through the root.

## The Subtle Trap

At first glance, one might think this is solved by finding the longest path or the path from root to some leaf. But the problem is far more permissive and far more treacherous: the path can start and end at *any* node in the tree, and it can go "upward" through a node's parent. That means the optimal path could be an entire subtree where one node acts as the "roof" connecting its left branch to its right branch.

My first instinct was to think about enumerating all possible paths, but that's exponential. The key observation is that in a binary tree, any path has exactly one node that is the "highest point" -- the common ancestor of the path's two endpoints. That node is where the path "bends." If we think from the perspective of each node as a potential turning point, the problem decomposes cleanly.

## The DFS Duality

The strategy is built on a post-order DFS where each recursive call carries **two simultaneous responsibilities**:

1. **Update the global maximum:** Treating the current node as the "roof" of the path, the best sum is `node.val + left_gain + right_gain`, where each gain is the maximum between the corresponding subtree's contribution and zero (because we can always choose not to take a negative branch).

2. **Report upward:** When the node returns a value to its parent, it can only offer *one* of its two branches, not both. A path cannot fork -- if the parent is going to use this node, the path must continue in a straight line. So we return `node.val + max(left_gain, right_gain)`.

This separation is the heart of the solution. The global maximum considers the node as the apex of the complete path (using both branches). The return value treats it as a pass-through node (using only the better branch). Without this distinction, it would be impossible to capture paths that "bend" at some intermediate node.

### A Concrete Example

Consider the tree `[-10, 9, 20, null, null, 15, 7]`:

```
    -10
    / \
   9   20
      / \
     15   7
```

- At node `9` (leaf): left gain = 0, right gain = 0. Local path: 9. Returns 9.
- At node `15` (leaf): local path: 15. Returns 15.
- At node `7` (leaf): local path: 7. Returns 7.
- At node `20`: left gain = 15, right gain = 7. Local path: 20 + 15 + 7 = 42. Returns 20 + max(15, 7) = 35.
- At node `-10`: left gain = 9, right gain = 35. Local path: -10 + 9 + 35 = 34. But global max is already 42.

The answer is **42**: the path `15 -> 20 -> 7`, where node `20` is the turning point. Notice that this path never passes through the root.

### Why `max(..., 0)`?

Applying `max(gain, 0)` to child contributions is equivalent to saying "if a subtree has a negative sum, we simply don't include it in the path." This elegantly handles nodes with negative values: the optimal path might be a single positive node surrounded by negative ones.

## Rust Solution

```rust
use std::cell::RefCell;
use std::cmp;
use std::rc::Rc;

impl Solution {
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut global_max = i32::MIN;

        Self::dfs(&root, &mut global_max);

        global_max
    }

    fn dfs(node_opt: &Option<Rc<RefCell<TreeNode>>>, global_max: &mut i32) -> i32 {
        if let Some(node_rc) = node_opt {
            let node = node_rc.borrow();

            let left_gain = cmp::max(Self::dfs(&node.left, global_max), 0);
            let right_gain = cmp::max(Self::dfs(&node.right, global_max), 0);

            let current_path_sum = node.val + left_gain + right_gain;
            *global_max = cmp::max(*global_max, current_path_sum);

            node.val + cmp::max(left_gain, right_gain)
        } else {
            0
        }
    }
}
```

The Rust implementation captures the duality with crystal clarity. The `global_max` is passed as a mutable reference throughout the entire recursion, updating every time a node discovers an "arch-shaped" path better than the current best. The return value, by contrast, only offers the best single branch upward to the parent. Initializing `global_max` with `i32::MIN` is crucial: all nodes could have negative values, and we need the first visited node to always update the maximum. The `if let Some` is Rust's idiomatic way of handling the null-node option, returning 0 when the subtree doesn't exist -- a clean base case that integrates naturally with the `max(..., 0)` logic.

## Conclusion

This problem reveals a powerful idea: sometimes the recursive function needs to do two things at once -- report a value upward for the parent to use, and simultaneously compute something broader that gets recorded in a global state. That tension between "what I return" and "what I compute" is what makes this problem Hard. Once you internalize the distinction between the node as the path's apex and the node as a pass-through point, the solution flows naturally, and the entire tree gets processed in a single linear traversal with space proportional to the height.
