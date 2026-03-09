---
title: "1382 Balance a Binary Search Tree - EN"
problemUrl: "https://leetcode.com/problems/balance-a-binary-search-tree/"
difficulty: "Medium"
pubDate: "2026-03-08"
tags: ["binary-search-tree", "divide-and-conquer", "tree", "greedy", "depth-first-search"]
complexity:
  time: "O(n)"
  space: "O(n)"
---

# Balance a Binary Search Tree: Flatten and Rebuild

## The Problem
Given the root of a binary search tree (BST), we need to return a **balanced** binary search tree with the same node values. A BST is balanced if the depth of the two subtrees of every node never differs by more than 1. If there is more than one valid answer, any of them is accepted.

One might initially think about rotations like those used in AVL or red-black trees. But there is a much more direct path that leverages a fundamental property of BSTs.

## The Intuition: Inorder Traversal Is Already Sorted

The key property of a BST is that its inorder traversal (left, root, right) produces values in ascending order. This means that if we flatten the tree into an array using an inorder walk, we get a sorted sequence of all values.

Once we have that sorted sequence, the problem transforms into something we already know well: **building a balanced BST from a sorted array**. The way to do this is exactly the same idea behind binary search: pick the middle element as the root, then recursively build the left subtree from the left half and the right subtree from the right half.

By always choosing the midpoint, we guarantee that both subtrees have roughly the same number of nodes, which produces a tree with the minimum possible height.

## The Algorithm
1. **Inorder traversal**: Walk the original BST in order (left, node, right) and store all values in a vector.
2. **Build the balanced tree**: Use the sorted array to construct a new BST. At each step, pick the middle element as the root of the current subtree, and recursively apply the same process to the left and right halves.

The recursion terminates when `start > end`, meaning there are no elements for that subtree and we return `None`.

## Rust Implementation

In Rust, trees with shared ownership require `Rc<RefCell<TreeNode>>`. The inorder traversal fills a `Vec<i32>` with the sorted values. The `build` function uses `i32` indices for the bounds, which lets `start > end` serve as the natural base case (for example, when `end` is -1 at the beginning of a range).

```rust
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn balance_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut vals = Vec::new();
        Self::inorder(&root, &mut vals);
        Self::build(&vals, 0, vals.len() as i32 - 1)
    }

    fn inorder(node: &Option<Rc<RefCell<TreeNode>>>, vals: &mut Vec<i32>) {
        if let Some(n) = node {
            let n = n.borrow();
            Self::inorder(&n.left, vals);
            vals.push(n.val);
            Self::inorder(&n.right, vals);
        }
    }

    fn build(vals: &Vec<i32>, start: i32, end: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if start > end {
            return None;
        }
        let mid = (start + end) / 2;
        let node = Rc::new(RefCell::new(TreeNode::new(vals[mid as usize])));

        node.borrow_mut().left = Self::build(vals, start, mid - 1);
        node.borrow_mut().right = Self::build(vals, mid + 1, end);

        Some(node)
    }
}
```

## Conclusion
Time complexity is O(n) where n is the number of nodes: the inorder traversal visits each node once, and building the balanced tree also processes each value once. Space is O(n) for the values vector and the recursion stack. This problem is a reminder that sometimes the best way to fix a structure is not to patch it piece by piece, but to deconstruct it down to its essence (the sorted values) and rebuild it from scratch with the right shape.
