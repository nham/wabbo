---
title: Introduction to topological spaces
---

A **topology** on any set $X$ is a collection $\mathcal{T}$ of subsets of $X$ that is

 - closed under arbitrary unions
 - closed under finite intersections
 - contains both $\emptyset$ and $X$.

The pair $(X, \mathcal{T})$ is said to be a **topological space**. (Usually we abuse terminology and just say $X$ is a topological space.)

The sets in the topology are said to be the **open subsets** of $X$. A subset $U \subseteq X$ is said to be a **neighborhood** of $x \in X$ if both $x \in U$ and $U$ is open.

If you're trying to understand what the above definition of a topology *means*, well, keep wondering, but do take this into account:

 > ... a topology is really a package of several different structures: the notion of openness, the notion of closedness, the notion of neighbourhoods, the notion of convergence, the notion of continuity, the notion of a homeomorphism, the notion of a homotopy, and so forth. They are all important, and it is somewhat artificial to try to designate one of them as being more "fundamental" than the other. But the notion of openness happens to generate all the other notions, and has a particularly elegant and simple axiomatisation, so we have elected to make it the basis for the standard minimalist definition of a topology. But it is important to realise that this is by no means the only way to define a topology, and adopting a more package-oriented point of view can be preferable in some cases (for instance, when generalising the notion of a topology to more abstract structures, such as topoi, in which open sets no longer are the most convenient foundation to begin with).

