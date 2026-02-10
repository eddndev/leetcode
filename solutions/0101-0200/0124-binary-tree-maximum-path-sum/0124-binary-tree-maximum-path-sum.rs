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
