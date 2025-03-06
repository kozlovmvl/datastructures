#[cfg(test)]
pub mod tests {
    use datastructures::binary_tree::Tree;

    #[test]
    fn test_new_empty_tree() {
        let tree: Tree<i32> = Tree::new(None);
        assert!(tree.root().is_none());
    }

    #[test]
    fn test_new_nonempty_tree() {
        let tree = Tree::new(Some(1));
        assert_eq!(tree.root(), Some(1));
    }

    #[test]
    fn test_create_from_vec() {
        let vec: Vec<Option<i32>> = vec![Some(1), Some(2), None, None, None];
        let tree = Tree::from(vec.as_ref());
        assert_eq!(tree.root(), Some(1));
    }

    #[test]
    fn test_as_vec() {
        let vec: Vec<Option<i32>> = vec![Some(1), Some(2), None, None, None];
        let tree = Tree::from(vec.as_ref());
        let array = tree.as_vec();
        assert_eq!(array.as_ref(), [Some(1), Some(2), None, None, None]);

        let vec: Vec<Option<i32>> = vec![Some(1), None, None];
        let tree = Tree::from(vec.as_ref());
        let array = tree.as_vec();
        assert_eq!(array.as_ref(), [Some(1), None, None]);

        let vec: Vec<Option<i32>> = vec![Some(1), Some(2), Some(3), None, None, None, None];
        let tree = Tree::from(vec.as_ref());
        let array = tree.as_vec();
        assert_eq!(
            array.as_ref(),
            [Some(1), Some(2), Some(3), None, None, None, None]
        );
    }
}
