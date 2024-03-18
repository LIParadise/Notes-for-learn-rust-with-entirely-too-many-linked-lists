pub mod mine;
pub mod textbook;

pub trait LinkedList<T> {
    fn new() -> Self;
    fn push(&mut self, elem: T);
    fn pop(&mut self) -> Option<T>;
    fn peek(&self) -> Option<&T>;
    fn peek_mut(&mut self) -> Option<&mut T>;
}
pub trait HaveIter {
    type Iter<'a, U: 'a>: Iterator<Item = &'a Self::GroundType>
    where
        Self: 'a;
    type GroundType;
    fn iter<'a>(&'a self) -> Self::Iter<'a, Self::GroundType>;
}
pub trait HaveIterMut {
    type GroundType;
    type IterMut<'a, U: 'a>: Iterator<Item = &'a mut Self::GroundType>
    where
        Self: 'a;
    // explicit lifetime ellision
    // here `'_` is just `'a` which in turn is just lifetime of self
    fn iter_mut<'a>(&mut self) -> Self::IterMut<'_, Self::GroundType>;
}
pub trait Clear {
    fn clear(&mut self);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::HaveIter;
    use foo::*;
    mod foo {
        #[derive(Debug, PartialEq, Eq, Default, Clone)]
        pub struct Foo {
            pub u: usize,
        }
        impl Foo {
            pub fn new(u: usize) -> Self {
                Self { u }
            }
        }
        impl From<i32> for Foo {
            fn from(value: i32) -> Self {
                Self::new(value as usize)
            }
        }
    }

    #[test]
    fn first() {
        let mut mine = mine::first::List::new();
        let mut textbook = textbook::first::List::new();
        basic_test_worker(&mut mine);
        basic_test_worker(&mut textbook);
    }

    #[test]
    fn second() {
        let mut textbook = textbook::second::List::new();
        let mut mine = mine::first::List::new();
        basic_test_worker(&mut textbook);
        basic_test_worker(&mut mine);
        let textbook: textbook::second::List<Foo> = textbook::second::List::new();
        let mine: mine::second::List<Foo> = mine::second::List::new();
        iter_worker(textbook);
        iter_worker(mine);
        let textbook: textbook::second::List<Foo> = textbook::second::List::new();
        iter_mut_worker(textbook);
    }

    fn iter_mut_worker<
        T: From<i32> + std::fmt::Debug + Eq + Clone,
        L: LinkedList<T> + HaveIterMut + Clear + HaveIterMut<GroundType = T>,
    >(
        mut list: L,
    ) where
        <L as HaveIterMut>::GroundType: std::fmt::Debug + Into<T> + PartialEq<T>,
    {
        let senpai = vec![
            T::from(1i32),
            1.into(),
            4.into(),
            5.into(),
            1.into(),
            4.into(),
        ];
        list.clear();
        senpai.into_iter().for_each(|t| list.push(t));
        let mut iter_mut = list.iter_mut();
        iter_mut.next();
        iter_mut.next();
        iter_mut.next().map(|optn| {
            *optn = T::from(114514);
        });
        drop(iter_mut);
        assert_eq!(
            vec![
                T::from(1),
                1.into(),
                4.into(),
                114514.into(),
                1.into(),
                4.into(),
            ]
            .into_iter()
            .rev()
            .inspect(|t| {
                assert_eq!(t.clone(), list.pop().unwrap());
            })
            .count(),
            6
        );
    }

    fn iter_worker<
        T: From<i32> + std::fmt::Debug + Eq + Clone,
        L: LinkedList<T> + IntoIterator + HaveIter + Clear,
    >(
        mut list: L,
    ) where
        <L as IntoIterator>::Item: std::fmt::Debug + Into<T> + Clone,
        <L as HaveIter>::GroundType: std::fmt::Debug + Into<T> + PartialEq<T> + Clone,
    {
        list.clear();
        let senpai = vec![
            T::from(1i32),
            1.into(),
            4.into(),
            5.into(),
            1.into(),
            4.into(),
        ];
        senpai.iter().for_each(|i| {
            list.push(i.clone());
        });
        assert_eq!(
            senpai
                .iter()
                .rev()
                .zip(list.iter())
                .inspect(|(a, b)| assert_eq!((*b).clone(), (*a).clone()))
                .count(),
            6
        );

        senpai.iter().for_each(|i| {
            list.push(i.clone());
        });
        let senpai = {
            let mut tmp = Vec::with_capacity(senpai.len() * 2);
            tmp.extend(senpai.iter().cloned());
            tmp.extend(senpai.into_iter());
            tmp
        };
        assert_eq!(
            senpai
                .into_iter()
                .rev()
                .zip(list.into_iter())
                .inspect(|(a, b)| assert_eq!(
                    *a,
                    <<L as IntoIterator>::Item as Into<T>>::into(b.clone())
                ))
                .count(),
            12
        );
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
