---
title: Solving the 8 queens problem from scratch
---

I recently read a highly-upvoted comment on Reddit expressing exasperation that someone hiring for a software development role would ask a candidate to solve the 8 Queens problem on a whiteboard during an interview. Quoth the redditor:

 > The solution (on Wikipedia) is quite unintuitive and really just involves you knowing the problem and solution ahead [of] the interview.

I had heard of this problem, but never looked at or attempted to solve it before, so as an exercise I decided to see whether this claim was true. This is not quite the same setting (I'm sitting comfortably in my apartment, leisurely typing into a text editor and am not being scrutinized by a stranger in a setting that will impact my livelihood), so even if this problem turns out to be not too difficult, it is still possible that this is a bad interview question. Here I'm mostly interested in whether the problem is difficult to reason about and solve from scratch.

## The problem

We have an 8x8 grid, and we want to write an algorithm that selects 8 of the cells in the grid such that no two cells share the same row, column or diagonal.

## The dumbest possible solution

That is certainly a doozy of a problem. I hardly know where to begin! My instinct is to write out the dumbest possible solution and then iterate on it to see where I can take it. With that in mind, here's the dumbest solution I can think of (I'd be interested in knowing if there are dumber solutions):

```
def queens() {
    loop forever {
        select 8 cells at random
        if the selection is a solution, return it
    }
}
```

This is certainly a solution. It's a really bad one, obviously, so let's fix it up. The first defect is that the operation that checks whether a set of 8 cells is a solution has not been described in detail. Let's write out that pseudocode first.

```
def verify_candidate(cells) {
    for all pairs (a, b) of cells (with a != b) {
        if a and b are in the same column, return false
        if a and b are in the same row, return false
        if a and b are in the same diagonal, return false
    }
    return true
}

def queens() {
    loop forever {
        cs = select 8 cells at random
        if verify_candidate(cs) is true, return cs
    }
}
```

This is just barely better, but to go into more detail we'd need to decide on a representation for cells in the grid. Most likely we would choose something like pairs (i, j) of numbers with 0 <= i, j <= 7 to represent cells. To check whether two cells are in the same column, we would check whether their coordinates have the same second component (or something like that), and similarly to check whether they're in the same row we would see if the coordinates share the same first component. Checking diagonals is only slightly more involved: we subtract the coordinates and see whether we get a pair (k, k) or (k, -k) for some k.

At this point I feel confident that I could write working code for the dumbest possible algorithm, so let's give it an upgrade and make it slightly less dumb.

## A slightly less dumb solution

Generating a random collection of 8 cells is not too great, especially when we know, for example, that they all have to be on different rows. We could start out by generating one cell in each row instead:

```
def queens() {
    loop forever {
        cs = empty set or something
        for i = 0 to 7 {
            select a random cell from row i
            add the cell to cs
        }

        if verify_candidate(cs) is true, return cs
    }
}
```

In the above we're using the same `verify_candidate` routine before, but actually it's doing a bit too much work, since by design no two cells will be in the same row. We could update it to the following:


```
def verify_candidate(cells) {
    for all pairs (a, b) of cells (with a != b) {
        if a and b are in the same column, return false
        if a and b are in the same diagonal, return false
    }
    return true
}
```

This gives me a vague idea about where to head in terms of improving the algorithm. We made our algorithm smarter by taking some of the work that was being done in `verify_candidate` and pushing it into the main `queens` routine instead. It seems like we should try to keep moving in this direction. Can we remove the logic that checks whether two pairs are in the same column from `verify_candidate`?

In order to do that, it seems we need to have the main loop in `queens` generate only candidates (sets of cells) where no two pairs share the same row or column (but diagonal sharing is allowed). I think I see how we can do that:

```
def queens() {
    loop forever {
        cols = {0, 1, ..., 7}
        cs = empty set or something
        for i = 0 to 7 {
            remove some (random) j from cols
            add (i, j) to cs
        }

        if verify_candidate(cs) is true, return cs
    }
}
```

On each iteration of the loop, have a list of columns that haven't been used yet. For each row we remove, at random, one column from the list. This gives us what we wanted. We can modify `verify_candidate` now to remove the "same column" check, since it's no longer needed:

```
def verify_candidate(cells) {
    for all pairs (a, b) of cells (with a != b) {
        if a and b are in the same diagonal, return false
    }
    return true
}
```

This seems a lot nicer than what we started out with. We're still randomly generating solutions, but what we do generate is constrained enough that we won't waste a bunch of time checking obviously wrong solutions.


