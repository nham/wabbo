---
title: Fuzzy string matching through dynamic programming
date: March 6, 2015
tags: programming, math
---

I've been studying math recently, and order to organize my studies I built a small command-line tool called [hippo](https://github.com/nham/hippo). All it does is schedule review of items (definitions, proofs of theorems, ideas) using [spaced repetition](http://en.wikipedia.org/wiki/Spaced_repetition). One of the commands, the `list` command, shows items that match a particular pattern. So if I type `hippo list interior`, it will give me a list of all items containing the string "interior", which currently looks like this:

     21 : def interior of subset of top. space
     23 : proof open subsets are those equal to their interior
     26 : proof interior is largest open subset
     30 : proof closure is disjoint union of interior and boundary
     34 : proof for any set S the whole space is partitioned into interior/boundary/exterior
     38 : example isolated points may either be boundary or interior points
     39 : example limit points may either be boundary or interior points
     46 : example why isn't nowhere dense defined to be sets with empty interior
     69 : proof interior of the complement is the complement of the closure

Unfortunately this doesn't work quite as well as I expected, because I discovered that I'm not always consistent about how I enter terms into the system. When I do `hippo list hausdorff`, I get:

     57 : proof limits of sequences are unique in hausdorff spaces
     58 : closure point proof finite subsets of hausdorff spaces are closed
     71 : union of open neighborhoods proof that finite subsets of hausdorff spaces are closed

which I can tell has a few items missing, notably what the definition of a Hausdorff space even is. This is because for the remaining items I used "Hausdorff", with a capital H, and not "hausdorff". Shucks.

One way to fix this is to go through all of the items that use "hausdorff" and change them to "Hausdorff", and then remember that I need to search with capital-H. This is not a great solution, however, because I'd like to be as lazy as I can possibly get away with. Another solution is to add an option for case-insensitive pattern matching, which I may actually yet do since it might be useful. However, after some thought, I realized that there is a feature that would fix this particular problem and a class of other problems I have with searching: fuzzy string matching. I was curious about how this works as well, since I'd never seen any fuzzy string matching algorithms.

The algorithm I describe below is basically the naive dynamic programming-based algorithm. The strings I'm working with are tiny, so the optimal, state-of-the-art algorithms for approximate string matching are huge overkill. For information on those you might want to look at *Flexible Pattern Matching in Strings: Practical On-Line Search Algorithms for Texts and Biological Sequences*, available online [here](http://nlp.cs.swarthmore.edu/~richardw/cs93-f07/flexiblePatternMatchingInStrings.pdf).

## Levenshtein metric

The approach below will hinge on being able to compute some kind of distance between strings. There are many possible choices for such a distance function, but a popular choice is the **Levenshtein distance**, also called the **edit distance**. We denote this function $ed$, and for any strings $s$ and $t$ define $ed(s, t)$ to be the length of the shortest sequence of operations (called an **edit sequence**) that turns $s$ into $t$. The allowed operations are:

  - inserting a symbol into $s$ ("metric" => "meatric")
  - deleting a symbol from $s$ ("chat" => "cat")
  - replacing one symbol in $s$ with another ("hello" => "jello")

The formal definition of this function is a bit complicated, but I'm going to give it anyway and you're just going to have to deal with it. A bit of notation: for any string $s$, $s_i$ denotes the $i$-th symbol in $s$, starting at $1$. So $s = s_1 \cdots s_n$ for some $n$ and some symbols $s_i$. I also denote the length of $s$ (i.e. $n$ in the last example) by $|s|$, and substrings $s_i \cdots s_j$ by $s_{i..j}$.

For any strings $s, t$, let's define a function $L_{s,t}: |s| \times |t| \to \mathbb{N}$ so that $L_{s, t}(i, j)$ is the length of the smallest edit sequence that turns $s_{1..i}$ into $t_{1..j}$. The formal definition (brace yourself) is as follows:

$$L_{s,t}(i, j) := \begin{cases}
    j & i = 0 \\
    i & j = 0 \\
    \min \begin{cases}
        L_{s,t}(i-1, j) + 1 \\
        L_{s,t}(i, j-1) + 1 \\
        L_{s,t}(i-1, j-1) + \delta(s_i, t_j)
    \end{cases} & \text{ otherwise }
