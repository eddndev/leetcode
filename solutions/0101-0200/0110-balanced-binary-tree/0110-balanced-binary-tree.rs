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
