use std::{io::Empty, mem};

use crate::LinkedList;

pub struct List<T> {
    head: Arrow<T>,
}

struct Node<T> {
    elem: T,
    next: Arrow<T>,
}

enum Arrow<T> {
    Empty,
    More(Box<Node<T>>),
}

impl<T> List<T> {
    fn pop_worker(&mut self) -> Arrow<T> {
        match mem::replace(&mut self.head, Arrow::Empty) {
            Arrow::Empty => Arrow::Empty,
            Arrow::More(mut node) => {
                self.head = mem::replace(&mut node.next, Arrow::Empty);
                Arrow::More(node)
            }
        }
    }
}

impl<T> super::super::LinkedList<T> for List<T> {
    fn new() -> Self {
        Self { head: Arrow::Empty }
    }
    fn push(&mut self, elem: T) {
        self.head = Arrow::More(Box::new(Node {
            elem,
            next: mem::replace(&mut self.head, Arrow::Empty),
        }));
    }
    fn peek(&self) -> Option<&T> {
        // Legacy code, the `ref` keyword
        // Try not to use it
        /*
        match self.head {
            Arrow::Empty => None,
            Arrow::More(ref ret) => Some(&ret.elem),
        }
        */
        match &self.head {
            Arrow::Empty => None,
            Arrow::More(ret) => Some(&ret.elem),
        }
    }
    fn peek_mut(&mut self) -> Option<&mut T> {
        // Legacy code, the `ref` keyword
        // Try not to use it
        /*
        match self.head {
            Arrow::Empty => None,
            Arrow::More(ref mut ret) => Some(&mut ret.elem),
        }
        */
        match &mut self.head {
            Arrow::Empty => None,
            Arrow::More(ret) => Some(&mut ret.elem),
        }
    }
    fn pop(&mut self) -> Option<T> {
        match self.pop_worker() {
            Arrow::Empty => None,
            Arrow::More(ret) => Some(ret.elem),
        }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        // `Box` makes tail recursion nearly impossible
        // since one have to call `Drop` for content _before_ deallocating
        //
        // DIY iterative version s.t. no stack overflow
        //
        // Notice we didn't move any element, so not `pop`;
        // instead we move only pointers, which is cheap
        while let Arrow::More(_) = self.pop_worker() {}
    }
}
