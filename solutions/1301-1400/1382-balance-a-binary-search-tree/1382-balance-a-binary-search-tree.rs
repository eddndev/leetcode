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
