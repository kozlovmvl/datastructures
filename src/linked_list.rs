#[derive(Clone, Debug)]
pub struct Node<T>
where
    T: Clone,
{
    pub val: T,
    pub next: Option<Box<Node<T>>>,
}

pub struct List<T>
where
    T: Clone,
{
    pub head: Option<Box<Node<T>>>,
    pub len: usize,
}

impl<T> Node<T>
where
    T: Clone,
{
    pub fn new(val: T) -> Self {
        Self { val, next: None }
    }
}

impl<T> List<T>
where
    T: Clone,
{
    pub fn new(head: Option<Box<Node<T>>>) -> Self {
        if head.is_none() {
            return Self { head, len: 0 };
        }
        Self { head, len: 1 }
    }

    pub fn push(&mut self, node: Box<Node<T>>) {
        if self.head.is_none() {
            self.head = Some(node);
            self.len = 1;
        } else {
            let mut cur = self.head.as_deref_mut().unwrap();
            loop {
                if cur.next.is_none() {
                    break;
                }
                cur = cur.next.as_deref_mut().unwrap();
            }
            cur.next = Some(node);
            self.len += 1;
        }
    }

    pub fn pop(&mut self) -> Option<Node<T>> {
        if self.len == 0 {
            return None;
        }
        if self.len == 1 {
            let res = self.head.as_deref().unwrap().clone();
            self.head = None;
            self.len = 0;
            return Some(res);
        }
        let mut prev = self.head.as_deref_mut().unwrap();
        for _ in 1..self.len - 1 {
            prev = prev.next.as_deref_mut().unwrap();
        }
        let res = prev.next.as_deref().unwrap().clone();
        prev.next = None;
        self.len -= 1;
        return Some(res);
    }
}
