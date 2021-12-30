use std::{cell::RefCell, rc::Rc, borrow::BorrowMut, sync::WaitTimeoutResult};

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

type Node = Option<Rc<RefCell<TreeNode>>>;

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }


    pub fn new_node(val: TreeNode) -> Node {
        Some(Rc::new(RefCell::new(val)))
    }
}

type NodeRef = Rc<RefCell<TreeNode>>;

pub fn merge_tree_iter(a: Option<NodeRef>, b: Option<NodeRef>) -> Option<NodeRef> {
    let result = a.as_ref().map(|r| Rc::clone(r));
    let mut stack = vec![(a, b)];

    while let Some((a, b)) = stack.pop() {
        if let (Some(mut a), Some(b)) = (
            a.as_deref().map(|x| RefCell::borrow_mut(x)),
            b.as_deref().map(RefCell::borrow)                
        ) {
            a.val += b.val;

            if let Some(b_left) = &b.left {
                let b_left = Rc::clone(b_left);
                if let Some(a_left) = &a.left {
                    stack.push((Some(Rc::clone(&a_left)), Some(b_left)))
                } else {
                    a.left = Some(b_left);
                }
            }

            if let Some(b_right) = &b.right {
                let b_right = Rc::clone(b_right);
                if let Some(a_right) = &a.right {
                    stack.push((Some(Rc::clone(a_right)), Some(b_right)))
                } else {
                    a.right = Some(b_right);
                }
            }
        }
    }

    result
}


pub fn merge_trees(a: Option<NodeRef>, b: Option<NodeRef>) -> Option<NodeRef> {
    if let (Some(mut a), Some(mut b)) = (
        a.as_deref().map(RefCell::borrow_mut),
        b.as_deref().map(RefCell::borrow_mut),
    ) {
        a.val += b.val;
        a.left = merge_trees(a.left.take(), b.left.take());
        a.right = merge_trees(a.right.take(), b.right.take());
    }
    a.or(b)
}

#[cfg(test)]
mod merge_trees_tests {
    use super::*;


    #[test]
    fn merge_trees_test_one() {
        // head
        let mut node_one = TreeNode::new(1);
        let mut left = TreeNode::new(3);
        let left_left = TreeNode::new(5);
        let right = TreeNode::new(2);

        left.left = TreeNode::new_node(left_left);
        node_one.left = TreeNode::new_node(left);
        node_one.right = TreeNode::new_node(right);
        let node_one = TreeNode::new_node(node_one);


        let mut node_two = TreeNode::new(2);
        let mut left = TreeNode::new(1);
        let left_right = TreeNode::new(4);
        let mut right = TreeNode::new(3);
        let right_right = TreeNode::new(7);

        left.right = TreeNode::new_node(left_right);
        right.right = TreeNode::new_node(right_right);
        node_two.left = TreeNode::new_node(left);
        node_two.right = TreeNode::new_node(right);
        let node_two = TreeNode::new_node(node_two);


        let mut result_node = TreeNode::new(3);
        let mut left = TreeNode::new(4);
        let left_left = TreeNode::new(5);
        let left_right = TreeNode::new(4);
        let mut right = TreeNode::new(5);
        let right_right = TreeNode::new(7);

        left.left = TreeNode::new_node(left_left);
        left.right = TreeNode::new_node(left_right);
        right.right = TreeNode::new_node(right_right);
        result_node.left = TreeNode::new_node(left);
        result_node.right = TreeNode::new_node(right);
        let result_node = TreeNode::new_node(result_node);

        assert_eq!(merge_tree_iter(node_one, node_two), result_node);
    }
}
