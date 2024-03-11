pub mod mine;
pub mod textbook;

pub trait LinkedList<T> {
    fn new() -> Self;
    fn push(&mut self, elem: T);
    fn pop(&mut self) -> Option<T>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use foo::*;
    mod foo {
        #[derive(Debug, PartialEq, Eq, Default)]
        pub struct Foo {
            u: usize,
        }
        impl Foo {
            pub fn new(u: usize) -> Self {
                Self { u }
            }
        }
    }

    #[test]
    fn first() {
        let mut mine = mine::List::new();
        let mut textbook = textbook::first::List::new();
        worker(&mut mine);
        worker(&mut textbook);
    }

    #[test]
    fn second() {
        let mut textbook = textbook::second::List::new();
        worker(&mut textbook);
    }

    fn worker<L: LinkedList<Foo> + ?Sized>(list: &mut L) {
        while let Some(_) = list.pop() {}

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(Foo::new(1));
        list.push(Foo::new(2));
        list.push(Foo::new(3));

        // Check normal removal
        assert_eq!(list.pop(), Some(Foo::new(3)));
        assert_eq!(list.pop(), Some(Foo::new(2)));

        // Push some more just to make sure nothing's corrupted
        list.push(Foo::new(4));
        list.push(Foo::new(5));

        // Check normal removal
        assert_eq!(list.pop(), Some(Foo::new(5)));
        assert_eq!(list.pop(), Some(Foo::new(4)));

        // Check exhaustion
        assert_eq!(list.pop(), Some(Foo::new(1)));
        assert_eq!(list.pop(), None);

        // Populate list so non-empty `Drop`
        list.push(Foo::new(1));
        list.push(Foo::new(2));
        list.push(Foo::new(3));
    }
}
