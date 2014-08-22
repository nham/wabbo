---
title: For loops in Rust without the standard library
---

### For loops and iterators in Rust
There is a language construct in Rust for `for` loops. They aren't like for loops from C. Instead, they work around [iterators](http://doc.rust-lang.org/std/iter/), which is a trait for types that yield some sequence of values. I will steal the description from the documentation because it's unlikely I would be able to improve on it:

  > An interface for dealing with "external iterators". These types of iterators can be resumed at any time as all state is stored internally as opposed to being located on the call stack.
  >
  > The Iterator protocol states that an iterator yields a (potentially-empty, potentially-infinite) sequence of values, and returns None to signal that it's finished. ...

You use for loops like this:

    for <variable name> in <iterator> {
        <some code that uses the variable from the first line>
    }

(Actually, it's not merely a variable that works right after the "for" keyword, but any pattern)

The key point is that the `for` loop in Rust is just syntactical sugar for some code that uses the iterator protocol. So if I have this vector of ints defined:

```rust
let values = vec![1i, 2, 3];
```

And I make a `for` loop like so:

```rust
// "Syntactical sugar" taking advantage of an iterator
for &x in values.iter() {
    println!("{}", x);
}
```

Then the Rust documentation says it gets translated into something like this:

```rust
// Rough translation of the iteration without a `for` iterator.
let mut it = values.iter();
loop {
    match it.next() {
        Some(&x) => {
            println!("{}", x);
        }
        None => { break }
    }
}
```

(The above example was shamelessly stolen from the [Rust documentation](http://doc.rust-lang.org/std/iter/index.html))


### Rust's standard prelude
There is a module in the Rust standard library, [std::prelude](http://doc.rust-lang.org/std/prelude/), which makes certain parts of the Rust standard library available to every Rust program. As the docs say:

  > Implicitly, all modules behave as if they contained the following prologue:
  >
  > use std::prelude::*;

However, there is a way to disable this behavior. We can use the `#![no_implicit_prelude]` [attribute](http://doc.rust-lang.org/rust.html#attributes). For example, if we try to run this code:


```rust
#![no_implicit_prelude]

fn main() {
    let v = vec!(1i, 3i, 5i, 7i);
    for i in v.iter() {
        println!("{}", i);
    }
}
```

then we [get](http://is.gd/wA1doM):

    <anon>:5:5: 8:2 error: unresolved enum variant, struct or const `Some`
    <anon>:5     for i in v.iter() {
    <anon>:6         println!("{}", i);
    <anon>:7     }
    <anon>:8 }
    error: aborting due to previous error
    playpen: application terminated with error code 101

This seems to be happening because an iterator returns an `Option` value, but we haven't imported the `Option` data constructors, which are `Some` and `None`. If we add `use std::option::{Some, None};`, we [get a different error](http://is.gd/3tFUHB):

    <anon>:7:5: 10:2 error: type `&mut core::slice::Items<'_,int>` does not 
    implement any method in scope named `next`
    <anon>:7     for i in v.iter() {
    <anon>:8         println!("{}", i);
    <anon>:9     }
    <anon>:10 }
    error: aborting due to previous error
    playpen: application terminated with error code 101

This error is happening because we didn't import the `Iterator` trait from the standard library. Adding `use std::iter::Iterator;` makes everything work as expected.

(Note that the macros `vec!` and `println!` still work despite disabling the prelude. I'm not sure what kind of magic powers these things)


### For loops without `Iterator`
The above struck me as a bit strange. For loops seem to be a language-level construct, but they rely on something defined in the standard library? 

I decided to see if I could use for loops without importing `std::iter::Iterator`, in attempt to see what makes for loops tick.

We'll start out by designing our own trait for iterators:

```rust
pub trait MyIterator<A> {
    fn next(&mut self) -> Option<A>;
}
```

Of course, the `Option` type is defined in the standard library, and we're trying to avoid importing from the standard library, so we'll need to define our own version of that. Let's call it `Mebbe` instead of `Option`, just for fun.

```rust
enum Mebbe<T> {
    Some(T),
    None,
}

pub trait MyIterator<A> {
    fn next(&mut self) -> Mebbe<A>;
}
```

We need to define some iterator to test with. An iterator that emits successive ints starting at some int and ending at another int is probably the simplest iterator I can think of:

```rust
struct Range {
    curr: int,
    stop: int, // exclusive of this
}

impl MyIterator<int> for Range {
    fn next(&mut self) -> Mebbe<int> {
        if self.curr == self.stop {
            None
        } else {
            let out = self.curr;
            self.curr += 1;
            Some(out)
        }
    }
}

impl Range {
    fn new(start: int, stop: int) -> Range {
        Range { curr: start, stop: stop }
    }
}
```

If we define a `Range` with `start = 2` and `stop = 7`, it should iterate through the values `2, 3, 4, 5, 6` and then stop. Adding a for loop to test and putting it all together, we have this code:


```rust
#![no_implicit_prelude]

enum Mebbe<T> {
    Some(T),
    None,
}

pub trait MyIterator<A> {
    fn next(&mut self) -> Mebbe<A>;
}

struct Range {
    curr: uint,
    stop: uint, // exclusive of this
}

impl MyIterator<uint> for Range {
    fn next(&mut self) -> Mebbe<uint> {
        if self.curr == self.stop {
            None
        } else {
            let next = self.curr;
            self.curr += 1;
            Some(next)
        }
    }
}

impl Range {
    fn new(start: uint, stop: uint) -> Range {
        Range { curr: start, stop: stop }
    }
}

fn main() {
    for i in Range::new(2, 7) {
        println!("{}", i);
    }
}
```

And...it [apparently works!](http://is.gd/CpxrdP)

So there's actually no reliance on `std::iter::Iterator` or `std::option::Option`, but what exactly is going on here? Is it a syntactical transformation where it literally emits the `loop` construct above, which repeatedly calls `next()` and checks whether the result is `Some` and `None`? Let's see what happens when we change the `Mebbe` type to:

```rust
enum Mebbe<T> {
    Yap(T),
    No,
}
```

(Remember, we also need to change Range's `next` implementation to use `Yap` and `No` instead of `Some` and `None`). When we do this, we get:

    <anon>:36:5: 39:2 error: unresolved enum variant, struct or const `Some`
    <anon>:36     for i in Range::new(2, 7) {
    <anon>:37         println!("{}", i);
    <anon>:38     }
    <anon>:39 }
    error: aborting due to previous error
    playpen: application terminated with error code 101

Also, if we change the name of `MyIterator`'s `next` method to anything else, we get this:

    <anon>:36:5: 39:2 error: type `&mut Range` does not implement any method in 
    scope named `next`
    <anon>:36     for i in Range::new(2, 7) {
    <anon>:37         println!("{}", i);
    <anon>:38     }
    <anon>:39 }
    error: aborting due to previous error
    playpen: application terminated with error code 101

If we make a different change and add another variant to the `Mebbe` enum:

    enum Mebbe<T> {
        Some(T),
        None,
        Foo,
    }

we get a new error because the match arm of the desugared code doesn't cover this new variant:

    <anon>:37:5: 40:2 error: non-exhaustive patterns: `Foo` not covered
    <anon>:37     for i in Range::new(2, 7) {
    <anon>:38         println!("{}", i);
    <anon>:39     }
    <anon>:40 }
    error: aborting due to previous error
    playpen: application terminated with error code 101

So it certainly seems that the transformation from above is exactly what's happening. We get code that attempts to call some method named `next()` and checks whether the result of that call matches a `Some` pattern or a `None` pattern. To confirm this, I guess we would need to delve into the Rust compiler, but I'm not sure I'm intrepid enough for that adventure.
