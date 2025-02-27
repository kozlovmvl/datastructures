#[cfg(test)]
pub mod tests {
    use datastructures::linked_list::{List, Node};

    #[test]
    fn test_new_empty_list() {
        let list: List<i32> = List::new(None);
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn test_new_nonempty_list() {
        let head: Node<i32> = Node::new(1);
        let list: List<i32> = List::new(Some(Box::new(head.clone())));
        assert_eq!(list.len(), 1);
        assert_eq!(list.head().unwrap().val, head.val);
    }

    #[test]
    fn test_push() {
        let head: Node<i32> = Node::new(1);
        let node: Node<i32> = Node::new(2);
        let mut list: List<i32> = List::new(Some(Box::new(head)));
        list.push(Box::new(node));
        assert_eq!(list.len(), 2);
    }

    #[test]
    fn test_pop() {
        let head: Node<i32> = Node::new(1);
        let node: Node<i32> = Node::new(2);
        let mut list: List<i32> = List::new(Some(Box::new(head.clone())));
        list.push(Box::new(node.clone()));
        let first_pop = list.pop();
        assert_eq!(list.len(), 1);
        assert_eq!(first_pop.unwrap().val, node.val);
        let second_pop = list.pop();
        assert_eq!(list.len(), 0);
        assert_eq!(second_pop.unwrap().val, head.val);
    }

    #[test]
    fn test_iterator() {
        let head: Node<i32> = Node::new(1);
        let node: Node<i32> = Node::new(2);
        let mut list: List<i32> = List::new(Some(Box::new(head.clone())));
        list.push(Box::new(node.clone()));
        let mut iter = list.iter();
        assert_eq!(iter.next().unwrap().val, 1);
        assert_eq!(iter.next().unwrap().val, 2);

        let list: List<i32> = List::new(None);
        let mut iter = list.iter();
        assert!(iter.next().is_none());
    }

    #[test]
    fn test_index() {
        let head: Node<i32> = Node::new(1);
        let node: Node<i32> = Node::new(2);
        let mut list: List<i32> = List::new(Some(Box::new(head.clone())));
        list.push(Box::new(node.clone()));
        for i in 0..list.len() {
            assert_eq!(list[i].val, (i + 1) as i32);
        }
        for i in 0..list.len() {
            list[i].val += 1;
            assert_eq!(list[i].val, (i + 2) as i32);
        }
    }
}
