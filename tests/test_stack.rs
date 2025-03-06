#[cfg(test)]
pub mod tests {
    use datastructures::stack::Stack;

    #[test]
    fn test_new_empty_stack() {
        let stack: Stack<i32> = Stack::new(None);
        assert_eq!(stack.len(), 0);
    }

    #[test]
    fn test_new_nonempty_stack() {
        let stack: Stack<i32> = Stack::new(Some(1));
        assert_eq!(stack.len(), 1);
        assert_eq!(stack.head(), Some(1));
    }

    #[test]
    fn test_push() {
        let mut stack: Stack<i32> = Stack::new(None);
        stack.push(1);
        stack.push(2);
        assert_eq!(stack.len(), 2);
        assert_eq!(stack.head(), Some(2));
    }

    #[test]
    fn test_pop() {
        let mut stack: Stack<i32> = Stack::new(Some(1));
        let val = stack.pop();
        assert_eq!(val, Some(1));
        assert_eq!(stack.len(), 0);
        let val = stack.pop();
        assert_eq!(val, None);
        assert_eq!(stack.len(), 0);
    }
}
