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
