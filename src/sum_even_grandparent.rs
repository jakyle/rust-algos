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

pub fn sum_even_grandparent(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    #[derive(PartialEq, Eq, Clone)]
    pub enum Relationship {
        Child,
        GrandChild,
    }

    use std::collections::VecDeque;
    type Node = Rc<RefCell<TreeNode>>;

    let mut dq: VecDeque<(Node, Vec<Relationship>)> = VecDeque::new();

    if let Some(node) = root {
        dq.push_back((node, vec![]));
    }

    let mut sum = 0;
    while !dq.is_empty() {
        for _ in 0..dq.len() {
            if let Some((node, relationships)) = dq.pop_front() {
                let mut new_relationships = vec![];

                if relationships.contains(&Relationship::GrandChild) {
                    let val = node.borrow().val;
                    sum += val;
                }

                if relationships.contains(&Relationship::Child) {
                    new_relationships.push(Relationship::GrandChild);
                }

                if node.borrow().val % 2 == 0 {
                    new_relationships.push(Relationship::Child);
                }

                if let Some(left) = node.borrow_mut().left.take() {
                    dq.push_back((left, new_relationships.clone()));
                }

                if let Some(right) = node.borrow_mut().right.take() {
                    dq.push_back((right, new_relationships));
                }
            }
        }
    }

    sum
}

pub fn sum_even_grandparent_dfs(mut root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    type Node = Rc<RefCell<TreeNode>>;

    fn dfs(node: &mut Node, parent: bool, grand_parent: bool, count: &mut i32) {
        let mut node = node.borrow_mut();

        if grand_parent {
            *count += node.val;
        }

        let is_parent = node.val % 2 == 0;

        if let Some(left) = &mut node.left {
            dfs(left, is_parent, parent, count);
        }

        if let Some(right) = &mut node.right {
            dfs(right, is_parent, parent, count);
        }
    }

    if let Some(node) = &mut root {
        let mut sum = 0;
        dfs(node, false, false, &mut sum);
        sum
    } else {
        0
    }
}
