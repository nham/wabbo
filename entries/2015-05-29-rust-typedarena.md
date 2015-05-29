---
title: Notes on Rust's `TypedArena`
tags: programming, rust
---

## Prerequisites

## `Cell`, `RefCell`

See [this post][cell] or the [API documentation][rust-std-cell].

### `PhantomData`


```
/// A faster arena that can hold objects of only one type.
pub struct TypedArena<T> {
    /// A pointer to the next object to be allocated.
    ptr: Cell<*const T>,

    /// A pointer to the end of the allocated area. When this pointer is
    /// reached, a new chunk is allocated.
    end: Cell<*const T>,

    /// A pointer to the first arena segment.
    first: RefCell<*mut TypedArenaChunk<T>>,

    /// Marker indicating that dropping the arena causes its owned
    /// instances of `T` to be dropped.
    _own: marker::PhantomData<T>,
}
```

[rust-std-cell]: https://doc.rust-lang.org/std/cell/
