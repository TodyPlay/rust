#[derive(Debug)]
pub struct TwoTree<T> where T: PartialOrd {
    root_node: Option<TreeNode<T>>,
}

#[derive(Debug)]
struct TreeNode<T> {
    val: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T> where T: PartialOrd {
    fn new(val: T) -> TreeNode<T> {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    fn put_val(&mut self, val: T) {
        let node;

        if val > self.val {
            node = &mut self.right;
        } else {
            node = &mut self.left;
        };

        match node {
            None => { *node = Some(Box::new(TreeNode::new(val))) }
            Some(nodes) => { nodes.put_val(val) }
        };
    }

    fn for_each<K>(&self, func: &K)
        where K: Fn(&T) {
        func(&self.val);
        match &self.left {
            Some(val) => val.for_each(func),
            None => {}
        }
        match &self.right {
            Some(val) => val.for_each(func),
            None => {}
        }
    }
}

impl<T> TwoTree<T> where T: PartialOrd {
    pub fn new() -> TwoTree<T> {
        TwoTree {
            root_node: None,
        }
    }

    pub fn add(&mut self, val: T) {
        match &mut self.root_node {
            None => self.root_node = Some(TreeNode::new(val)),
            Some(node) => node.put_val(val),
        };
    }

    pub fn for_each<K>(&mut self, func: K)
        where K: Fn(&T) {
        match &self.root_node {
            None => {}
            Some(node) => { node.for_each(&func) }
        };
    }
}