-- [Terence Tao](http://mathoverflow.net/a/30231)


## Why study them?

The short answer is because you can define continuous functions between topological spaces, and continuous functions are important and worthy of study. Another answer is that lots of things are topological spaces. Probably the most important class of examples are [metric spaces](http://en.wikipedia.org/wiki/Metric_space). Common examples include $\mathbb{R}$, $\mathbb{C}$, $\mathbb{R}^n$ and $\mathbb{C}^n$ for any $n$, also any subsets of these sets. These sets have many interesting topological properties and can serve as a building block for more exotic spaces like [topological manifolds](http://en.wikipedia.org/wiki/Topological_manifold) and [differentiable manifolds](http://en.wikipedia.org/wiki/Differentiable_manifold).

## What is a continuous function anyway

The answer to this question is a bit unsatisfactory. One way to understand what continuous functions between topological spaces are is to study continuous functions between metric spaces. These continuous functions are the ones that are "locally approximately constant". In $\epsilon$-$\delta$ speak, if $(X, d)$, $(Y, e)$ are metric spaces, then any function $f: X \to Y$ is said to be continuous at $a \in X$ when

 - for every $\epsilon > 0$
 - there is a $\delta > 0$
 - such that:
    - for every $x \in X$ with $d(x, a) < \delta$, then $e(f(x), f(a)) < \epsilon$ is true too

In words: $f$ is continuous at $a$ when you can always find an open ball (in $X$) centered at $a$ that $f$ will map into the $\epsilon$-ball (in $Y$) centered at $f(a)$. Essentially, we can make $f$'s output more and more constant (arbitrarily close to constant, in fact) by restricting the input to be closer and closer to $a$.

A function $f: X \to Y$ is just said to be **continuous** when it is continuous at $a$ for all $a \in X$.

Now,

It turns out that you can prove this:

A function $f: X \to Y$ between metric spaces is continuous iff for every $V \subseteq Y$ that is open in $Y$, the preimage $f^{pre}(V)$ is open in $X$. ("Open" here meaning w.r.t. the metric space topologies on $X$ and $Y$, respectively)

## Okay, but what is a continuous function

Right. So a function $f: X \to Y$ between topological spaces is said to be **continuous** when for every $V \subseteq Y$ open in $Y$, $f^{pre}(V)$ is open in $X$.

Here's a few alternative definitions of continuity that you can prove are equivalent without too much trouble:

 - For every $x \in X$ and any neighborhood $V$ of $f(x)$ in $Y$, there is a neighborhood $U$ of $x$ such that $f(U) \subseteq V$.
 - $f^{pre}(C)$ is closed for every $C$ closed in $Y$
 - For any $A \subseteq X$, $f$ maps closure points of $A$ to closure points of $f(A)$.


## Here's the problem

The problem is that to continue learning this topic there's a whole mess of definitions and proofs you have to learn. This is because scholarship is hard and life is unfair.


## Interior, closure, closed sets.

A point $x \in X$ is said to be an **interior point** of a subset $S \subseteq X$ precisely when there is a neighborhood $U$ of $x$ contained in $S$. Also, a point $x$ is said to be a **closure point** of $S$ when every neighborhood of $x$ intersects $S$. (Note that every $s \in S$ is a closure point of $S$, but there may be other closure points not in $S$. Not every $s \in S$ is an interior point, on the other hand, but all the interior points must be in $S$).

The **interior** of a set $S$ is the collection of interior points, and the **closure** is the set of all closure points.

It is straightforward to prove that open subsets are precisely the sets that are equal to their interiors. Analogously to this, we can define **closed subsets** to be those sets that are equal to their closures.

### Another characterization of closure and interior

The interior of any set $S$ is the union of all open sets contained in $S$, while the closure of $S$ is the intersection of all closed sets that contain $S$.

To prove the former, note that the union of all open subsets contains every interior point, and that every point in said union is in one open subset and so must be an interior point.

To prove the latter, note that the closure of $S$ is a closed set containing $S$, and that every closed superset $Z$ of $S$ contains all its closure points. The closure points of any subset of $Z$ are also closure points of $Z$ itself, so $Z$ contains the closure of $S$.

By these definitions, the interior is *the largest open set contained in $S$*, and the closure is *the smallest closed set containing $S$*.


## Boundary

A **boundary point** of a set is a point that is neither in the interior of the set, nor in the interior of the complement of the set. Equivalently, every neighborhood of a boundary point intersects both $S$ and $X - S$. The **boundary** of a set $S$ is the collection of all boundary points of $S$, sometimes denoted $\partial S$.

Notice that by definition, the boundary of $X - S$ is the same as the boundary $S$ (because $X - (X - S) = X$).

## Closure is partitioned into the interior and the boundary

The closure of a set is the disjoint union of the interior and the boundary.

*Proof:* Take any closure point $x$ of $S$. Then either every neighborhood around $x$ intersects both $S$ and $X - S$, or one doesn't and so is contained in $S$.


## A set is closed iff contains boundary, open iff contains no boundary point

We have defined closed subsets to be those that are equal to their closures. But the closure is the disjoint union of the interior and the boundary, and sets always contain their interiors. So a subset is closed iff it contains its boundary.

Similarly, open subsets are those equal to their interiors, and boundary points are by definition not interior points. So open sets contain no boundary points, and a set containing no boundary points contains only interior points.

## A set is closed iff complement is open

A set $S$ is closed in a top. space $X$ iff $X - S$ is open. The reason: $S$ and $X - S$ have the same boundary, so $S$ contains the boundary iff $X - S$ contains none of it.


## The axioms for closed sets

Now we can prove, using DeMorgan's laws, that the collection of all closed subsets of a space $X$ is

  - closed under arbitrary intersection
  - closed under finite union
  - contains $\emptyset$ and $X$

The first two follow immediately from $X - \bigcap_i C_i = \bigcup_i (X - C_i)$, $X - \bigcup_i C_i = \bigcap_i (X - C_i)$ , and the topology axioms. The third is because $\emptyset$ and $X$ are complements of one another.


## Clopen subsets

By the way, did you notice that $\emptyset$ and $X$ are simultaneously closed and open in every topological space? Such sets are sometimes called **clopen**.

## Exterior

If $X$ is a space and $S \subseteq X$, then the interior of $X - S$ is identically the complement of the closure of $S$.

To prove this, note that the points of $X$ that fail to be closure points of $S$ are exactly the ones that have a neighborhood contained in $X - S$, i.e. the interior points of $X - S$.

We define the **exterior of $S$** to be $\text{int}(X - S)$.

## Partioning into interior/boundary/exterior

For any space $X$ and any subset $S$, $X$ is partitioned into the closure and exterior of $S$ (by definition of exterior), and the closure of $S$ is partitioned into the interior and boundary of $S$.

## Boundary is closed

The boundary of any set $S$ is closed because it is the complement of an open set (the union of the interior of $S$ and the interior of $X - S$).
