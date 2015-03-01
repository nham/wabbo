# Fuzzy string matching through dynamic programming

## Motivation

I've been studying math recently, and order to organize my studies I built a small command-line tool called [hippo](https://github.com/nham/hippo). All it does is schedule review of items (definitions, proofs of theorems, ideas) using [spaced repetition](http://en.wikipedia.org/wiki/Spaced_repetition). One of the commands, the `list` command, shows items that match a particular pattern. So if I type `hippo list interior`, it will give me a list of all items containing the string "interior", which currently looks like this:

     21 : def interior of subset of top. space
     23 : proof open subsets are those equal to their interior
     26 : proof interior is largest open subset
     30 : proof closure is disjoint union of interior and boundary
     34 : proof for any set S the whole space is partitioned into interior/boundary/exterior
     38 : proof isolated points may either be boundary or interior points
     39 : proof limit points may either be boundary or interior points
     46 : proof why isn't nowhere dense defined to be sets with empty interior
     69 : proof interior of the complement is the complement of the closure

Unfortunately this doesn't work quite as well as I expected, because I discovered that I'm not always consistent about how I enter terms into the system. When I do `hippo list hausdorff`, I get:

     57 : proof limits of sequences are unique in hausdorff spaces
     58 : closure point proof finite subsets of hausdorff spaces are closed
     71 : union of open neighborhoods proof that finite subsets of hausdorff spaces are closed

which I can tell has a few items missing, notably what the definition of a Hausdorff space even is. This is because for the remaining items I used "Hausdorff", with a capital H, and not "hausdorff". Shucks.

One way to fix this is to go through all of the items that use "hausdorff" and change them to "Hausdorff", and then remember that I need to search with capital-H. This is not a great solution, however, because I'd like to be as lazy as I can possibly get away with. Another solution is to add an option for case-insensitive pattern matching, but after some thought I decided it would be more ergonomic to implement some kind of fuzzy string matching feature. Such a feature would help me to be even more lazy and not have to correct small typos when I make them.

The algorithm I describe below is basically the naive dynamic programming-based algorithm. The strings I'm working with are tiny, so the optimal, state-of-the-art algorithms for approximate string matching are huge overkill. For information on those you might want to look at *Flexible Pattern Matching in Strings: Practical On-Line Search Algorithms for Texts and Biological Sequences*, available online [here](http://nlp.cs.swarthmore.edu/~richardw/cs93-f07/flexiblePatternMatchingInStrings.pdf).

## Levenshtein metric

The approach below will hinge on being able to compute some kind of distance between strings. There are many possible choices for such a distance function, but a popular choice is the **Levenshtein distance**, also called the **edit distance**. We denote this function $ed$, and for any strings $s$ and $t$ define $ed(s, t)$ to be the length of the shortest sequence of operations (called an **edit sequence**) that turns $s$ into $t$. The allowed operations are:

  - inserting a symbol into $s$ ("metric" => "meatric")
  - deleting a symbol from $s$ ("chat" => "cat")
  - replacing one symbol in $s$ with ("hello" => "jello")

The formal definition of this function is a bit complicated, but I'm going to give it anyway and you're just going to have to deal with it. A bit of notation: for any string $s$, $s_i$ denotes the $i$-th symbol in $s$, starting at $1$. So $s = s_1 \cdots s_n$ for some $n$. We also denote the length of $s$ (i.e. $n$ in the last example) by $|s|$. We will notate $s_i \cdots s_{j}$ by $s[i..j]$.

For any strings $s, t$, we define a function $L_{s,t}: |s| \times |t| \to \mathbb{N}$ so that $L_{s, t}(i, j)$ is the length of the smallest edit sequence that turns $s_1 \cdots s_i$ into $t_1 \cdots t_j$. Formally we can do this by:

    $$L_{s,t}(i, j) = \begin{cases}
        j & i = 0 \\
        i & j = 0 \\
        min{L_{s,t}(i-1, j) + 1, L_{s,t}(i, j-1) + 1, L_{s,t}(i-1, j-1) + \delta(s_i, t_j)} & \text{otherwise}
        \end{cases}$$

where $\delta$ is defined so that $\delta(x, y) = 1$ iff $x = y$. We then define $ed(s, t) = L_{s,t}(|s|, |t|)$.

