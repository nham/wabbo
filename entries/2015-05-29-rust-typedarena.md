---
title: Notes on Rust's `TypedArena`
tags: programming, rust
---

## Prerequisites

## `Cell`, `RefCell`

See [this post][cell] or the [API documentation][rust-std-cell].

### `PhantomData`

`[PhantomData][phantom-data]` is a [marker type][rust-std-marker] provided by the standard library.


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


```
struct TypedArenaChunk<T> {
    marker: marker::PhantomData<T>,

    /// Pointer to the next arena segment.
    next: *mut TypedArenaChunk<T>,

    /// The number of elements that this chunk can hold.
    capacity: usize,

    // Objects follow here, suitably aligned.
}

fn calculate_size<T>(capacity: usize) -> usize {
    let mut size = mem::size_of::<TypedArenaChunk<T>>();
    size = round_up(size, mem::min_align_of::<T>());
    let elem_size = mem::size_of::<T>();
    let elems_size = elem_size.checked_mul(capacity).unwrap();
    size = size.checked_add(elems_size).unwrap();
    size
}

impl<T> TypedArenaChunk<T> {
    #[inline]
    unsafe fn new(next: *mut TypedArenaChunk<T>, capacity: usize)
           -> *mut TypedArenaChunk<T> {
        let size = calculate_size::<T>(capacity);
        let chunk = allocate(size, mem::min_align_of::<TypedArenaChunk<T>>())
                    as *mut TypedArenaChunk<T>;
        if chunk.is_null() { alloc::oom() }
        (*chunk).next = next;
        (*chunk).capacity = capacity;
        chunk
    }
}



impl<T> TypedArena<T> {
    /// Creates a new `TypedArena` with preallocated space for eight objects.
    #[inline]
    pub fn new() -> TypedArena<T> {
        TypedArena::with_capacity(8)
    }

    /// Creates a new `TypedArena` with preallocated space for the given number of
    /// objects.
    #[inline]
    pub fn with_capacity(capacity: usize) -> TypedArena<T> {
        unsafe {
            let chunk = TypedArenaChunk::<T>::new(ptr::null_mut(), capacity);
            TypedArena {
                ptr: Cell::new((*chunk).start() as *const T),
                end: Cell::new((*chunk).end() as *const T),
                first: RefCell::new(chunk),
                _own: marker::PhantomData,
            }
        }
    }
}
```



[rust-std-cell]: https://doc.rust-lang.org/std/cell/
[rust-std-marker]: https://doc.rust-lang.org/std/marker/index.html
[phantom-data]: https://doc.rust-lang.org/std/marker/struct.PhantomData.html
