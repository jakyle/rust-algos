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

pub fn deepest_leaves_sum_bfs(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    use std::collections::VecDeque;
    type Node = Rc<RefCell<TreeNode>>;

    let mut dq = VecDeque::new();

    if let Some(node) = root {
        dq.push_back(node);
    }

    let mut sum = 0;
    while !dq.is_empty() {
        sum = 0;

        for _ in 0..dq.len() {
            if let Some(node) = dq.pop_front() {
                sum += node.borrow().val;

                if let Some(left) = node.borrow_mut().left.take() {
                    dq.push_back(left);
                }

                if let Some(right) = node.borrow_mut().right.take() {
                    dq.push_back(right);
                }
            }
        }
    }

    sum
}

use std::collections::HashMap;

pub fn deepest_leaves_sum_dfs(mut root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn count(
        tree: &mut Rc<RefCell<TreeNode>>,
        mut sum: &mut i32,
        depth: i32,
        mut best_depth: &mut i32,
    ) {
        let mut node = tree.borrow_mut();

        if depth > *best_depth {
            *sum = 0;
            *best_depth = depth;
        }

        if let Some(left) = &mut node.left {
            count(left, &mut sum, depth + 1, best_depth);
        }
        if let Some(right) = &mut node.right {
            count(right, &mut sum, depth + 1, best_depth);
        }

        if depth == *best_depth {
            *sum += node.val;
        }
    }

    let mut sum = 0;

    if let Some(tree) = &mut root {
        count(tree, &mut sum, 0, &mut 0);
    }

    sum
}
