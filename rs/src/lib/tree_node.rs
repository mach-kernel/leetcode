use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None
        }
    }

    pub fn from_vec(vals: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::build_from_vec(&vals, 0)
    }

    fn build_from_vec(vals: &Vec<Option<i32>>, i: usize) -> Option<Rc<RefCell<TreeNode>>> {
        if (i >= vals.len()) {
            return None;
        }

        vals[i].map(|val| Rc::new(RefCell::new(
            TreeNode {
                val,
                left: Self::build_from_vec(vals, 2*i+1),
                right: Self::build_from_vec(vals, 2*i+2)
            }
        )))
    }
}
