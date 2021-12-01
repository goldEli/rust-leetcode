#[allow(dead_code)]
// pub struct Solution {}

/*
 * @lc app=leetcode.cn id=94 lang=rust
 *
 * [94] 二叉树的中序遍历
 */

// @lc code=start
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
use crate::util::tree::{to_tree, TreeNode};
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = Vec::new();
        // Solution::inorder_traverse(root.as_ref(), &mut (|v| res.push(v)));
        res
    }

    // fn inorder_traverse<F: FnMut(i32)>(root: Option<&Rc<RefCell<TreeNode>>>, consumer: &mut F) {
    //     if let Some(node) = root {
    //         Solution::inorder_traverse(node.borrow().left.as_ref(), consumer);
    //         consumer(root.as_ref().unwrap().borrow().val);
    //         Solution::inorder_traverse(node.borrow().right.as_ref(), consumer);
    //     }
    // }
}
// @lc code=end

