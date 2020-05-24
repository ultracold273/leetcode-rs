
use crate::utils::TreeNode;
use std::rc::Rc;
use std::cell::{Ref, RefCell};
use std::collections::HashMap;

fn exist_palindrome(nums: &Vec<i32>) -> bool {
    let n = nums.len();
    let mut map = HashMap::new();
    for n in nums.iter() {
        let ptr = map.entry(*n).or_insert(0);
        *ptr += 1;
    }
    let mut odd_count = 0;
    for (_, cnt) in map.iter() {
        if cnt % 2 != 0 { odd_count += 1;}
    }
    (nums.len() % 2 == 0 && odd_count == 0) || (nums.len() % 2 == 1 && odd_count == 1)
}

fn traverse(root: Ref<'_, TreeNode>, paths: &mut Vec<i32>) -> i32 {
    paths.push(root.val);
    let mut count = 0;
    if root.left.is_none() && root.right.is_none() {
        if exist_palindrome(paths) {
            count += 1;
        }
    } else {
        if let Some(lnode) = root.left.clone() {
            count += traverse(lnode.borrow(), paths);
        }
        if let Some(rnode) = root.right.clone() {
            count += traverse(rnode.borrow(), paths);
        }
    }
    paths.pop();
    count
}

fn pseudo_palindromic_paths (root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut res = 0;
    let mut paths = Vec::new();
    if let Some(root_node) = root {
        res += traverse(root_node.borrow(), &mut paths);
    }
    res
}