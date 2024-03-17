# Learn Rust with Entirely Too Many Linked Lists

## index.html

`VecDeque`
intrusive lists, stack?
trie
MPSC queue

> linked lists are the defacto heroes of the dark world of lock-free concurrency.

## first-layout.html

> One of the few nice things about a linked list is that you can construct the element in the node itself, and then freely shuffle it around lists without ever moving it. You just fiddle with pointers and stuff gets "moved". Layout 1 trashes this property.

> null pointer optimization... eliminates the space needed for the tag... This is why Rust leaves enum layout totally unspecified... several other important types in Rust have no overhead when put in an `Option`

## first-ownership.html

> When you pass something by value, it's moved to the new location.

> The only thing you can't do with an `&mut` is move the value out with no replacement.

- How costly is move (semantics), though?

## first-push.html

[exception safety](https://doc.rust-lang.org/nightly/nomicon/exception-safety.html)

## first-pop.html

[diverging function](https://doc.rust-lang.org/nightly/book/ch19-04-advanced-types.html#the-never-type-that-never-returns)

> Diverging functions never return to the caller, so they may be used in places where a value of any type is expected. 

- Why `mem::replace` result must be used? It's `T` rather than `Result<T,E>`...

> The key insight is we want to remove things, which means we want to get the head of the list _by value_... We also "only" have a mutable reference to self, so the only way we can move stuff is to replace it.

## first-test.html

> use `mod` to basically create a whole new file inline:

> (`#[cfg(test)]`) ...indicate that the whole test module should only be compiled if we're running tests. 

## first-drop.html

> automatic handling (`list -> A -> B -> C`)... might be thinking "this is clearly tail recursive..."... is in fact, incorrect!

> We can't drop the contents of the Box after deallocating, so there's no way to drop in a tail-recursive manner!

## second-peek.html

Actually it's fine using the match syntax, with the help of the magical keyword `ref` in pattern branch, which affects _how_ it binds but _if_ it binds. It's kinda an old feature, so try not to use it, but in fact it's exactly how `Option::as_ref()` is implemented as of Rust 1.76.0 (07dca489a 2024-02-04).

``` rust
fn peep(&self) -> Option<&T> {
    match self.head {
        None => None,
        Some(ref n) => Some(&n.elem),
    }
}
```

``` rust
// Notice `ref mut` != `mut ref`
// the former produces `&mut`
// the latter produces `mut &`, a borrow that could rebind.
fn peek_mut(&mut self) -> Option<&mut T> {
    match self.head {
        Arrow::Empty => None,
        Arrow::More(ref mut ret) => Some(&mut ret.elem),
    }
}
```

``` rust
// [Rust src](https://doc.rust-lang.org/src/core/option.rs.html#680)
pub const fn as_ref(&self) -> Option<&T> {
    match *self {
        Some(ref x) => Some(x),
        None => None,
    }
}
```

About the borrow checker, currently if all branches are with `ref` keyword when pattern matching, then it's considered a borrow, and subsequent uses are fine. Else, if one of the branches is pattern matching, than the usual move semantics applies, and subsequent uses are invalid.

## second-into-iter.html

> ...Sadly, Rust has nothing like a `yield` statement (yet)

what is a `yield` again?

## second-iter.html

> Normally Rust is very good at doing this kind of conversion implicitly, through a process called _deref coercion_, where basically it can insert `*`'s throughout your code to make it type-check. It can do this because we have the borrow checker to ensure we never mess up pointers!


``` rust
pub fn map<U, F>(self, f: F) -> Option<U>
self.next = node.next.as_ref().map(|node| &**node);
self.next = node.next.as_ref().map::<&Node<T>, _>(|node| &node);
self.next = node.next.as_deref();
```

> `map` is a generic function, the turbofish, `::<>`, lets us tell the compiler what we think the types of those generics should be. In this case `::<&Node<T>, _>` says "it should return a `&Node<T>`, and I don't know/care about that other type".

> This in turn lets the compiler know that &node should have deref coercion applied to it, so we don't need to manually apply all those `*`'s!
