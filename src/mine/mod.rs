pub struct List<T> {
    head: Option<Box<Node<T>>>,
}
struct Node<T> {
    elem: T,
    next: Option<Box<Node<T>>>,
}
impl<T> super::LinkedList<T> for List<T> {
    fn new() -> Self {
        Self { head: None }
    }
    fn push(&mut self, elem: T) {
        self.head = Some(Box::new(Node {
            elem,
            next: self.head.take(),
        }));
    }
    fn pop(&mut self) -> Option<T> {
        match self.head.take() {
            None => None,
            Some(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}
impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while let Some(boxed_node) = self.head.take() {
            self.head = boxed_node.next;
        }
    }
}
