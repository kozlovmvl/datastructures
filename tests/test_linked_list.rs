#[cfg(test)]
pub mod tests {
    use datastructures::linked_list::List;

    #[test]
    fn test_new_empty_list() {
        let list: List<i32> = List::new(None);
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn test_new_nonempty_list() {
        let list: List<i32> = List::new(Some(1));
        assert_eq!(list.len(), 1);
        assert_eq!(list.head(), Some(1));
    }

    #[test]
    fn test_push() {
        let mut list: List<i32> = List::new(Some(1));
        list.push(2);
        assert_eq!(list.len(), 2);
    }

    #[test]
    fn test_pop() {
        let mut list: List<i32> = List::new(Some(1));
        list.push(2);
        let first_pop = list.pop();
        assert_eq!(list.len(), 1);
        assert_eq!(first_pop.unwrap(), 2);
        let second_pop = list.pop();
        assert_eq!(list.len(), 0);
        assert_eq!(second_pop.unwrap(), 1);
    }

    #[test]
    fn test_iterator() {
        let mut list: List<i32> = List::new(Some(1));
        list.push(2);
        let mut iter = list.iter();
        assert_eq!(iter.next().unwrap().val, 1);
        assert_eq!(iter.next().unwrap().val, 2);

        let list: List<i32> = List::new(None);
        let mut iter = list.iter();
        assert!(iter.next().is_none());
    }

    #[test]
    fn test_index() {
        let mut list: List<i32> = List::new(Some(1));
        list.push(2);
        for i in 0..list.len() {
            assert_eq!(list[i].val, (i + 1) as i32);
        }
        for i in 0..list.len() {
            list[i].val += 1;
            assert_eq!(list[i].val, (i + 2) as i32);
        }
    }
}
