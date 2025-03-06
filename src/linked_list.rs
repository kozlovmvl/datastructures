use std::{
    fmt::Debug,
    ops::{Index, IndexMut},
};

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
    head: Option<Box<Node<T>>>,
    len: usize,
}

pub struct ListIterator<'a, T>
where
    T: Clone + 'a,
{
    cur: Option<&'a Box<Node<T>>>,
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
    pub fn head(&self) -> Option<T> {
        match self.head.as_ref() {
            Some(node) => return Some(node.val.clone()),
            None => return None,
        }
    }

    pub fn len(&self) -> usize {
        return self.len;
    }

    pub fn new(head: Option<T>) -> Self {
        match head {
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
        if self.head.is_none() {
            self.head = Some(Box::new(Node::new(val)));
            self.len = 1;
        } else {
            let mut cur = self.head.as_deref_mut().unwrap();
            loop {
                if cur.next.is_none() {
                    break;
                }
                cur = cur.next.as_deref_mut().unwrap();
            }
            cur.next = Some(Box::new(Node::new(val)));
            self.len += 1;
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }
        if self.len == 1 {
            let res = self.head.as_deref().unwrap().val.clone();
            self.head = None;
            self.len = 0;
            return Some(res);
        }
        let mut prev = self.head.as_deref_mut().unwrap();
        for _ in 1..self.len - 1 {
            prev = prev.next.as_deref_mut().unwrap();
        }
        let res = prev.next.as_deref().unwrap().val.clone();
        prev.next = None;
        self.len -= 1;
        return Some(res);
    }

    pub fn iter(&self) -> ListIterator<T> {
        ListIterator::new(self.head.as_ref())
    }
}

impl<'a, T> ListIterator<'a, T>
where
    T: Clone + 'a,
{
    pub fn new(head: Option<&'a Box<Node<T>>>) -> Self {
        Self { cur: head }
    }
}

impl<'a, T> Iterator for ListIterator<'a, T>
where
    T: Clone + 'a,
{
    type Item = &'a Box<Node<T>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur.is_none() {
            return None;
        }
        let res = self.cur.unwrap();
        self.cur = res.next.as_ref();
        return Some(res);
    }
}

impl<T> Index<usize> for List<T>
where
    T: Clone,
{
    type Output = Box<Node<T>>;

    fn index(&self, index: usize) -> &Self::Output {
        let mut cur = self.head.as_ref().unwrap();
        for _ in 0..index {
            cur = cur.next.as_ref().unwrap();
        }
        return cur;
    }
}

impl<T> IndexMut<usize> for List<T>
where
    T: Clone,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let mut cur = self.head.as_mut().unwrap();
        for _ in 0..index {
            cur = cur.next.as_mut().unwrap();
        }
        return cur;
    }
}
