---
title: Some notes on Rust's std::borrow::{Borrow, ToOwned, Cow}
tags: programming, Rust
---

Some notes on two traits and a type from the `std::borrow` module in the Rust standard library. All the definitions discussed can be found in `libcollections/borrow.rs`.

## Borrow

The first trait we'll look at is `std::borrow::Borrow`. The docs call this "a trait for borrowing data". It's a trait with a single method, `borrow`. Here's the definition:

```rust
pub trait Borrow<Borrowed: ?Sized> {
    /// Immutably borrows from an owned value.
    fn borrow(&self) -> &Borrowed;
}
```

(You can safely ignore the `?Sized` thing here and in everything that follows if you find it to be baffling. It just means that the trait accepts [unsized][the-sized-trait] type parameters.)

What is the meaning of this trait? If `T` and `U` are types and `T` implements `Borrow<U>`, then you can use the `borrow` method on a value of type `&T` to obtain a value of type `&U`.

Let's build a toy implementation of this trait in order to understand it better. We'll start with perhaps the simplest possible types, unit structs:

```rust
struct Foo;
struct Baz;
```

Suppose that we want to make `Baz` implement `Borrow<Foo>`. The implementation will look something like this:

```rust
impl Borrow<Foo> for Baz {
    fn borrow(&self) -> &Foo { ... }
}
```

There's a problem here. How are we going to get a reference to a `Foo` from a reference to a `Baz`?  As defined, it doesn't seem possible, but we can do it if we change the definition of `Baz` so that it wraps a `Foo` value:

```rust
struct Baz(Foo);
```

The implementation is now straightforward:

```rust
impl Borrow<Foo> for Baz {
    fn borrow(&self) -> &Foo {
        let Baz(ref foo) = *self;
        foo
    }
}
```

Inside the `borrow` method, `self` is a reference to a `Baz` value (i.e. it is of type `&Baz`). The `let Baz(ref foo) = *self;` line destructures the `&Baz` to obtain a reference to the inner `Foo`. `borrow` subsequently returns this reference.

Now, since `Baz` implements `Borrow<Foo>`, we can use the `borrow` method on a value of type `&Baz` to obtain a value of type `&Foo`:

```rust
fn main() {
    let foo = Foo;
    let baz = Baz(foo);
    let foo_ref: &Foo = baz.borrow();
}
```

(The type annotation on `foo_ref` is not necessary, I just put it there for emphasis.)

So this compiles and that's great and all, but the program we've made is clearly useless. What's the *point* of this trait? Perhaps the docs can help us out now:

 > A trait for borrowing data.
 >
 > In general, there may be several ways to "borrow" a piece of data.  The
 > typical ways of borrowing a type `T` are `&T` (a shared borrow) and `&mut T`
 > (a mutable borrow). But types like `Vec<T>` provide additional kinds of
 > borrows: the borrowed slices `&[T]` and `&mut [T]`.
 >
 > When writing generic code, it is often desirable to abstract over all ways
 > of borrowing data from a given type. That is the role of the `Borrow`
 > trait: if `T: Borrow<U>`, then `&U` can be borrowed from `&T`.  A given
 > type can be borrowed as multiple different types. In particular, `Vec<T>:
 > Borrow<Vec<T>>` and `Vec<T>: Borrow<[T]>`.

Hopefully this makes at least some sense. This level of understanding should be enough to begin studying the next trait, `ToOwned`.

## ToOwned

The `ToOwned` trait is of particular interest since it is in the [Rust 1.0 prelude][v1-prelude] The Rust docs call it a "generalization of `Clone` to borrowed data". Here's the definition:

```rust
pub trait ToOwned {
    type Owned: Borrow<Self>;

    /// Creates owned data from borrowed data, usually by cloning.
    fn to_owned(&self) -> Self::Owned;
}
```

