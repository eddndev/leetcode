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
