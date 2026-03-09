---
title: "0110 Balanced Binary Tree - ES"
problemUrl: "https://leetcode.com/problems/balanced-binary-tree/"
difficulty: "Easy"
pubDate: "2026-03-08"
tags: ["tree", "depth-first-search", "binary-tree"]
complexity:
  time: "O(n)"
  space: "O(n)"
---

# Balanced Binary Tree

## Problema
Dado un árbol binario, determinar si está balanceado en altura. Un árbol binario balanceado en altura es aquel en el que, para cada nodo, la diferencia entre las alturas de sus subárboles izquierdo y derecho no excede 1.

## Solución
Usamos una función recursiva que calcula la altura de cada subárbol de abajo hacia arriba. Si en algún momento la diferencia de alturas entre los subárboles izquierdo y derecho supera 1, retornamos `None` para indicar que el árbol no está balanceado y cortamos la recursión tempranamente. Si ambos subárboles son válidos, retornamos la altura máxima más 1.

### Implementación en Rust

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
