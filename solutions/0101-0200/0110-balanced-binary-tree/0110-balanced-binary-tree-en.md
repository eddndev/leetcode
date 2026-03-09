---
title: "0110 Balanced Binary Tree - EN"
problemUrl: "https://leetcode.com/problems/balanced-binary-tree/"
difficulty: "Easy"
pubDate: "2026-03-08"
tags: ["tree", "depth-first-search", "binary-tree"]
complexity:
  time: "O(n)"
  space: "O(n)"
---

# Balanced Binary Tree

## Problem
Given a binary tree, determine if it is height-balanced. A height-balanced binary tree is one in which, for every node, the difference between the heights of its left and right subtrees does not exceed 1.

## Solution
We use a recursive function that computes the height of each subtree bottom-up. If at any point the height difference between the left and right subtrees exceeds 1, we return `None` to signal the tree is unbalanced and short-circuit the recursion early. If both subtrees are valid, we return the maximum height plus 1.

### Implementation in Rust

```rust
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::cell::RefCell;
use std::cmp;
use std::rc::Rc;
impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::check_height(&root).is_some()
    }

    fn check_height(root: &Option<Rc<RefCell<TreeNode>>>) -> Option<i32> {
        match root {
            None => Some(0),
            Some(node) => {
                let node = node.borrow();

                let left_h = Self::check_height(&node.left)?;
                let righ_h = Self::check_height(&node.right)?;

                if (left_h - righ_h).abs() > 1 {
                    None
                } else {
                    Some(cmp::max(left_h, righ_h) + 1)
                }
            }
        }
    }
}
```
