---
title: A bug in Rust's coherence rules
---

I ran into a Rust trait bug today. I was trying to implement an extremely generic solution to the problem of computing the sum of a tree of numbers (I actually generalized it to any type that implements Rust's [Add trait](http://static.rust-lang.org/doc/master/std/ops/trait.Add.html).) The code I wrote for that is [here](https://github.com/nham/hs_fri_job_prep/blob/master/11jul_recursion/add_tree.rs), but I don't want to discuss that example. Here's a contrived example instead. Suppose we have two traits, `Foo` and `Bar`:

```rust
trait Foo {
    fn foo(&self);
}

trait Bar {
    fn bar(&self);
}
```

The `Foo` trait has just one method, `foo`, and the `Bar` trait has just one method, `bar`. Suppose that for every type that implements Bar, we want to implement `Foo` in a standard way:

```rust
impl<T: Bar> Foo for T {
    fn foo(&self) { 
        println!("I'm a Bar!");
        self.bar() 
    }
}
```

This says that we implement Foo for every type `T` that implements `Bar` by printing "I'm a Bar!" and then calling `bar`. Simple enough.

Now suppose we define a type, `Thingy`, and try to implement `Foo` for it:

```rust
struct Thingy;

impl Foo for Thingy {
    fn foo(&self) {
        println!("I'm a thingy");
    }
}
```

(`struct Thingy;` just defines a struct with no fields). We get this compiler error:

    trait_problem.rs:9:1: 11:2 error: conflicting implementations for trait `Foo`
    trait_problem.rs:9 impl<T: Bar> Foo for T {
    trait_problem.rs:10     fn foo(&self) { self.bar() }
    trait_problem.rs:11 }
    trait_problem.rs:15:1: 19:2 note: note conflicting implementation here
    trait_problem.rs:15 impl Foo for Thingy {
    trait_problem.rs:16     fn foo(&self) {
    trait_problem.rs:17         println!("I'm a thingy");
    trait_problem.rs:18     }
    trait_problem.rs:19 }

This error is coming from rustc's coherence checker, which ensures that a trait is implemented for a type *no more than once*. I think the Rust compiler is saying this: "You can't implement Foo for Thingy. What if you later decide to implement Bar for Thingy! Then you'll have conflicting implementations for foo()". This is true, but it's also irrelevant, since we *don't actually implement Bar for Thingy*.

The Rust developers are fortunately aware of this problem, and they have recently [accepted an RFC](https://github.com/rust-lang/rfcs/blob/master/active/0024-traits.md) to overhaul the trait system. This overhaul, in part, should fix the problem, if I'm reading it correctly. See the section on [overly conservative coherence rules](https://github.com/rust-lang/rfcs/blob/master/active/0024-traits.md#overly-conservative-coherence). Also, that RFC has a better description of the problem, so if the above didn't make any sense you should read that instead!
