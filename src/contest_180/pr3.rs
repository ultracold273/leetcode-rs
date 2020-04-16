
use std::rc::Rc;
use std::cell::RefCell;
use crate::utils::TreeNode;

fn traverse_inorder(root: Option<&Rc<RefCell<TreeNode>>>, v: &mut Vec<Rc<RefCell<TreeNode>>>) {
    if let Some(node) = root {
        traverse_inorder(node.borrow().left.as_ref(), v);
        v.push(Rc::clone(node));
        traverse_inorder(node.borrow().right.as_ref(), v);
    }
}

fn bst(v: &[Rc<RefCell<TreeNode>>]) -> Option<Rc<RefCell<TreeNode>>> {
    let n = v.len();
    if n == 0 {
        None
    } else {
        let mid = n / 2;
        let left = bst(&v[0..mid]);
        let right = bst(&v[mid+1..n]);
        let mut d = v[mid].borrow_mut();
        d.left = left;
        d.right = right;
        Some(Rc::clone(&v[mid]))
    }
}

fn balance_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    let mut vnode = Vec::new();
    traverse_inorder(root.as_ref(), &mut vnode);

    // for n in vnode {
    //     println!("{:?}", n);
    // }
    bst(&vnode)
    // None
}

#[test]
fn balance_bst_test() {
    let node4 = Rc::new(RefCell::new(TreeNode::new(4)));
    let node3 = Rc::new(RefCell::new(TreeNode::new(3)));
    let node2 = Rc::new(RefCell::new(TreeNode::new(2)));
    let node1 = Rc::new(RefCell::new(TreeNode::new(1)));
    node3.borrow_mut().right = Some(Rc::clone(&node4));
    node2.borrow_mut().right = Some(Rc::clone(&node3));
    node1.borrow_mut().right = Some(Rc::clone(&node2));
    balance_bst(Some(node1));
}