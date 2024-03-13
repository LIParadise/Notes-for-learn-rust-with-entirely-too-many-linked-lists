use crate::LinkedList;

pub struct List<T> {
    head: Link<T>,
}

struct Node<T> {
    elem: T,
    next: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

pub struct MyIntoIter<T>(List<T>);
impl<T> std::iter::Iterator for MyIntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

pub struct MyIter<'a, T>(&'a Link<T>);
impl<'a, T> std::iter::Iterator for MyIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.as_ref().map(|boxed_node| {
            self.0 = &boxed_node.next;
            &boxed_node.elem
        })
    }
}

impl<T> std::iter::IntoIterator for List<T> {
    type IntoIter = MyIntoIter<T>;
    type Item = T;
    fn into_iter(self) -> Self::IntoIter {
        MyIntoIter(self)
    }
}

pub trait IHaveIterSuperTrait {
    type Iter<'a, U: 'a>: Iterator<Item = &'a U>;
}
pub trait IHaveIter: IHaveIterSuperTrait {
    fn iter<'a, T>(&'a self) -> Self::Iter<'a, T>;
}
impl<T> IHaveIterSuperTrait for List<T> {
    type Iter<'a, U: 'a> = MyIter<'a, U>;
}
impl<T> IHaveIter for List<T> {
    fn iter<'a, U>(&'a self) -> MyIter<'a, U> {
        MyIter(&self.head)
    }
}

impl<'a, T> List<T> {
    pub fn iter(&'a self) -> MyIter<'a, T> {
        MyIter(&self.head)
    }
}

impl<T> List<T> {
    fn pop_by_box(&mut self) -> Link<T> {
        self.head.take().map(|mut boxed_node| {
            self.head = boxed_node.next.take();
            boxed_node
        })
    }
}

impl<T> super::super::LinkedList<T> for List<T> {
    fn new() -> Self {
        Self { head: None }
    }
    fn push(&mut self, elem: T) {
        self.head = Some(Box::new(Node {
            elem,
            next: self.head.take(),
        }));
    }
    fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|ret| &ret.elem)
    }
    fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|ret| &mut ret.elem)
    }
    fn pop(&mut self) -> Option<T> {
        self.pop_by_box().map(|boxed_node| boxed_node.elem)
    }
}

impl<T> Drop for List<T> {
    // Iterative approach s.t. no stack overflow
    fn drop(&mut self) {
        while let Some(_) = self.pop_by_box() {}
    }
}
