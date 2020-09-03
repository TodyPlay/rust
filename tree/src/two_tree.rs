#[derive(Debug)]
pub struct TwoTree {
    root_node: Option<TreeNode>,
}

#[derive(Debug)]
struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(val: i32) -> TreeNode {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    fn put_val(&mut self, val: i32) {
        if val > self.val {
            match &mut self.right {
                None => { self.right = Some(Box::new(TreeNode::new(val))) }
                Some(nodes) => { nodes.put_val(val) }
            }
        } else {
            match &mut self.left {
                None => { self.left = Some(Box::new(TreeNode::new(val))) }
                Some(nodes) => { nodes.put_val(val) }
            }
        }
    }
}

impl TwoTree {
    pub fn new() -> TwoTree {
        TwoTree {
            root_node: None,
        }
    }

    pub fn add(&mut self, val: i32) {
        match &mut self.root_node {
            None => self.root_node = Some(TreeNode::new(val)),
            Some(node) => node.put_val(val),
        };
    }
}