\end{cases}$$

where $\delta$ is defined so that $\delta(x, y) = 1$ iff $x = y$. We then define

$$ed(s, t) := L_{s,t}(|s|, |t|)$$

Once you get past all the symbols, this really isn't so bad. It says that

  - the shortest edit sequence that turns the empty string into $t$ is just inserting all the symbols of $t$
  - the shortest edit sequence that turns $s$ into the empty string is deleting all the symbols of $s$
  - when $s_{1..i}$ and $t_{1..j}$ are non-empty prefixes of $s$ and $t$, respectively, then the shortest edit sequence turning $s_{1..i}$ into $t_{1..j}$ is the smallest of these three:

     1. The shortest sequence turning $s_{1..i-1}$ into $t_{1..j}$ followed by deletion of $s_i$
     2. The shortest sequence turning $s_{1..i}$ into $t_{1..j-1}$ followed by insertion of $t_j$.
     3. If $s_i = t_j$, we can just use the shortest sequence turning $s_{1..i-1}$ into $t_{1..j-1}$. Otherwise, it's that sequence followed by replacing $s_i$ with $t_j$.

Hopefully I've convinced you this is a well-defined function. But are we justified in calling this a "distance" function? Does it have all the properties we would expect such a function to have? What even is distance?

Actually, mathematicians have an answer for that last question. What we're really asking here is whether the Levenshtein distance is a [metric](http://en.wikipedia.org/wiki/Metric_%28mathematics%29) on the collection of strings over some alphabet. To verify this we need to prove that $ed$ obeys these properties:

  - positive-definite: $ed(s,t) \geq 0$ for all $s, t$ and $ed(s,t) = 0$ iff $s = t$.
  - symmetric: $ed(s,t) = ed(t,s)$ for all $s$ and $t$
  - transitive: $ed(s, u) \leq ed(s, t) + ed(t, u)$

By definition we have non-negativity, and clearly the only way we can get an edit sequence of length 0 is for the strings to be identical. Transitivity is also easy to see: the smallest edit sequence taking $s$ to $t$ concatenated with the smallest edit sequence taking $t$ to $u$ is in fact an edit sequence of length $ed(s,t) + ed(t, u)$ that takes $s$ to $u$, and we've defined $ed(s, u)$ to be the *smallest* edit sequence that does that.

The symmetricity of $ed$ is not much harder to prove, but the proof I have seems a bit clunky: the idea is that for any operation on $s$, there's a well-defined inverse operation on $t$. So if we insert a symbol $a$ into $s$ at some point, the inverse is deleting $a$ from $t$, and vice versa for deleting a symbol from $s$. The inverse of replacing symbol $a$ in $s$ with symbol $b$ is to replace symbol $b$ in $t$ with symbol $a$. So for any edit sequence turning $s$ into $t$, reverse it and find the inverse of each operation to obtain an edit sequence turning $t$ into $s$.

(I would like to point out that since $ed$ is a metric, the collection of all strings equipped with $ed$ is a metric space. But by basic topology all metric spaces are Hausdorff, so we're right back where we started. Topology is destiny.)

There's a way to generalize the Levenshtein metric as well. Instead of talking about the length of the edit sequence, we can assign each operation type some non-negative cost (provided at least one of the costs is positive). Then $ed(s,t)$ is defined to be the minimal cost of all edit sequences taking $s$ to $t$. (I'm not going to explore this, but I thought it was worth mentioning.)

## Implementing the Levenshtein metric

Since this essay is ostensibly about programming, probably we should do some of that. First I'll present a straightforward and unbearably slow implementation of the Levenshtein metric, and then we'll modify it to be actually usable.

Here is my attempt at translating the math above into Rust code:

```rust
fn ed<'a, 'b>(s: &'a [char], t: &'b [char]) -> usize {
    let (i, j) = (s.len(), t.len());
    if i == 0 {
        j
    } else if j == 0 {
        i
    } else {
        let (a, b) = (i-1, j-1);
        let v = vec![
            ed(&s[..a], &t[..b]) + if s[a] == t[b] { 0 } else { 1 },
            ed(&s[..a], t) + 1,
            ed(s, &t[..b]) + 1
        ];
        v.into_iter().min().unwrap()
    }
}
```