Once you get past all the symbols, this really ain't so bad. It says that:

  - when $i$ is the empty string, the shortest edit sequence is just inserting all the characters of $t$
  - when $j$ is the empty string, the shortest edit sequence is deleting all the characters of $s$
  - otherwise, it's the shortest of these 3:
    
     1. The shortest sequence turning $s[1..i-1]$ into $t[1..j]$ followed by deletion of $s_i$
     2. The shortest sequence turning $s[1..i]$ into $t[1..j-1]$ followed by insertion of $t_j$.
     3. If $s_i = t_j$, we can just use the shortest sequence turning $s[1..i-1]$ into $t[1..j-1]$. Otherwise, it's that sequence followed by replacing $s_i$ with $t_j$.


Hopefully I've convinced you this is a well-defined function. But are we justified in calling this a "distance" function? Does it have all the properties we would expect such a function to have? What even is distance?

Actually, mathematicians have an answer for that last question. What we're really asking here is whether the Levenshtein distance is a [metric](http://en.wikipedia.org/wiki/Metric_%28mathematics%29) on the collection of strings over some alphabet. To verify this we need to prove that $ed$ obeys these properties:

  - positive-definite: $ed(s,t) \geq 0$ for all $s, t$ and $ed(s,t) = 0$ iff $s = t$.
  - symmetric: $ed(s,t) = ed(t,s)$ for all $s$ and $t$
  - transitive: $ed(s, u) \leq ed(s, t) + ed(t, u)$

By definition we have non-negativity, and clearly the only way we can get an edit sequence of 0 is for the strings to be identical. Transitivity is also very easy to see: the smallest edit sequence taking $s$ to $t$ concatenated with the smallest edit sequence taking $t$ to $u$ is in fact an edit sequence of length $ed(s,t) + ed(t, u)$ that takes $s$ to $u$, and we've defined $ed(s, u)$ to be the *smallest* edit sequence that does that.

The symmetricity of $ed$ is not any harder, but seems a bit clunkier to state. The idea I have in mind is that for any operation on $s$, there's a well-defined inverse operation on $t$. So if we insert a symbol $a$ into $s$ at some point, the inverse is deleting $a$ from $t$, and vice versa for deleting a symbol from $s$. The inverse of replacing symbol $a$ in $s$ with symbol $b$ is to replace symbol $b$ in $t$ with symbol $a$. So for any edit sequence turning $s$ into $t$, reverse it and find the inverse of each operation to obtain an edit sequence turning $t$ into $s$.

(I would like to note that since $ed$ is positive-definite, symmetric and obeys the triangle inequality, it is a metric space and hence a Hausdorff topological space, which leads us right back to where we started. Topology is destiny.)

There's a way to generalize the Levenshtein metric as well. Instead of talking about the length of the edit sequence, we can assign each operation type some non-negative cost. Then $ed(s,t)$ is defined to be the minimal cost of all edit sequences taking $s$ to $t$. (I'm not going to explore this, but I thought it was worth mentioning.)

P.S. I'm not really going to use the fact that the edit distance is a bona fide metric in what follows. I just thought it was interesting, and also wanted to trick you into learning some math.

## Implementing the Levenshtein metric

Since this essay is ostensibly about programming, probably we should do some of that. First I'll present a straightforward and unbearably slow implementation of the Levenshtein metric, and then we'll modify it to be actually usable.

Here is my attempt at translating the math above into Rust code:

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

A couple of differences and technical notes: I'm using slices of chars rather than strings because strings in Rust are UTF-8, which makes going to the previous char in a string a bit tricky (there's a way to do it, but the code was less readable). This means you have to convert any strings to a slice of chars in order to use this. I ended up defining a simple conversion function for this:

    fn lev<'a, 'b>(s: &'a str, t: &'b str) -> usize {
        let s_chars: Vec<char> = s.chars().collect();
        let t_chars: Vec<char> = t.chars().collect();
        ed(&s_chars[], &t_chars[])
    }

To calculate the min I'm using [IterExt::min](http://doc.rust-lang.org/std/iter/trait.IteratorExt.html#tymethod.min) method from the Rust standard library.

Finally, there are differences because the math version uses 1-indexed strings, but Rust has 0-indexed strings. Also, my math notation had $s[i..j]$ with $i$ and $j$ being inclusive in the range, but in Rust ranges $s[i..j]$ includes $i$ but *excludes* $j$. Sorry.

