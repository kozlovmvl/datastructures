use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct Node<T>
where
    T: Clone,
{
    val: T,
    left: Option<Rc<RefCell<Node<T>>>>,
    right: Option<Rc<RefCell<Node<T>>>>,
}

#[derive(Clone)]
pub struct Tree<T>
where
    T: Clone,
{
    root: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Node<T>
where
    T: Clone,
{
    pub fn new(val: T) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }
}

impl<T> Tree<T>
where
    T: Clone,
{
    pub fn new(root: Option<Rc<RefCell<Node<T>>>>) -> Self {
        Self { root }
    }
}
