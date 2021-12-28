#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
use std::cell::RefCell;
use std::rc::Rc;

pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    type Node = Rc<RefCell<TreeNode>>;

    fn f(left: Option<&Node>, right: Option<&Node>) -> bool {
        match (left, right) {
            (None, None) => true,
            (Some(l), Some(r)) => {
                let left_node = l.borrow();
                let right_node = r.borrow();

                left_node.val == right_node.val
                    && f(left_node.left.as_ref(), right_node.right.as_ref())
                    && f(left_node.right.as_ref(), right_node.left.as_ref())
            }
            _ => false,
        }
    }

    match root {
        None => true,
        Some(n) => {
            let n = n.borrow();
            f(n.left.as_ref(), n.right.as_ref())
        }
    }
}
