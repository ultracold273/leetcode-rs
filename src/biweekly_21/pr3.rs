use std::rc::Rc;
use std::cell::RefCell;
use crate::utils::TreeNode;

fn postorder_traverse(root: &TreeNode, max: &mut i32) -> (i32, i32) {
    let lbst = if let Some(ref lnode) = root.left {
        let ltnode = lnode.borrow();
        postorder_traverse(&(*ltnode), max)
    } else { (0, 0) };
    let rbst = if let Some(ref rnode) = root.right {
        let rtnode = rnode.borrow();
        postorder_traverse(&(*rtnode), max)
    } else { (0, 0) };

    *max = (*max).max(lbst.1 + 1).max(rbst.0 + 1);
    (lbst.1 + 1, rbst.0 + 1)
}

fn longest_zig_zag(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if root.is_none() { return 0; }
    let mut longest = 0;
    postorder_traverse(&(*root.unwrap().borrow()), &mut longest);
    longest - 1
}