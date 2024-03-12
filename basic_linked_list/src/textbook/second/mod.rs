type Link<T> = Option<Box<Node<T>>>;

pub struct List<T> {
    head: Link<T>,
}

struct Node<T> {
    elem: T,
    next: Link<T>,
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
        self.head.take().map(|boxed_node| {
            self.head = boxed_node.next;
            boxed_node.elem
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while let Some(boxed_node) = self.head.take() {
            self.head = boxed_node.next;
        }
    }
}
