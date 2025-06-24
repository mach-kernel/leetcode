use std::cell::RefCell;
use std::cmp::max;
use std::rc::Rc;
use crate::lib::tree_node::TreeNode;

pub fn dls(root: Option<Rc<RefCell<TreeNode>>>, depth: usize) -> (i32, usize) {
    if let None = root {
        return (0, 0);
    }

    let node = root.unwrap();
    let node = node.borrow();
    let (lsum, ldepth) = dls(node.left.clone(), depth + 1);
    let (rsum, rdepth) = dls(node.right.clone(), depth + 1);

    let zsum = if (ldepth > rdepth) {
        lsum
    } else if (rdepth > ldepth) {
        rsum
    } else {
        lsum + rsum
    };

    let zdepth = max(ldepth, rdepth);

    if (depth > zdepth) {
        (node.val, depth)
    } else {
        (zsum, zdepth)
    }
}

pub fn deepest_leaves_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let (s, _) = dls(root, 1);
    s
}

#[cfg(test)]
mod tests {
    use crate::lib::tree_node::TreeNode;
    use crate::p1302::deepest_leaves_sum;

    #[test]
    fn test_deepest_leaves_sum() {
        let tree1 = TreeNode::from_vec(
            vec![Some(6),Some(7),Some(8),Some(2),Some(7),Some(1),Some(3),Some(9),None,Some(1),Some(4),None,None,None,Some(5)]
        );

        println!("{:?}", tree1);

        assert_eq!(
            deepest_leaves_sum(tree1),
            19
        );
    }
}
