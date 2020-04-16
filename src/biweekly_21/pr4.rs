use std::rc::Rc;
use std::cell::RefCell;
use crate::utils::TreeNode;

fn postorder_traverse(root: &TreeNode, map: &mut i32) -> (bool, i32, i32) {
    let lbst = if let Some(ref lnode) = root.left {
        let ltnode = lnode.borrow();
        postorder_traverse(&(*ltnode), map)
    } else { (true, std::i32::MIN, 0) };
    let rbst = if let Some(ref rnode) = root.right {
        let rtnode = rnode.borrow();
        postorder_traverse(&(*rtnode), map)
    } else { (true, std::i32::MAX, 0) };
    
    if lbst.0 && rbst.0 && (root.val > lbst.1) && (root.val < rbst.1) {
        let sum = root.val + lbst.2 + rbst.2;
        *map = (*map).max(sum);
        (true, root.val, sum)
    } else {
        (false, 0, 0)
    }
}

fn max_sum_bst(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if root.is_none() { return 0; }
    let mut sum = 0;
    postorder_traverse(&(*root.unwrap().borrow()), &mut sum);
    sum
}