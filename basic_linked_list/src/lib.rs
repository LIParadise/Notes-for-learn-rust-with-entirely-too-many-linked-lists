pub mod mine;
pub mod textbook;

pub trait LinkedList<T> {
    fn new() -> Self;
    fn push(&mut self, elem: T);
    fn pop(&mut self) -> Option<T>;
    fn peek(&self) -> Option<&T>;
    fn peek_mut(&mut self) -> Option<&mut T>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use foo::*;
    mod foo {
        #[derive(Debug, PartialEq, Eq, Default)]
        pub struct Foo {
            pub u: usize,
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
        basic_test_worker(&mut mine);
        basic_test_worker(&mut textbook);
    }

    #[test]
    fn second() {
        let mut textbook = textbook::second::List::new();
        basic_test_worker(&mut textbook);
        let mut textbook: textbook::second::List<i32> = textbook::second::List::new();
        iter_worker(&mut textbook);
    }

    fn iter_worker<T: From<i32>, L: LinkedList<T> + IntoIterator + Iterator>(list: &mut L)
    where
        <L as IntoIterator>::Item: std::fmt::Debug,
        <L as Iterator>::Item: std::fmt::Debug,
    {
        list.push(1.into());
        list.push(1.into());
        list.push(4.into());
        list.push(5.into());
        list.push(1.into());
        list.push(4.into());
        vec![1.into(), 1.into(), 4.into(), 5.into(), 1.into(), 4.into()]
            .into_iter()
            .rev()
            .zip(list.into_iter())
            .for_each(|(a, b)| assert_eq!(a, b));
        vec![1.into(), 1.into(), 4.into(), 5.into(), 1.into(), 4.into()]
            .iter()
            .rev()
            .zip(list.iter())
            .for_each(|(a, b)| assert_eq!(a, b));
    }

    fn basic_test_worker<L: LinkedList<Foo> + ?Sized>(list: &mut L) {
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

        // Populate list so non-empty `Drop` and peep
        list.push(Foo::new(1));
        list.push(Foo::new(2));
        list.push(Foo::new(3));
        assert_eq!(list.peek(), Some(&Foo::new(3)));
        assert_eq!(list.peek_mut(), Some(&mut Foo::new(3)));
        list.push(Foo::new(6));
        list.push(Foo::new(8));
        assert_eq!(list.peek(), Some(&Foo::new(8)));
        assert_eq!(list.peek_mut(), Some(&mut Foo::new(8)));
        list.peek_mut().map(|x| *x = Foo::new(69));
        assert_eq!(list.peek(), Some(&Foo::new(69)));
        assert_eq!(list.peek_mut(), Some(&mut Foo::new(69)));
    }
}