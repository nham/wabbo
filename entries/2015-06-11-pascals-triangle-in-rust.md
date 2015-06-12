---
title: Pascal's triangle in Rust
---

Introduce and define pascals triangle. Pose the 3 questions to be solved by code:

 - print out a specific the triangle through a specific row
 - print out a specific row
 - print out a a given element of a given row


Idea: define an iterator that yields rows of the element. Since each row in the triangle can generate the next row, we just need to seed it with some data when we construct a given iterator and every other row will be automatically generated.

```rust
struct Triangle {
    count: usize,
    last_row: Vec<u64>,
}

impl Iterator for Triangle {
    type Item = Vec<u64>;
    fn next(&mut self) -> Option<Self::Item> {
        let next = match count {
            0 => vec![0, 1, 0],
            _ => unimplemented!(),
        };

        self.count += 1;
        Some(next)
    }
}
```

```rust
struct Triangle {
    count: usize,
    last_row: Vec<u64>,
}

impl Iterator for Triangle {
    type Item = Vec<u64>;
    fn next(&mut self) -> Option<Self::Item> {
        let next = match count {
            0 => vec![0, 1, 0],
            1 => {
                let mut new = vec![];
                new.push(0);
                new.push(0 + 1);
                new.push(1 + 0);
                new.push(0);
                self.last_row = new.clone();
                new
            },
            2 => {
                let mut new = vec![];
                new.push(0);
                new.push(0 + 1);
                new.push(1 + 1);
                new.push(1 + 0);
                new.push(0);
                self.last_row = new.clone();
                new
            },
            3 => {
                let mut new = vec![];
                new.push(0);
                new.push(0 + 1);
                new.push(1 + 2);
                new.push(2 + 1);
                new.push(1 + 0);
                new.push(0);
                self.last_row = new.clone();
                new
            }
            count => {
                let mut new = vec![];
                new.push(0);
                for i in 0..(count + 1) {
                    new.push(self.last_row[i] + self.last_row[i+1]);
                }
                new.push(0);
                self.last_row = new.clone();
                new
            }
        };

        self.count += 1;
        Some(next)
    }
}
```