As was the case for `Borrow`, `ToOwned` has only one method. However, this trait is much more confusing (to me) because it has an associated type `Owned` with this weird `Borrow<Self>` bound on it. What could possibly be the point of that? Well, we know from our previous exploration of the `Borrow` trait that `Owned: Borrow<Self>` means that `&Self` can be borrowed from `&Owned`, but this fact alone does not shed much light on it for me. Maybe the docs say something about it?

 > A generalization of `Clone` to borrowed data.
 >
 > Some types make it possible to go from borrowed to owned, usually by
 > implementing the `Clone` trait. But `Clone` works only for going from `&T`
 > to `T`. The `ToOwned` trait generalizes `Clone` to construct owned data
 > from any borrow of a given type.

This is helpful, but it does not seem to say anything about why the `Owned` type must implement `Borrow<Self>`. Hrm.

Let's try to expand our toy example from earlier. Recall that we had these definitions:

```rust
struct Foo;

struct Baz(Foo);

impl Borrow<Foo> for Baz {
    fn borrow(&self) -> &Foo {
        let Baz(ref foo) = *self;
        foo
    }
}
```

Since we already have `Baz: Borrow<Foo>`, it seems like the path of least resistance is to make an implementation of `ToOwned` for `Foo` where the associated `Owned` type is `Baz`:

```rust
impl ToOwned for Foo {
    type Owned = Baz;

    fn to_owned(&self) -> Baz { ... }
}
```

How should the `to_owned` implementation work? We need to manufacture a `Baz` value from a `&Foo` value. The easiest thing I can think of is to make `Foo` implement `Clone`, so that we can go `&Foo` -> `Foo`. Then we can just use `Baz`'s constructor on the cloned `Foo` value:

```rust
#[derive(Clone)]
struct Foo;

impl ToOwned for Foo {
    type Owned = Baz;

    fn to_owned(&self) -> Baz {
        Baz(self.clone())
    }
}
```

This seems like it will compile! Except it actually doesn't:

```
toy_std_borrow.rs:16:1: 22:2 error: conflicting implementations for trait `collections::borrow::ToOwned` [E0119]
toy_std_borrow.rs:16 impl ToOwned for Foo {
toy_std_borrow.rs:17     type Owned = Baz;
toy_std_borrow.rs:18
toy_std_borrow.rs:19     fn to_owned(&self) -> Baz {
toy_std_borrow.rs:20         Baz(self.clone())
toy_std_borrow.rs:21     }
                     ...
toy_std_borrow.rs:16:1: 22:2 help: run `rustc --explain E0119` to see a detailed explanation
toy_std_borrow.rs:16:1: 22:2 note: conflicting implementation in crate `collections`
toy_std_borrow.rs:16 impl ToOwned for Foo {
toy_std_borrow.rs:17     type Owned = Baz;
toy_std_borrow.rs:18
toy_std_borrow.rs:19     fn to_owned(&self) -> Baz {
toy_std_borrow.rs:20         Baz(self.clone())
toy_std_borrow.rs:21     }
                     ...
error: aborting due to previous error
```

If you look in `libcollections/borrow.rs`, you'll find this implementation, which seems to be the culprit here:

```rust
impl<T> ToOwned for T where T: Clone {
    type Owned = T;
    fn to_owned(&self) -> T { self.clone() }
}
```

