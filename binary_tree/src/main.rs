/// A node in the binary tree.
#[derive(Debug)]
struct Node<T: Ord> {
    value: T,
    left: Subtree<T>,
    right: Subtree<T>,
}

impl<T: Ord> Node<T>{
    fn new(value: T) -> Self{
        Self { value, left: Subtree::new(), right: Subtree::new() }
    }
}

/// A possibly-empty subtree.
#[derive(Debug)]
struct Subtree<T: Ord>(Option<Box<Node<T>>>);

/// A container storing a set of values, using a binary tree.
///
/// If the same value is added multiple times, it is only stored once.
#[derive(Debug)]
pub struct BinaryTree<T: Ord> {
    root: Subtree<T>,
}

impl<T: Ord> BinaryTree<T> {
    fn new() -> Self {
        Self { root: Subtree::new() }
    }

    fn insert(&mut self, value: T) {
        self.root.insert(value);
    }

    fn has(&self, value: &T) -> bool {
        self.root.has(value)
    }

    fn len(&self) -> usize {
        self.root.len()
    }
}

// Implement `new`, `insert`, `len`, and `has` for `Subtree`.
impl <T: Ord> Subtree<T>{
    fn new() -> Self{
        Self(None)
    }

    fn insert(&mut self, value: T){
        if self.0.is_none(){
            self.0 = Some(Box::new(Node::new(value)));
            return;
        }
        let current = &self.0.as_ref().unwrap().value;
        if value < *current{
            self.0.as_mut().unwrap().left.insert(value);
        }
        else if value > *current{
            self.0.as_mut().unwrap().left.insert(value);
        }
    }

    fn len(&self) -> usize{
        if self.0.is_none(){
            0
        }else{
            1 + self.0.as_ref().unwrap().left.len() + self.0.as_ref().unwrap().right.len()
        }
    }

    fn has(&self, value: &T) -> bool{
        if self.0.is_none(){
            return false;
        }
        let current = &self.0.as_ref().unwrap().value;
        if value < current{
            self.0.as_ref().unwrap().left.has(value)
        }else if value > current{
            self.0.as_ref().unwrap().left.has(value)
        }else{
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn len() {
        let mut tree = BinaryTree::new();
        assert_eq!(tree.len(), 0);
        tree.insert(2);
        assert_eq!(tree.len(), 1);
        tree.insert(1);
        assert_eq!(tree.len(), 2);
        tree.insert(2); // not a unique item
        assert_eq!(tree.len(), 2);
        tree.insert(3);
        assert_eq!(tree.len(), 3);
    }

    #[test]
    fn has() {
        let mut tree = BinaryTree::new();
        fn check_has(tree: &BinaryTree<i32>, exp: &[bool]) {
            let got: Vec<bool> =
                (0..exp.len()).map(|i| tree.has(&(i as i32))).collect();
            assert_eq!(&got, exp);
        }

        check_has(&tree, &[false, false, false, false, false]);
        tree.insert(0);
        check_has(&tree, &[true, false, false, false, false]);
        tree.insert(4);
        check_has(&tree, &[true, false, false, false, true]);
        tree.insert(4);
        check_has(&tree, &[true, false, false, false, true]);
        tree.insert(3);
        check_has(&tree, &[true, false, false, true, true]);
    }

    #[test]
    fn unbalanced() {
        let mut tree = BinaryTree::new();
        for i in 0..100 {
            tree.insert(i);
        }
        assert_eq!(tree.len(), 100);
        assert!(tree.has(&50));
    }
}