use std::fmt::{Display, Formatter};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    pub(crate) fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }

    pub fn build_from_vec(vals: &Vec<i32>) -> Option<Box<ListNode>> {
        vals.iter().rev().fold(None, |b, a| {
            let mut node = ListNode::new(*a);
            node.next = b;
            Some(Box::new(node))
        })
    }
}

impl Display for ListNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut dummy = Some(Box::new(self.clone()));

        while let Some(node) = dummy {
            write!(f, "{} ", node.val).expect("Must write");
            dummy = node.next;
        }

        write!(f, "(None)")
    }
}
