// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }
}

fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut v1 = Vec::new();
    let mut v2 = Vec::new();
    let mut l1p = &l1;
    let mut l2p = &l2;
    while !l1p.is_none() {  let v = l1p.as_ref().unwrap(); l1p = &v.next; v1.push(v); }
    while !l2p.is_none() {  let v = l2p.as_ref().unwrap(); l2p = &v.next; v2.push(v); }
    let mut i = v1.len() as i32 - 1;
    let mut j = v2.len() as i32 - 1;
    let mut carry = 0;
    let mut next = None;
    while i >= 0 && j >= 0 {
        let mut nv = v1[i as usize].val + v2[j as usize].val + carry;
        if nv >= 10 { carry = 1; nv = nv % 10; } else { carry = 0; }
        i -= 1; j -= 1;
        let mut node = Box::new(ListNode::new(nv));
        node.next = next;
        next = Some(node);
    }
    while i >= 0 {
        let val = if carry != 0 { v1[i as usize].val + carry } else { v1[i as usize].val };
        carry = if val > 9 { 1 } else { 0 };
        let mut node = Box::new(ListNode::new(val % 10));
        node.next = next;
        next = Some(node);
        i -= 1;
    }
    while j >= 0 {
        let val = if carry != 0 { v2[j as usize].val + carry } else { v2[j as usize].val };
        carry = if val > 9 { 1 } else { 0 };
        let mut node = Box::new(ListNode::new(val % 10));
        node.next = next;
        next = Some(node);
        j -= 1;
    }
    if carry != 0 {
        let mut node = Box::new(ListNode::new(1));
        node.next = next;
        next = Some(node);
    }
    next
}