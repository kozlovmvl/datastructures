use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct Node<T>
where
    T: Clone,
{
    pub val: T,
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
    pub fn new(root: Option<T>) -> Self {
        match root {
            Some(v) => {
                return Self {
                    root: Some(Rc::new(RefCell::new(Node::new(v)))),
                }
            }
            None => return Self { root: None },
        }
    }

    pub fn root(&self) -> Option<T> {
        match self.root.as_ref() {
            Some(node) => return Some((**node).borrow().val.clone()),
            None => return None,
        }
    }

    pub fn as_vec(&self) -> Vec<Option<T>> {
        let mut array: Vec<Option<T>> = Vec::new();

        fn rec<T>(node: Rc<RefCell<Node<T>>>, array: &mut Vec<Option<T>>)
        where
            T: Clone,
        {
            let node_ = (*node).borrow();
            let left = node_.left.borrow();
            let right = node_.right.borrow();
            if left.is_none() {
                array.push(None);
            } else {
                array.push(Some((*left.clone().unwrap()).borrow().val.clone()));
            }
            if right.is_none() {
                array.push(None);
            } else {
                array.push(Some((*right.clone().unwrap()).borrow().val.clone()));
            }

            if let Some(l) = left {
                rec(Rc::clone(&l), array);
            }
            if let Some(r) = right {
                rec(Rc::clone(&r), array);
            }
        }
        if let Some(root_) = self.root.as_ref() {
            array.push(Some((**root_).borrow().val.clone()));
            rec(Rc::clone(root_), &mut array);
        } else {
            array.push(None);
        }

        array
    }
}

impl<T> From<&[Option<T>]> for Tree<T>
where
    T: Clone,
{
    fn from(value: &[Option<T>]) -> Self {
        let root = Rc::new(RefCell::new(Node::new(value[0].clone().unwrap())));

        fn rec<T>(root: Rc<RefCell<Node<T>>>, value: &[Option<T>], i: usize) -> usize
        where
            T: Clone,
        {
            if i >= value.len() {
                return i;
            }
            let mut j = i + 2;
            if let Some(val) = value[i].clone() {
                let left = Rc::new(RefCell::new(Node::new(val)));
                (*root).borrow_mut().left = Some(Rc::clone(&left));
                j = rec(Rc::clone(&left), value, i + 2);
            }
            if let Some(val) = value[i + 1].clone() {
                let right = Rc::new(RefCell::new(Node::new(val)));
                (*root).borrow_mut().right = Some(Rc::clone(&right));
                j = rec(Rc::clone(&right), value, j);
            }
            j
        }

        rec(Rc::clone(&root), value, 1);

        let tree = Self { root: Some(root) };
        tree
    }
}