A couple of differences and technical notes: I'm using slices of chars rather than strings because strings in Rust are UTF-8, which makes going to the previous char in a string a bit tricky (there's a way to do it, but the code was less readable). This means you have to convert any strings to a slice of chars in order to use this. I ended up defining a simple conversion function for this:

```rust
fn lev<'a, 'b>(s: &'a str, t: &'b str) -> usize {
    let s_chars: Vec<char> = s.chars().collect();
    let t_chars: Vec<char> = t.chars().collect();
    ed(&s_chars[..], &t_chars[..])
}
```

To calculate the min I'm using [IterExt::min](http://doc.rust-lang.org/std/iter/trait.IteratorExt.html#tymethod.min) method from the Rust standard library.

Finally, there are differences because the math version uses 1-indexed strings, but Rust has 0-indexed strings.

The good news is that this program is correct (I think). The bad news is that it is slower than continental drift. Computing `lev("tyrannosaurus rex", "oedipus rex")` takes 5 and a half minutes on my machine, even when using `-C opt-level=3` during compilation.

The reason it is slow isn't too hard to see: the time complexity is exponential since for each call that doesn't have one of the strings empty, we will make 3 other calls. A lower bound on the time complexity is $3^{\min(|s|, |t|)}$ since we must make at least $\min(|s|, |t|)$ calls before hitting the base case of an empty string.

How can we make this function actually usable? Notice that even though the run-time of the above implementation is exponential, *there aren't actually an exponential number of distinct calls that can be made*. There are $|s| + 1$ prefixes of $s$ and $|t| + 1$ prefixes of $t$, so there are only $(|s| + 1)(|t| + 1)$ different calls. The slowdown must be due to the fact that we are repeating some of the calls over and over. So one idea is to just save the results in a lookup table so we aren't being wasteful by re-computing them many times.

Here's a data structure I wrote for this lookup table:

```rust
struct MemoMatrix<T> {
    rows: usize,
    cols: usize,
    v: Vec<Option<T>>,
}

impl<T> MemoMatrix<T> {
    fn new(rows: usize, cols: usize) -> Self {
        let mut v = Vec::with_capacity(rows*cols);
        for _ in 0..(rows*cols) {
            v.push(None);
        }
        MemoMatrix { rows: rows, cols: cols, v: v }
    }
}

impl<T> Index<(usize, usize)> for MemoMatrix<T> {
    type Output = Option<T>;
    fn index<'a>(&'a self, index: &(usize, usize)) -> &'a Option<T> {
        let (r, c) = *index;
        assert!(r < self.rows, "row index '{}' is out of bounds", r);
        assert!(c < self.cols, "column index '{}' is out of bounds", c);
        &self.v[r * self.cols + c]
    }
}

impl<T> IndexMut<(usize, usize)> for MemoMatrix<T> {
    fn index_mut<'a>(&'a mut self, index: &(usize, usize)) -> &'a mut Option<T> {
        let (r, c) = *index;
        assert!(r < self.rows, "row index '{}' is out of bounds", r);
        assert!(c < self.cols, "column index '{}' is out of bounds", c);
        &mut self.v[r * self.cols + c]
    }
}
```

