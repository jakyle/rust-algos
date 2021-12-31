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

use std::{cell::RefCell, rc::Rc};
type Node = Option<Rc<RefCell<TreeNode>>>;

pub fn is_same_tree(p: Node, q: Node) -> bool {
    let mut stack = vec![(p, q)];
    while let Some((a, b)) = stack.pop() {
        match (
            a.as_deref().map(RefCell::borrow_mut),
            b.as_deref().map(RefCell::borrow_mut),
        ) {
            (Some(mut a), Some(mut b)) if a.val == b.val => {
                stack.push((a.left.take(), b.left.take()));
                stack.push((a.right.take(), b.right.take()));
            }
            (Some(_), Some(_)) | (Some(_), None) | (None, Some(_)) => return false,
            (None, None) => (),
        }
    }

    true
}

pub fn is_same_tree_recursive(p: Node, q: Node) -> bool {
    match (
        p.as_deref().map(RefCell::borrow_mut),
        q.as_deref().map(RefCell::borrow_mut),
    ) {
        (Some(mut p), Some(mut q)) if p.val == q.val => {
            is_same_tree_recursive(p.left.take(), q.left.take())
                && is_same_tree_recursive(p.right.take(), q.right.take())
        }
        (None, None) => true,
        _ => false,
    }
}

#[cfg(test)]
mod is_same_tree_tests {
    use super::*;

    fn new_node(val: TreeNode) -> Node {
        Some(Rc::new(RefCell::new(val)))
    }

    #[test]
    fn is_same_tree_test_one() {
        // head
        let mut node_one = TreeNode::new(1);
        let left = TreeNode::new(2);
        let right = TreeNode::new(3);

        node_one.left = new_node(left);
        node_one.right = new_node(right);
        let node_one = new_node(node_one);

        let mut node_two = TreeNode::new(1);
        let left = TreeNode::new(2);
        let right = TreeNode::new(3);

        node_two.left = new_node(left);
        node_two.right = new_node(right);
        let node_two = new_node(node_two);

        assert_eq!(is_same_tree(node_one, node_two), true);
    }
}
