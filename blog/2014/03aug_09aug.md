---
title: "A Rust curiosity: pointers to zero-sized types"
---

Today I was looking at the code for [core::slice::Items](http://static.rust-lang.org/doc/master/std/slice/struct.Items.html), a standard iterator over a vector slices, when I noticed something that seemed a bit odd. Here's the [block of code](https://github.com/rust-lang/rust/blob/f5ac41185a821681f4bfaf93ef0569955d24ef4a/src/libcore/slice.rs#L902-L915) I was looking at:

```rust
if mem::size_of::<T>() == 0 {
    // purposefully don't use 'ptr.offset' because for
    // vectors with 0-size elements this would return the
    // same pointer.
    self.ptr = transmute(self.ptr as uint + 1);

    // Use a non-null pointer value
    Some(transmute(1u))
} else {
    let old = self.ptr;
    self.ptr = self.ptr.offset(1);

    Some(transmute(old))
}
```

To understand what's going on, you need to know three things:

  1. The function `std::mem::size_of` returns the number of bytes a type takes up in memory. For example, `std::mem::size_of::<u8>()` returns `1` and `std::mem::size_of::<i32>()` returns `4`. 
  2. Certain types in Rust take up zero bytes. These types include `()`, the [unit type](http://static.rust-lang.org/doc/master/core/unit/index.html) and any unit structs, which are structs with no fields (such a struct is declared by `struct Foo;`).
  3. The `transmute` function above is [std::mem::transmute](http://static.rust-lang.org/doc/master/std/mem/fn.transmute.html). Its signature is `pub fn transmute<T, U>(T) -> U`. As far as I understand, it doesn't change anything about the value being transmuted, all it does is change its type. For example, one way to print out the memory address of a variable `x` in Rust is to transmute a `&x` to a `uint` and just print out the `uint`. As you might imagine, this operation is **HIGHLY DANGEROUS**.There's nothing to stop you from transmuting `0` into a borrow, `&T` and then trying to dereference it. Voila, null pointer dereferencing in Rust!

So the above code is checking whether the type of the elements that the slice contains is zero-sized or not. If it is, we advance the iterator to the next element and then return `Some(transmute(1u))`. This iterator is supposed to be returning a borrow, `&T`, so in effect this code says: when `size_of::<T>() == 0`, transmute `1` to a `&T` and return it.

This initially seemed nuts to me, because you literally have a pointer to memory address `1`. That's a segfault waiting to happen, surely? But I asked around in the Rust IRC channel and this was explained as follows: you don't actually need to follow any pointer to a type with zero size, because there's no bits in memory representing it. As long as you have the type, you have all you need to know about the value. In fact, it's not clear to me why this code insists on transmuting a non-zero value, because doing this seems to work just fine:

```rust
fn main() {
    let x: &() = unsafe { std::mem::transmute(0u) };
    println!("{}", x); // prints '()'
}
```

It looks like a pointer to a zero-sized type is never even followed, so it doesn't matter if its null or not!