`MemoMatrix` is a "2-dimensional" lookup table (with values stored in a `Vec`, which is the Rust standard library's dynamic array). The default entries are `None`, and whenever we compute the cost for cell `(i, j)` we update the corresponding entry of `MemoMatrix` to be `Some(cost)`. The implementations of `Index` and `IndexMut` are so that we can use the indexing syntax `[]` on any MemoMatrix. This makes it easier to use in the new `ed` implementation:


```rust
fn lev<'a, 'b>(s: &'a str, t: &'b str) -> usize {
    let s_chars: Vec<char> = s.chars().collect();
    let t_chars: Vec<char> = t.chars().collect();
    // rect(i, j) is the minimal cost of an edit sequence that turns s[..i] into t[..j]
    let mut rect = MemoMatrix::new(s.chars().count() + 1, t.chars().count() + 1);
    ed(&mut rect, &s_chars[..], &t_chars[..])
}

fn ed<'a, 'b>(rect: &mut MemoMatrix<usize>, s: &'a [char], t: &'b [char]) -> usize {
    let (i, j) = (s.len(), t.len());

    // check if this has already been computed and use it if so
    match rect[(i, j)] {
        Some(dist) => return dist,
        None => {},
    }

    let dist = if i == 0 {
        j
    } else if j == 0 {
        i
    } else {
        let (a, b) = (i-1, j-1);
        let v = vec![
            ed(rect, &s[..a], &t[..b]) + if s[a] == t[b] { 0 } else { 1 },
            ed(rect, &s[..a], t) + 1,
            ed(rect, s, &t[..b]) + 1
        ];
        v.into_iter().min().unwrap()
    };

    rect[(i, j)] = Some(dist);
    dist
}
```

The only real difference is that `ed` now takes a `MemoMatrix` and, on every call, checks it to see if the value being requested has already been computed.

By using the table to store values, the algorithm now has a time complexity of $O(|s||t|)$ on average (we must compute all the values in the table).

The above `ed` implementation can actually be slightly improved upon. In computing $L_{s, t}(i, j)$, if $s_i = t_j$, then there is no need to evaluate the edit sequences involving $L_{s, t}(i-1, j)$ or $L_{s, t}(i, j-1)$, because it cannot be the case that

$$L_{s,t}(i-1, j-1) > L_{s, t}(i-1, j) + 1$$

or

$$L_{s,t}(i-1, j-1) > L_{s, t}(i, j-1) + 1$$

This is due to the triangle inequality holding for the Levenshtein distance. ($L_{s, t}(i-1, j-1)$ is the distance between $s_{1..i-1}$ and $t_{1..j-1}$, $L_{s,t}(i-1, j)$ is the distance between $s_{1..i-1}$ and $t_{1..j}$, and $1$ is the distance between $t_{1..j-1}$ and $t_{1..j}$ (and similarly for the other case).)

The `if` statement now becomes:

```rust
    let dist = if i == 0 {
        j
    } else if j == 0 {
        i
    } else {
        let (a, b) = (i-1, j-1);
        if s[a] == t[b] {
            ed(rect, &s[..a], &t[..b])
        } else {
            let v = vec![
                ed(rect, &s[..a], &t[..b]),
                ed(rect, &s[..a], t),
                ed(rect, s, &t[..b])
            ];
            v.into_iter().min().unwrap() + 1
        }
    };
```

Computing `lev("tyrannosaurus rex", "oedipus rex")` happens instantaneously on my machine now. Huzzah!


## Let's get fuzzy

What we've done so far is implemented a function that quantifies the fuzziness between two strings, which is a great start, but that's not what we were aiming for. We were aiming for a way to determine if some pattern string `p` can be found in some other text string `t` up to a certain degree of fuzziness. To be more precise, we would like to know if there are any substrings of `t` that are within an edit distance of `k` from our pattern string `p`. Can we somehow use or modify our function for computing the Levenshtein metric to accomplish this task?

The simplest idea is probably this: we add two new operations, both with zero cost. The new operations are inserting a prefix of `t` into the beginning of `s`, and inserting a suffix of `t` at the end of `s`. For example, suppose we want to do a fuzzy search `p = "eieio"` in the string `t = "Karl Weierstrass"`. It should be clear that the substring "eiers" of `t` is edit distance 2 away from `p`. So under this new "distance" function, one edit sequence that turns `p` into `t` is:

  1. prefix insert of "Karl W" ["eieio" => "Karl Weieio"]
  2. replacement of "i" with "r" ["Karl Weieio" => "Karl Weiero"]
  3. replacement of "o" with "s" ["Karl Weiero" => "Karl Weiers"]
  4. suffix insert of "trass" ["Karl Weiers" => "Karl Weierstrass"]

As mentioned before, the first and last operations have zero cost, so the total cost of this sequence is 2. With this new "distance" function in hand, we can just see if `new_dist(p, t)` does not exceed `k`. There's nothing more to it than that.

It should be clear that this new "distance" function is not actually a metric since we have introduced an asymmetry in the way distance is calculated. It's actually a bit worse than that: it fails positive definiteness as well since whenever `p` is a (non-fuzzy) substring of `t`, we have an edit cost of zero.

(Bit of a tangent: a metric that potentially fails the "definiteness" part of positive-definiteness is called a [pseudo-metric](http://en.wikipedia.org/wiki/Pseudometric_space), and a metric that potentially fails the symmetric property is called a [quasi-metric](http://en.wikipedia.org/wiki/Metric_%28mathematics%29#Quasimetrics). So it seems we have a ["quasi-pseudo-metric"](http://link.springer.com/article/10.1007%2FBF01301400), or maybe a ["pseudoquasimetric"](http://en.wikipedia.org/wiki/Metric_%28mathematics%29#Pseudoquasimetrics) on our hands here.)

In lieu of directly implementing the new distance function described above, there's a way to implement it with minimal change to the `ed` function. We will define an `ed_sub` function whose only difference is that `ed(p, t) = 0` whenever `p` is empty. This effectively lets us skip to anywhere in `t` at zero cost, so it implements the prefix insert operation above. To implement the suffix insert part, we just call `ed_sub` on all prefixes of `s` (calculating on a prefix of `t` means that we're leaving out a suffix of `t`, which is effectively the same thing as a zero-cost suffix insert operation).

Here's what this looks like in pseudocode:

    fuzzy_contains(p, t, k):
        return fuzzy_sub_dist(p, t) <= k
            
    fuzzy_sub_dist(p, t):
        let min = some really big integer
        for each prefix s of t:
            let res = ed_sub(p, s)
            if res <= min:
                min = res
        return min

And here's an implementation of it in Rust:

```rust
fn fuzzy_contains<'a, 'b>(p: &'a str, t: &'b str, k: usize) -> bool {
    fuzzy_sub_dist(p, t) <= k
}

fn fuzzy_sub_dist<'a, 'b>(p: &'a str, t: &'b str) -> usize {
    let p_chars: Vec<char> = p.chars().collect();
    let t_chars: Vec<char> = t.chars().collect();

    // rect(i, j) is the minimal cost of an edit sequence that turns p[..i] into t[..j]
    // but now with free "initial insertions" into p
    let n = t.chars().count();
    let mut rect = MemoMatrix::new(p.chars().count() + 1, n + 1);

    let mut min = usize::MAX;
    for k in 0..(n+1) {
        let dist = ed_sub(&mut rect, &p_chars[..], &t_chars[..k]);
        if dist < min {
            min = dist;
        }
    }

    min
}

fn ed_sub<'a, 'b>(rect: &mut MemoMatrix<usize>, p: &'a [char], t: &'b [char]) -> usize {
    let (i, j) = (p.len(), t.len());

    // check if this has already been computed and use it if so
    match rect[(i, j)] {
        Some(dist) => return dist,
        None => {},
    }


    let dist = if i == 0 {
        0
    } else if j == 0 {
        i
    } else {
        let (a, b) = (i-1, j-1);
        if p[a] == t[b] {
            ed_sub(rect, &p[..a], &t[..b])
        } else {
            let v = vec![
                ed_sub(rect, &p[..a], &t[..b]),
                ed_sub(rect, &p[..a], t),
                ed_sub(rect, p, &t[..b])
            ];
            v.into_iter().min().unwrap() + 1
        }
    };

    rect[(i, j)] = Some(dist);
    dist
}
```

By running a few examples we can verify that this seems to work:

```rust
fn main() {
    println!("{}", fuzzy_contains("nana", "bananas", 0));
    println!("{}", fuzzy_contains("I", "team", 0));
    println!("{}", fuzzy_contains("annually", "simulated annealing", 2));
    println!("{}", fuzzy_contains("annually", "simulated annealing", 3));
}
```

The results:

    true
    false
    false
    true

Note that when we call `fuzzy_contains` with `k = 0`, we just have regular non-fuzzy string matching (though I doubt it is quite as good as dedicated non-fuzzy algorithms).

## Conclusion

There are various improvements you can make to what I've presented above. One major one, noted in the book by Navarro and Raffinot, is that it's possible to modify the algorithm to work with $O(|p|)$ space instead of $O(|p||t|)$ space. There are also non-dynamic-programming-based algorithms for doing fuzzy string matching. I don't really feel like getting into any of this so I'm just going to stop writing.