## Dead Cthulhu waits dreaming (of recursion)

How can we improve on this? I'm not sure we can remove the final diagonal check in `verify_candidate`, since if each of our candidates adhered to that as well it'd already be a solution to the problem. The thing that feels wrong about what we have so far is the random element to it, primarily because we might randomly generate the same candidate more than once. It's obviously a waste of time to check a candidate more than once. It seems like we should try to modify the algorithm to systematically generate candidates to test.

Let's briefly consider the 4-queens problem, which is just the 8-queens problem with every instance of "8" replaced by "4". We could systematically generate candidates in this order:

  (0, 0)
  (1, 1)
  (2, 2)
  (3, 3)

  (0, 0)
  (1, 1)
  (2, 3)
  (3, 2)

  (0, 0)
  (1, 2)
  (2, 1)
  (3, 3)

  (0, 0)
  (1, 2)
  (2, 3)
  (3, 1)

  (0, 0)
  (1, 3)
  (2, 1)
  (3, 2)

  ...

In words, we generate all permutations of (0,1,2,3) and use those as the columns in our algorithm. This would certainly eliminate the randomness. It's kind of hard to explain what I mean here, but hopefully the pseudocode makes sense:

```
def queens() {
    for each permutation p of (0, 1, ..., 7) {
        cs = empty set or something
        for i = 0 to 7 {
            add (i, p[i]) to cs
        }

        if verify_candidate(cs) is true, return cs
    }
}
```

`verify_candidate` is the same as before. Alright! So this seems like progress. We've eliminated the randomness from our algorithm. It's still a bit dumb though. We don't actually want to consider *every* permutation, for example any permutation starting with (0, 1, ...) will not work, since (0, 0) and (1, 1) lie in the same diagonal. It'd be nice if we could somehow detect that and skip over that class of permutations.

This is going to involve modifying the permutation-generation module, so we should probably expand on how exactly that gets done.


```
def queens() {
    for a = 0 to 7 {
     for b = 0 to 7 but not a {
      for c = 0 to 7 but not a, b {
       for d = 0 to 7 but not a, b, c {
        for e = 0 to 7 but not a, b, c, d {
         for f = 0 to 7 but not a, b, c, d, e {
          for g = 0 to 7 but not a, b, c, d, e, f {
           for h = 0 to 7 but not a, b, c, d, e, f, g {
               p = (a, b, c, d, e, f, g, h)
               cs = empty set or something
               for i = 0 to 7 {
                   add (i, p[i]) to cs
               }

               if verify_candidate(cs) is true, return cs
           }
          }
         }
        }
       }
      }
     }
    }
}
```

I know, I know, this code is horrible, but it should be easy enough to understand. Now we can try to modify it to skip over bad classes of permutations. It seems like as soon as we detect, for example, that a = 0 and b = 1, then we should be able to break out of that second for loop and skip to a = 1, since none of the permutations that start out (0, 1, ...) will work, as mentioned above. But how can we detect bad permutations in general?

My instinct here was to play around with the 4-queens problem to find an example. However the 4 queens problem seemed to be a bit too small, so I went to the 5 queens problem instead and quickly found this example:

    |_|#|_|_|_|
    |_|_|_|#|_|
    |#|_|_|_|_|
    |_|_|_|_|#|
    |_|_|#|_|_|

This corresponds to the column permutation of (1, 3, 0, 4, 2). There are two diagonal conflicts in this: (0, 1) and (3, 4) are in the same diagonal, as are (2, 0) and (4, 2).

In the code we would assign a = 1, b = 3, c = 0, and then come to the fourth for loop and try to assign d = 4. However, what we want is some logic that realizes theres no point in doing that because we have a diagonal conflict and continues with the next value of c.

Remember how we detected diagonals above? If (a, b) and (c, d) are the coordinates of two cells and (a, b) - (c, d) results in (k, k), then we have a diagonal conflict. Well, we basically need to do that, but we need to do it with the column permutation instead. In the case of (1, 3, 0, 4), we should be able to see that the index of column "1" is 0 and the index of column "4" is 3, and 4 - 1 = 3 - 0, so they lie in the same diagonal. This should probably be implemented like this (brace yourself):

