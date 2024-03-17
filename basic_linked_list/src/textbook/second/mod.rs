use crate::LinkedList;

pub struct List<T> {
    head: Link<T>,
}
struct Node<T> {
    elem: T,
    next: Link<T>,
}
type Link<T> = Option<Box<Node<T>>>;

impl<T> List<T> {
    fn pop_by_box(&mut self) -> Link<T> {
        self.head.take().map(|mut boxed_node| {
            self.head = boxed_node.next.take();
            boxed_node
        })
    }
}

pub struct MyIntoIter<T>(List<T>);
impl<T> std::iter::Iterator for MyIntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}
impl<T> std::iter::IntoIterator for List<T> {
    type IntoIter = MyIntoIter<T>;
    type Item = T;
    fn into_iter(self) -> Self::IntoIter {
        MyIntoIter(self)
    }
}

pub struct MyIter<'a, T>(Option<&'a Node<T>>);
impl<'a, T> std::iter::Iterator for MyIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.map(|ref_node| {
            // self.0 = ref_node.next.as_ref().map::<&Node<T>, _>(|n| &n);
            self.0 = ref_node.next.as_deref();
            &ref_node.elem
        })
    }
}
impl<T> crate::HaveIter for List<T> {
    type GroundType = T;
    type Iter<'a, U: 'a> = MyIter<'a, T> where T: 'a;
    // Original:
    // fn iter<'a>(&'a self) -> MyIter<'a, T> {
    // Struct lifetime ellision:
    // fn iter(&self) -> MyIter<T> {
    // _explicitly elided_ lifetime
    fn iter(&self) -> MyIter<'_, T> {
        // MyIter(self.head.as_ref().map(|n| &**n))
        MyIter(self.head.as_deref())
    }
}

impl<T> crate::Clear for List<T> {
    fn clear(&mut self) {
        while let Some(_) = self.pop_by_box() {}
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
        <Self as crate::Clear>::clear(self)
    }
}
