use core::fmt;

#[derive(Debug, Clone)]
pub struct TreeNode<T> {
    val: T,
    children: Vec<TreeNode<T>>,
}
impl<T: fmt::Display> fmt::Display for TreeNode<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display_recursive(f, 0)
    }
}
impl<T : fmt::Display> TreeNode<T> {
    // Constructor to create a new TreeNode
    pub fn new(val: T) -> Self {
        let children = Vec::new();
        TreeNode {
            val,
            children,
        }
    }

    // Method to add a child node to the current node
    pub fn add_child(&mut self, child: TreeNode<T>) {
        self.children.push(child);
    }
    // Method to get the value of the current node
    #[allow(dead_code)]
    pub fn val(&self) -> &T {
        &self.val
    }

    // Method to get the children of the current node, if any
    pub fn children(&self) -> &Vec<TreeNode<T>> {
        self.children.as_ref()
    }
    pub fn children_mut(&mut self) -> &mut Vec<TreeNode<T>> {
        self.children.as_mut()
    }
    pub fn last_mut(&mut self) -> &mut TreeNode<T> {
        self.children.last_mut().unwrap()
    }
    fn display_recursive(&self, f: &mut fmt::Formatter<'_>, depth: usize) -> fmt::Result {
        // Print indentation based on the depth
        for _ in 0..depth {
            write!(f, "  ")?;
        }
        
        // Print the current node's value
        write!(f, "{}\n", self.val)?;

        // Recursively print children
        for child in self.children() {
            child.display_recursive(f, depth + 1)?;
        }
        Ok(())
    }
    
    pub fn set_val(&mut self, val: T) {
        self.val = val;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_tree_node() {
        let node = TreeNode::new(5);
        assert_eq!(node.val(), &5);
        assert!(node.children().is_empty());
    }

    #[test]
    fn test_add_child() {
        let mut parent = TreeNode::new(5);
        let child = TreeNode::new(10);
        parent.add_child(child.clone());

        assert_eq!(parent.children().len(), 1);
        assert_eq!(parent.children()[0].val(), &10);
    }

    #[test]
    fn test_display() {
        let mut root = TreeNode::new("Root");
        let mut child1 = TreeNode::new("Child1");
        let child2 = TreeNode::new("Child2");
        child1.add_child(child2);
        root.add_child(child1);

        let expected_output = "Root\n  Child1\n    Child2\n";
        assert_eq!(format!("{}", root), expected_output);
    }

    #[test]
    #[should_panic]
    fn test_last_mut_empty() {
        let mut node: TreeNode<i32> = TreeNode::new(5);
        node.last_mut();
    }

    #[test]
    fn test_last_mut() {
        let mut root = TreeNode::new(5);
        let child1 = TreeNode::new(10);
        let child2 = TreeNode::new(15);
        root.add_child(child1);
        root.add_child(child2.clone());

        assert_eq!(root.last_mut().val(), &15);
    }
}