```
def queens() {
    p = new vector (dynamic array)
    for a = 0 to 7 {
     p.push(a)

     for b = 0 to 7 but not a {
      p.push(b)

      if diagonal_conflict(p) is true {
         p.pop()
         continue
      }

      for c = 0 to 7 but not a, b {
       p.push(c)

       if diagonal_conflict(p) is true {
          p.pop()
          continue
       }

       for d = 0 to 7 but not a, b, c {
        p.push(d)

        if diagonal_conflict(p) is true {
           p.pop()
           continue
        }

        for e = 0 to 7 but not a, b, c, d {
         p.push(e)

         if diagonal_conflict(p) is true {
            p.pop()
            continue
         }

         for f = 0 to 7 but not a, b, c, d, e {
          p.push(f)

          if diagonal_conflict(p) is true {
             p.pop()
             continue
          }

          for g = 0 to 7 but not a, b, c, d, e, f {
           p.push(g)

           if diagonal_conflict(p) is true {
              p.pop()
              continue
           }

           for h = 0 to 7 but not a, b, c, d, e, f, g {
             p.push(h)

             if diagonal_conflict(p) is true {
                p.pop()
                continue
             }

             cs = empty set or something
             for i = 0 to 7 {
                 add (i, p[i]) to cs
             }

             return cs
           }
           p.pop()
          }
          p.pop()
         }
         p.pop()
        }
        p.pop()
       }
       p.pop()
      }
      p.pop()
     }
     p.pop()
    }
}
```

I want to kill this Cthulian monstrosity of a function with fire, but it accomplishes something important: the `verify_candidate` function is gone! We need to write `diagonal_conflict`, but that's not too hard:

```
def diagonal_conflict(v) {
    for i = 0 to (v.length - 1) {
        for j = (i+1) to (v.length - 1) {
            col_diff = v[j] - v[i]
            if col_diff == j - i or col_diff == i - j {
                return true
            }
        }
    }
    return false
}
```

Above we're testing if `v[j] - v[i]` is equal to either `j - i` or `i - j` since we need to check both diagonals.

This seems like it will work, but it's just awful code. How can we make it not so horrible? Notice that the for loops are basically all the same. I smell recursion.


```
def foo(p, set) {
    if set is not empty {
        for i in set {
            p.push(i)

            if diagonal_conflict(p) is true {
               p.pop()
               continue
            }

            set2 = a copy of set, but with i removed
            x = foo(p, set2)
            if x is None {
                p.pop()
            } else {
                return x
            }
        }
        return None
    } else {
         // we have a solution, so just return it!
         cs = empty set or something
         for i = 0 to 7 {
             add (i, p[i]) to cs
         }

         return cs
    }
}
```

```
def queens() {
    p = new vector (dynamic array)
    cols = {0, 1, ..., 7}

    return foo(p, cols)
}
```

I'm not really sure what to call `foo`, but hopefully what I've done here is clear. The `set` parameter to `foo` is a collection of available column indices.

## Actual code

The above algorithm is where I end my investigations. I have no doubt it can be pushed farther, but I want to verify that  haven't gone totally off the deep end by actually coding and running the solution I arrived at. For that I will use my current drug of choice, Rust!

What follows is a direct translation of the code into Rust, with a slight modification: the code generates all possible solutions in order to verify that it is correct.

```rust
use std::collections::HashSet;

type Solution = Vec<(uint, uint)>;

fn foo(p: &mut Vec<uint>, set: HashSet<uint>, sols: &mut Vec<Solution>) {
    if !set.is_empty() {
        for &i in set.iter() {
            p.push(i);

            if diagonal_conflict(p) {
                p.pop();
                continue;
            }

            let mut set2 = set.clone();
            set2.remove(&i);

            foo(p, set2, sols);
            p.pop();
        }
    } else {
        let mut sol = vec!();
        for i in range(0u, 8) {
            sol.push((i, (&*p)[i]));
        }

        sols.push(sol);
    }
}


fn diagonal_conflict(v: &Vec<uint>) -> bool {
    for i in range(0, v.len()) {
        for j in range(i+1, v.len()) {
            let col_diff = v[j] - v[i];
            if col_diff == j - i || col_diff == i - j {
                return true;
            }
        }
    }
    false
}

fn main() {
    let mut p = vec!();
    let cols: HashSet<uint> = range(0, 8).collect();
    let mut sols = vec!();

    foo(&mut p, cols, &mut sols);
    for i in sols.iter() {
        println!("{}", i);
    }
}
```

