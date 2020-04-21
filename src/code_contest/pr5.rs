use std::rc::Rc;
use std::cell::RefCell;
use crate::utils::TreeNode;

fn min_time(root: &TreeNode) -> (f64, i32) {
    let (ltbest, ltsum) = if let Some(ref ltree) = root.left {
        let rltree = ltree.borrow();
        min_time(&(*rltree))
    } else { (0f64, 0) };
    let (rtbest, rtsum) = if let Some(ref rtree) = root.right {
        let rrtree = rtree.borrow();
        min_time(&(*rrtree))
    } else { (0f64, 0) };
    let temp = (ltsum as f64 + rtsum as f64) / 2f64;
    (ltbest.max(rtbest).max(temp) + root.val as f64, root.val + ltsum + rtsum)
}

fn minimal_exec_time(root: Option<Rc<RefCell<TreeNode>>>) -> f64 {
    if root.is_none() { return 0f64; }
    let r = root.unwrap();
    let rtnode = r.borrow();
    min_time(&(*rtnode)).0
}