use std::rc::Rc;
use std::cell::RefCell;
use crate::utils::TreeNode;

fn dfs(root: &TreeNode, depth: i32, res: &mut Vec<i32>) {
    if depth as usize > res.len() {
        res.push(root.val);
    }

    if let Some(ref rtcell) = root.right {
        let rtnode = rtcell.borrow();
        dfs(&(*rtnode), depth + 1, res);
    }

    if let Some(ref ltcell) = root.left {
        let ltnode = ltcell.borrow();
        dfs(&(*ltnode), depth + 1, res);
    }
}

fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut res = Vec::new();
    if let Some(ref rtcell) = root {
        let rtnode = rtcell.borrow();
        dfs(&(*rtnode), 1, &mut res);
    }
    res
}