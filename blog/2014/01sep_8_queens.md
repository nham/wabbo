# Solving the 8 queens problem from scratch

I recently read a highly-upvoted comment on Reddit expressing exasperation that someone hiring for a software development role would ask a candidate to solve the 8 Queens problem on a whiteboard during an interview. Quoth the redditor:

 > The solution (on Wikipedia) is quite unintuitive and really just involves you knowing the problem and solution ahead [of] the interview.

I had heard of this problem, but never looked at or attempted to solve it before, so as an exercise I decided to see whether this claim was true. This is not quite the same setting (I'm sitting comfortably in my apartment, leisurely typing into a text editor and am not being scrutinized by a stranger in a setting that will impact my financial situation, for example), so even if this problem turns out to be not too hard to solve form scratch, it is still possible that this is a bad interview question. Here I'm mostly interested in whether the problem is really all that difficult to reason about and solve from scratch.

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

This is just barely better, but to go into more detail we'd need to decide on a representation for cells in the grid. Most likely we would choose something like pairs $(i, j)$ of numbers with $0 \leq i, j \leq 7$ to represent cells. To check whether two cells are in the same column, we would check whether their coordinates have the same second component (or something like that), and similarly to check whether they're in the same row we would see if the coordinates share the same first component. Checking diagonals is only slightly more involved: we subtract the coordinates and see whether we get a pair $(k, k)$ for some $k$.

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


## Systematically generating candidates

How can we improve on this? I'm not sure we can remove the final diagonal check in `verify_candidate`, since if each of our candidates adhered to that as well it'd already be a solution to the problem. The thing that feels wrong about what we have so far is the random element to it. It seems like we should be able (somehow) to systematically generate candidates.
