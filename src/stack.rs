#[derive(Clone)]
pub struct Node<T>
where
    T: Clone,
{
    val: T,
    prev: Option<Box<Node<T>>>,
}

#[derive(Clone)]
pub struct Stack<T>
where
    T: Clone,
{
    head: Option<Box<Node<T>>>,
    len: usize,
}

impl<T> Node<T>
where
    T: Clone,
{
    pub fn new(val: T) -> Self {
        Self { val, prev: None }
    }
}

impl<T> Stack<T>
where
    T: Clone,
{
    pub fn new(val: Option<T>) -> Self {
        match val {
            Some(v) => {
                return Self {
                    head: Some(Box::new(Node::new(v))),
                    len: 1,
                }
            }
            None => return Self { head: None, len: 0 },
        }
    }

    pub fn push(&mut self, val: T) {
        let mut node = Node::new(val);
        node.prev = self.head.clone();
        self.head = Some(Box::new(node));
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }
        let val = self.head.as_ref().unwrap().val.clone();
        self.len -= 1;
        self.head = self.head.as_ref().unwrap().prev.clone();
        return Some(val);
    }

    pub fn head(&self) -> Option<T> {
        if self.len == 0 {
            return None;
        }
        Some(self.head.as_ref().unwrap().val.clone())
    }

    pub fn len(&self) -> usize {
        self.len
    }
}
