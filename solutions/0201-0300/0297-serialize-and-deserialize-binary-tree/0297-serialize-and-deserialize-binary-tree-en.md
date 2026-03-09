---
title: "0297 Serialize and Deserialize Binary Tree - EN"
problemUrl: "https://leetcode.com/problems/serialize-and-deserialize-binary-tree/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["tree", "binary-tree", "design", "string", "dfs", "recursion"]
complexity:
  time: "O(N) where N is the number of nodes in the tree"
  space: "O(N) for storing the serialized string and the recursion stack"
---

# Packing Trees into Strings and Bringing Them Back to Life

## The Problem
Design an algorithm to serialize and deserialize a binary tree. Serialization is the process of converting a data structure into a sequence of bits so that it can be stored or transmitted and reconstructed later. There is no restriction on how the serialization/deserialization algorithm should work, only that a binary tree can be serialized to a string and that this string can be deserialized back to the original tree structure.

## The Tension between Structure and Text

A binary tree is inherently a two-dimensional structure: each node branches left and right, forming a hierarchy that has no obvious linear representation. The challenge of this problem is not simply traversing the tree, but encoding it in such a way that the *exact shape* of the tree -- including where null nodes are -- can be recovered unambiguously from a flat string.

My first impulse was to use a BFS level-order traversal, like the classic LeetCode representation. But on reflection, a preorder (DFS) traversal turns out more elegant: the root appears first, followed recursively by the entire left subtree and then the entire right subtree. The key is to explicitly record null nodes with a sentinel, because without them it would be impossible to distinguish trees with the same sequence of values but different structures.

## The Strategy: Preorder with Sentinels

### Serialization

The idea is straightforward: I traverse the tree in preorder (root, left, right). For each existing node, I write its value followed by a space. For each null position, I write `#` followed by a space. The space acts as a universal delimiter.

Consider the tree:

```
    1
   / \
  2   3
     / \
    4   5
```

The preorder traversal with sentinels produces: `1 2 # # 3 4 # # 5 # #`

Each node contributes exactly one token, and each null position contributes a `#`. This representation is *complete*: there is no possible ambiguity. Two different trees will always produce different strings, and a valid string always reconstructs exactly one tree.

### Deserialization

This is where the magic of preorder shines. I create an iterator over the whitespace-separated tokens and consume them one by one. For each token:

1. If it is `#`, I return `None` -- this position is a null node.
2. If it is a number, I create a `TreeNode` with that value, and recursively deserialize its left child and then its right child.

The beauty of this approach is that the iterator maintains state implicitly. I need no indices, no position calculations. Each recursive call simply consumes the next available token, and the preorder ordering guarantees that the tokens align perfectly with the tree structure.

### Reconstructing the Example

With the string `1 2 # # 3 4 # # 5 # #`:

```
Consume "1" -> create node(1)
  Left: consume "2" -> create node(2)
    Left: consume "#" -> None
    Right: consume "#" -> None
  Right: consume "3" -> create node(3)
    Left: consume "4" -> create node(4)
      Left: consume "#" -> None
      Right: consume "#" -> None
    Right: consume "5" -> create node(5)
      Left: consume "#" -> None
      Right: consume "#" -> None
```

The original tree is perfectly reconstructed, node by node, without needing to explicitly compute parent-child relationships.

## Why Preorder and Not Another Traversal

An inorder traversal would not work here because the root's position is not predictable without additional information. A postorder traversal could work, but deserialization would be less intuitive -- I would have to read the tokens in reverse or use an explicit stack. Preorder is the natural choice because the first token is always the root, which enables a top-down reconstruction that aligns perfectly with recursion.

## Rust Solution

```rust
use std::cell::RefCell;
use std::rc::Rc;

struct Codec {}

impl Codec {
    fn new() -> Self {
        Codec {}
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut result = String::new();
        self.r_serialize(root, &mut result);
        result
    }

    fn r_serialize(&self, root: Option<Rc<RefCell<TreeNode>>>, out: &mut String) {
        match root {
            Some(node) => {
                let n = node.borrow();
                out.push_str(&n.val.to_string());
                out.push(' ');

                self.r_serialize(n.left.clone(), out);
                self.r_serialize(n.right.clone(), out);
            }
            None => {
                out.push_str("# ");
            }
        }
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        let mut iter = data.split_whitespace();
        self.r_deserialize(&mut iter)
    }

    fn r_deserialize(&self, iter: &mut std::str::SplitWhitespace) -> Option<Rc<RefCell<TreeNode>>> {
        match iter.next() {
            Some(val_str) => {
                if val_str == "#" {
                    return None;
                }

                if let Ok(val) = val_str.parse::<i32>() {
                    let mut node = TreeNode::new(val);
                    node.left = self.r_deserialize(iter);
                    node.right = self.r_deserialize(iter);

                    Some(Rc::new(RefCell::new(node)))
                } else {
                    None
                }
            }
            None => None,
        }
    }
}
```

The Rust implementation uses `Rc<RefCell<TreeNode>>` for the shared ownership and interior mutability that LeetCode's tree structure requires. In serialization, `r_serialize` receives a mutable reference to a `String` to avoid unnecessary allocations -- each node simply appends its representation to the existing buffer. The `borrow()` call on the `RefCell` obtains an immutable reference to the node, and `n.left.clone()` clones the `Rc` (incrementing the reference count, not copying the node) to pass it into the recursive call. In deserialization, `split_whitespace()` produces a lazy iterator that is passed by mutable reference through the recursion, allowing each call to consume exactly one token without needing an external index. The `if let Ok(val)` defensively handles a non-numeric token, though by construction the serialized string will only contain integers and the `#` sentinel. The `match` on `iter.next()` naturally handles iterator exhaustion, returning `None` if the string runs out prematurely.

## Conclusion

Serialize and Deserialize Binary Tree is a design problem that appears open-ended but has an elegantly constrained solution. Preorder traversal with null sentinels captures the complete tree structure in a linear string, and deserialization reconstructs it by sequentially consuming tokens with recursion. No indices needed, no queues needed, no position calculations needed. Just the confidence that preorder ordering and sentinels contain all the structural information required. The symmetry between serialization and deserialization -- both recursive functions that process root, left, right in the same order -- is what makes this solution both correct by construction and easy to reason about.
