// This problem is very interesting, TLE at first
use std::rc::Rc;
use std::cell::RefCell;
use crate::utils::{TreeNode, ListNode};

fn dfs(h: Option<&Box<ListNode>>, r: Option<&Rc<RefCell<TreeNode>>>) -> bool {
    if h.is_none() { return true; }
    if r.is_none() { return false; }
    let node = h.unwrap();
    let tnode = r.unwrap();
    if node.val != tnode.borrow().val {
        // here we need to directly return
        false
    } else {
        dfs(node.next.as_ref(), tnode.borrow().left.as_ref()) ||
        dfs(node.next.as_ref(), tnode.borrow().right.as_ref())
    }
}

fn is_sub_path_ref(head: Option<&Box<ListNode>>, root: Option<&Rc<RefCell<TreeNode>>>) -> bool {
    if root.is_none() { return false; }
    dfs(head, root) || is_sub_path_ref(head, root.unwrap().borrow().left.as_ref())
    || is_sub_path_ref(head, root.unwrap().borrow().right.as_ref())
}

fn is_sub_path(head: Option<Box<ListNode>>, root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    is_sub_path_ref(head.as_ref(), root.as_ref())
}