So by making `Foo` implement `Clone`, that automatically makes this implementation apply. I suppose that was to be expected. The docs did say that `ToOwned` was "a generalization of `Clone`". Since Rust currently lacks [impl specialization](https://github.com/rust-lang/rfcs/pull/1210), our toy implementation is not allowed.

Actually, we don't need `Foo` to implement Clone. It's a zero-sized type, so there's nothing to clone. This should work instead:

```rust
impl ToOwned for Foo {
    type Owned = Baz;

    fn to_owned(&self) -> Baz {
        Baz(Foo)
    }
}
```

Hooray! We have a useless implementation of `ToOwned` that successfully compiles. Does this clear up our earlier confusion on why the `Borrow<Self>` bound was needed? Consider this code:

```rust
fn main() {
    let foo = Foo;
    let baz = foo.to_owned();
    let baz_ref = &baz;
    let foo_ref = baz_ref.borrow();
}
```

The `baz_ref.borrow()` line is only permitted because `<Foo as ToOwned>::Owned` (i.e. `Baz`) implements `Borrow<Foo>`, but... I'm still not sure why this is needed or even desired. I think the use of toy implementations here is impeding understanding, unfortunately.

Let's set aside our confusion for now and move on to the `Cow` type. Maybe that will clear it up?

## Cow

This seemingly oddly-named type stands for "clone-on-write". It is a smart pointer type, an `enum` with two variants:

```rust
pub enum Cow<'a, B: ?Sized + 'a> where B: ToOwned {
    /// Borrowed data.
    Borrowed(&'a B),

    /// Owned data.
    Owned(<B as ToOwned>::Owned)
}
```

The docs say this about it:

 > The type `Cow` is a smart pointer providing clone-on-write functionality: it
 > can enclose and provide immutable access to borrowed data, and clone the
 > data lazily when mutation or ownership is required. The type is designed to
 > work with general borrowed data via the `Borrow` trait.

This type has two inherent methods defined for it (implementations omitted temporarily):

```rust
impl<'a, B: ?Sized> Cow<'a, B> where B: ToOwned {
    /// Acquires a mutable reference to the owned form of the data.
    ///
    /// Clones the data if it is not already owned.
    pub fn to_mut(&mut self) -> &mut <B as ToOwned>::Owned { ... }

    /// Extracts the owned data.
    ///
    /// Clones the data if it is not already owned.
    pub fn into_owned(self) -> <B as ToOwned>::Owned { ... }
}
```

The implementations of the methods make sense to me given the descriptions. `to_mut` acquires a mutable reference, converting a `Borrowed` variant to the `Owned` variant as needed:

```rust
pub fn to_mut(&mut self) -> &mut <B as ToOwned>::Owned {
    match *self {
        Borrowed(borrowed) => {
            *self = Owned(borrowed.to_owned());
            self.to_mut()
        }
        Owned(ref mut owned) => owned
    }
}
```

On the other hand, `into_owned` consumes the `Cow` value, again converting the `Borrowed` variant to the `Owned` variant as needed and yielding the inner `Owned` value:

```rust
pub fn into_owned(self) -> <B as ToOwned>::Owned {
    match self {
        Borrowed(borrowed) => borrowed.to_owned(),
        Owned(owned) => owned
    }
}
```

This all seems to make sense. Note that the type parameter `B` for `Cow` has a `ToOwned` bound on it. Does this help us resolve our earlier confusion with the `ToOwned` trait? Well, since `Cow` is a smart pointer type, it makes sense that it would implement `Deref` right? It's important that pointers can be dereferenced. Indeed, the docs say:

 > `Cow` implements `Deref`, which means that you can call
 > non-mutating methods directly on the data it encloses. If mutation
 > is desired, `to_mut` will obtain a mutable reference to an owned
 > value, cloning if necessary.

The `Deref` impl is probably worth a peek, huh?

```rust
impl<'a, B: ?Sized> Deref for Cow<'a, B> where B: ToOwned {
    type Target = B;

    fn deref(&self) -> &B {
        match *self {
            Borrowed(borrowed) => borrowed,
            Owned(ref owned) => owned.borrow()
        }
    }
}
```

This is an interesting definition! It says that if we have a Cow value, `some_cow_ptr`, and we dereference it as in `*some_cow_ptr`, the result is a reference to the borrowed version. Of course, this means that if `some_cow_ptr` is actually the `Owned` variant, TODO

[the-sized-trait]: http://huonw.github.io/blog/2015/01/the-sized-trait/
[v1-prelude]: https://doc.rust-lang.org/std/prelude/v1/
