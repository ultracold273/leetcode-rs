use std::rc::Rc;
use std::cell::{Ref, RefCell};
use crate::utils::TreeNode;

fn count_good_nodes(root: Ref<'_, TreeNode>, max: i32) -> i32 {
    let mut count = 0;
    if root.val >= max { count += 1; }
    let new_max = max.max(root.val);
    if let Some(lnode) = root.left.clone() {
        count += count_good_nodes(lnode.borrow(), new_max);
    }
    if let Some(rnode) = root.right.clone() {
        count += count_good_nodes(rnode.borrow(), new_max);
    }
    count
}

fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if root.is_none() {
        0
    } else {
        count_good_nodes(root.unwrap().borrow(), std::i32::MIN)
    }
}