And the output (there are 92 solutions, so I'm assuming this is correct?):

    [(0, 3), (1, 5), (2, 7), (3, 1), (4, 6), (5, 0), (6, 2), (7, 4)]
    [(0, 3), (1, 5), (2, 7), (3, 2), (4, 0), (5, 6), (6, 4), (7, 1)]
    [(0, 3), (1, 5), (2, 0), (3, 4), (4, 1), (5, 7), (6, 2), (7, 6)]
    [(0, 3), (1, 6), (2, 4), (3, 1), (4, 5), (5, 0), (6, 2), (7, 7)]
    [(0, 3), (1, 6), (2, 4), (3, 2), (4, 0), (5, 5), (6, 7), (7, 1)]
    [(0, 3), (1, 6), (2, 0), (3, 7), (4, 4), (5, 1), (6, 5), (7, 2)]
    [(0, 3), (1, 6), (2, 2), (3, 7), (4, 1), (5, 4), (6, 0), (7, 5)]
    [(0, 3), (1, 1), (2, 6), (3, 4), (4, 0), (5, 7), (6, 5), (7, 2)]
    [(0, 3), (1, 1), (2, 6), (3, 2), (4, 5), (5, 7), (6, 4), (7, 0)]
    [(0, 3), (1, 1), (2, 6), (3, 2), (4, 5), (5, 7), (6, 0), (7, 4)]
    [(0, 3), (1, 1), (2, 4), (3, 7), (4, 5), (5, 0), (6, 2), (7, 6)]
    [(0, 3), (1, 1), (2, 7), (3, 5), (4, 0), (5, 2), (6, 4), (7, 6)]
    [(0, 3), (1, 1), (2, 7), (3, 4), (4, 6), (5, 0), (6, 2), (7, 5)]
    [(0, 3), (1, 7), (2, 4), (3, 2), (4, 0), (5, 6), (6, 1), (7, 5)]
    [(0, 3), (1, 7), (2, 0), (3, 4), (4, 6), (5, 1), (6, 5), (7, 2)]
    [(0, 3), (1, 7), (2, 0), (3, 2), (4, 5), (5, 1), (6, 6), (7, 4)]
    [(0, 3), (1, 0), (2, 4), (3, 7), (4, 5), (5, 2), (6, 6), (7, 1)]
    [(0, 3), (1, 0), (2, 4), (3, 7), (4, 1), (5, 6), (6, 2), (7, 5)]
    [(0, 5), (1, 3), (2, 6), (3, 0), (4, 7), (5, 1), (6, 4), (7, 2)]
    [(0, 5), (1, 3), (2, 6), (3, 0), (4, 2), (5, 4), (6, 1), (7, 7)]
    [(0, 5), (1, 3), (2, 1), (3, 7), (4, 4), (5, 6), (6, 0), (7, 2)]
    [(0, 5), (1, 3), (2, 0), (3, 4), (4, 7), (5, 1), (6, 6), (7, 2)]
    [(0, 5), (1, 1), (2, 6), (3, 0), (4, 3), (5, 7), (6, 4), (7, 2)]
    [(0, 5), (1, 1), (2, 6), (3, 0), (4, 2), (5, 4), (6, 7), (7, 3)]
    [(0, 5), (1, 7), (2, 1), (3, 3), (4, 0), (5, 6), (6, 4), (7, 2)]
    [(0, 5), (1, 0), (2, 4), (3, 1), (4, 7), (5, 2), (6, 6), (7, 3)]
    [(0, 5), (1, 2), (2, 6), (3, 3), (4, 0), (5, 7), (6, 1), (7, 4)]
    [(0, 5), (1, 2), (2, 6), (3, 1), (4, 3), (5, 7), (6, 0), (7, 4)]
    [(0, 5), (1, 2), (2, 6), (3, 1), (4, 7), (5, 4), (6, 0), (7, 3)]
    [(0, 5), (1, 2), (2, 4), (3, 6), (4, 0), (5, 3), (6, 1), (7, 7)]
    [(0, 5), (1, 2), (2, 4), (3, 7), (4, 0), (5, 3), (6, 1), (7, 6)]
    [(0, 5), (1, 2), (2, 0), (3, 6), (4, 4), (5, 7), (6, 1), (7, 3)]
    [(0, 5), (1, 2), (2, 0), (3, 7), (4, 3), (5, 1), (6, 6), (7, 4)]
    [(0, 5), (1, 2), (2, 0), (3, 7), (4, 4), (5, 1), (6, 3), (7, 6)]
    [(0, 6), (1, 3), (2, 1), (3, 4), (4, 7), (5, 0), (6, 2), (7, 5)]
    [(0, 6), (1, 3), (2, 1), (3, 7), (4, 5), (5, 0), (6, 2), (7, 4)]
    [(0, 6), (1, 1), (2, 3), (3, 0), (4, 7), (5, 4), (6, 2), (7, 5)]
    [(0, 6), (1, 1), (2, 5), (3, 2), (4, 0), (5, 3), (6, 7), (7, 4)]
    [(0, 6), (1, 4), (2, 2), (3, 0), (4, 5), (5, 7), (6, 1), (7, 3)]
    [(0, 6), (1, 0), (2, 2), (3, 7), (4, 5), (5, 3), (6, 1), (7, 4)]
    [(0, 6), (1, 2), (2, 7), (3, 1), (4, 4), (5, 0), (6, 5), (7, 3)]
    [(0, 6), (1, 2), (2, 0), (3, 5), (4, 7), (5, 4), (6, 1), (7, 3)]
    [(0, 1), (1, 3), (2, 5), (3, 7), (4, 2), (5, 0), (6, 6), (7, 4)]
    [(0, 1), (1, 5), (2, 7), (3, 2), (4, 0), (5, 3), (6, 6), (7, 4)]
    [(0, 1), (1, 5), (2, 0), (3, 6), (4, 3), (5, 7), (6, 2), (7, 4)]
    [(0, 1), (1, 6), (2, 4), (3, 7), (4, 0), (5, 3), (6, 5), (7, 2)]
    [(0, 1), (1, 6), (2, 2), (3, 5), (4, 7), (5, 4), (6, 0), (7, 3)]
    [(0, 1), (1, 4), (2, 6), (3, 3), (4, 0), (5, 7), (6, 5), (7, 2)]
    [(0, 1), (1, 4), (2, 6), (3, 0), (4, 2), (5, 7), (6, 5), (7, 3)]
    [(0, 1), (1, 7), (2, 5), (3, 0), (4, 2), (5, 4), (6, 6), (7, 3)]
    [(0, 4), (1, 6), (2, 3), (3, 0), (4, 2), (5, 7), (6, 5), (7, 1)]
    [(0, 4), (1, 6), (2, 1), (3, 3), (4, 7), (5, 0), (6, 2), (7, 5)]
    [(0, 4), (1, 6), (2, 1), (3, 5), (4, 2), (5, 0), (6, 3), (7, 7)]
    [(0, 4), (1, 6), (2, 1), (3, 5), (4, 2), (5, 0), (6, 7), (7, 3)]
    [(0, 4), (1, 6), (2, 0), (3, 3), (4, 1), (5, 7), (6, 5), (7, 2)]
    [(0, 4), (1, 6), (2, 0), (3, 2), (4, 7), (5, 5), (6, 3), (7, 1)]
    [(0, 4), (1, 1), (2, 3), (3, 5), (4, 7), (5, 2), (6, 0), (7, 6)]
    [(0, 4), (1, 1), (2, 3), (3, 6), (4, 2), (5, 7), (6, 5), (7, 0)]
    [(0, 4), (1, 1), (2, 5), (3, 0), (4, 6), (5, 3), (6, 7), (7, 2)]
    [(0, 4), (1, 1), (2, 7), (3, 0), (4, 3), (5, 6), (6, 2), (7, 5)]
    [(0, 4), (1, 7), (2, 3), (3, 0), (4, 6), (5, 1), (6, 5), (7, 2)]
    [(0, 4), (1, 7), (2, 3), (3, 0), (4, 2), (5, 5), (6, 1), (7, 6)]
    [(0, 4), (1, 0), (2, 3), (3, 5), (4, 7), (5, 1), (6, 6), (7, 2)]
    [(0, 4), (1, 0), (2, 7), (3, 3), (4, 1), (5, 6), (6, 2), (7, 5)]
    [(0, 4), (1, 0), (2, 7), (3, 5), (4, 2), (5, 6), (6, 1), (7, 3)]
    [(0, 4), (1, 2), (2, 7), (3, 3), (4, 6), (5, 0), (6, 5), (7, 1)]
    [(0, 4), (1, 2), (2, 0), (3, 5), (4, 7), (5, 1), (6, 3), (7, 6)]
    [(0, 4), (1, 2), (2, 0), (3, 6), (4, 1), (5, 7), (6, 5), (7, 3)]
    [(0, 7), (1, 3), (2, 0), (3, 2), (4, 5), (5, 1), (6, 6), (7, 4)]
    [(0, 7), (1, 1), (2, 3), (3, 0), (4, 6), (5, 4), (6, 2), (7, 5)]
    [(0, 7), (1, 1), (2, 4), (3, 2), (4, 0), (5, 6), (6, 3), (7, 5)]
    [(0, 7), (1, 2), (2, 0), (3, 5), (4, 1), (5, 4), (6, 6), (7, 3)]
    [(0, 0), (1, 5), (2, 7), (3, 2), (4, 6), (5, 3), (6, 1), (7, 4)]
    [(0, 0), (1, 6), (2, 3), (3, 5), (4, 7), (5, 1), (6, 4), (7, 2)]
    [(0, 0), (1, 6), (2, 4), (3, 7), (4, 1), (5, 3), (6, 5), (7, 2)]
    [(0, 0), (1, 4), (2, 7), (3, 5), (4, 2), (5, 6), (6, 1), (7, 3)]
    [(0, 2), (1, 5), (2, 3), (3, 1), (4, 7), (5, 4), (6, 6), (7, 0)]
    [(0, 2), (1, 5), (2, 3), (3, 0), (4, 7), (5, 4), (6, 6), (7, 1)]
    [(0, 2), (1, 5), (2, 1), (3, 6), (4, 4), (5, 0), (6, 7), (7, 3)]
    [(0, 2), (1, 5), (2, 1), (3, 6), (4, 0), (5, 3), (6, 7), (7, 4)]
    [(0, 2), (1, 5), (2, 1), (3, 4), (4, 7), (5, 0), (6, 6), (7, 3)]
    [(0, 2), (1, 5), (2, 7), (3, 1), (4, 3), (5, 0), (6, 6), (7, 4)]
    [(0, 2), (1, 5), (2, 7), (3, 0), (4, 3), (5, 6), (6, 4), (7, 1)]
    [(0, 2), (1, 5), (2, 7), (3, 0), (4, 4), (5, 6), (6, 1), (7, 3)]
    [(0, 2), (1, 6), (2, 1), (3, 7), (4, 5), (5, 3), (6, 0), (7, 4)]
    [(0, 2), (1, 6), (2, 1), (3, 7), (4, 4), (5, 0), (6, 3), (7, 5)]
    [(0, 2), (1, 4), (2, 6), (3, 0), (4, 3), (5, 1), (6, 7), (7, 5)]
    [(0, 2), (1, 4), (2, 1), (3, 7), (4, 5), (5, 3), (6, 6), (7, 0)]
    [(0, 2), (1, 4), (2, 1), (3, 7), (4, 0), (5, 6), (6, 3), (7, 5)]
    [(0, 2), (1, 4), (2, 7), (3, 3), (4, 0), (5, 6), (6, 1), (7, 5)]
    [(0, 2), (1, 7), (2, 3), (3, 6), (4, 0), (5, 5), (6, 1), (7, 4)]
    [(0, 2), (1, 0), (2, 6), (3, 4), (4, 7), (5, 1), (6, 3), (7, 5)]


In fact, we don't need to assume anything. We can implement the verification function from earlier and run it on each solution:


```rust
fn verify(v: &Vec<(uint, uint)>) -> bool {
    for i in range(0, v.len()) {
        for j in range(i+1, v.len()) {
            if v[i].val0() == v[j].val0() {
                return false;
            } else if v[i].val1() == v[j].val1() {
                return false;
            } else {
                let (x, y) = ((v[j].val0() - v[i].val0()) as int,
                              (v[j].val1() - v[i].val1()) as int);
                if x.abs() == y.abs() {
                    return false;
                }
            }
        }
    }
    true
}
```

When you call this for each of the solutions above, you get `true`. So it seems to work.


## Checking Wikipedia

I decided to consult Wikipedia to see how my solution matched up to other solutions. If you look on the [8 queens](http://en.wikipedia.org/wiki/Eight_queens_puzzle) page, you can see this:

 > One algorithm solves the eight rooks puzzle by generating the permutations of the numbers 1 through 8 (of which there are 8! = 40,320), and uses the elements of each permutation as indices to place a queen on each row....A further improvement, which examines only 5,508 possible queen placements, is to combine the permutation based method with the early pruning method: the permutations are generated depth-first, and the search space is pruned if the partial permutation produces a diagonal attack.

This sounds like the solution I came up with. Yay?

## Conclusion

The problem isn't impossible to solve if you've never seen it, as I hope I've illustrated above. However, I'm somewhat doubtful I could have done this in an interview, especially if I was expected to come up with this in an hour or less.
