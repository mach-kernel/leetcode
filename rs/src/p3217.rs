use std::any::Any;
use crate::lib::list_node::ListNode;
use std::ops::{Deref, DerefMut};
use std::ptr::null_mut;

// Keep m nodes, drop n nodes, repeat
// m is always at least 1 (so always keep head)

pub fn delete_nodes(head: Option<Box<ListNode>>, m: i32, n: i32) -> Option<Box<ListNode>> {
    let (mut keep, mut drop) = (m, n);

    // Make some raw pointers to head
    let mut head = head.unwrap();
    let head_ptr = &mut head as &mut ListNode as *mut ListNode;

    let mut tail = head_ptr;
    let mut dummy = head_ptr;

    unsafe {
        while let Some(_) = dummy.as_ref() {
            // Tail moves with dummy as we keep nodes
            if (keep > 0) {
                tail = dummy;
            }
            // Tail does not move with dummy as we drop
            else if (keep < 1 && drop > 0) {
                drop -= 1;
            }
            // When both are not pos, we start over
            else {
                keep = m;
                drop = n;
                
                // dummy is currently owned by another ListNode box, so let's clone it
                let mut new_head = Box::new(dummy.as_ref().unwrap().clone());
                dummy = &mut new_head as &mut ListNode as *mut ListNode;
                
                tail.as_mut().unwrap().next = Some(new_head);
                tail = dummy;
            }

            keep -= 1;

            let next = if dummy.as_ref().unwrap().next.is_some() {
                dummy.as_ref().unwrap().next.as_ref().unwrap().as_ref() as *const _ as *mut ListNode
            } else {
                null_mut()
            };

            dummy = next;
        }

        tail.as_mut().unwrap().next = None;
    }

    Some(head)
}

#[cfg(test)]
mod tests {
    use crate::lib::list_node::ListNode;
    use crate::p3217::delete_nodes;

    #[test]
    fn test_delete_nodes() {
        let list1 = ListNode::build_from_vec(&vec![1,2,3,4,5,6,7,8,9,10,11,12,13]);
        let list2 = ListNode::build_from_vec(&vec![1,2,3,4,5,6,7,8,9,10,11]);
        println!("{}", delete_nodes(list1, 2, 3).unwrap());
        println!("{}", delete_nodes(list2, 1, 3).unwrap());
    }
}
