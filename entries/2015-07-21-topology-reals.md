---
title: Topology of the reals
tags: math, topology, real analysis
---

An **interval** of $\mathbb{R}$ is a subset $I$ such that for all $a, b \in I$, $a < x < b$ implies that $x \in I$. This includes as degenerate cases singletons and the empty set. An **open interval** is an interval of the form $(a, b), $(a, \infty)$ or $(-\infty, b)$.

**Proposition:** Every interval $I$ is connected.

*Proof:* We can clearly ignore degenerate cases, they are connected. Suppose $A$ and $B$ are a disconnection of $I$. There is some $a \in A$, $b \in B$. WLOG we assume $a < b$. Then interval $[a, b]$ has to be contained in $I$.

Consider $c := \inf \{ x \in B : a < x \}$, which is valid since $a$ is a lower bound of the set. $c$ has to be a boundary point of $A$ (and $B$), because

 - every open ball centered at $c$ must intersect $A$ (if one did not,it would contradict the definition of $c$ since we could find elements of the set smaller than $c$)

 - every open ball centered at $c$ must also intersect $B$ by definition of infimum (if one did not, we could find a greater lower bound)

But $A$ and $B$ are open, and so do not contain boundary points. But they also cover $I$, so one of them must contain $c$. This is a contradiction.

$\Box$

**Proposition:** Every connected subset $S$ of $\mathbb{R}$ must be an interval.

*Proof:* We again neglect the case of $S$ with less than two points. If $S$ is connected and contains $a, b$ with $a < b$ and is missing some $x$ with $a < x < b$, then $(- \infty, x) \cap I$ and $(x, \infty) \cap I$ are together a disconnection of $I$.

$\Box$

**Corollary:** The connected subsets of $\mathbb{R}$ are exactly the intervals in $\mathbb{R}$.

The **intermediate value theorem** says that if $X$ is a connected topological space and $f: X \to \mathbb{R}$ is continuous, then for any $a, b \in X$, WLOG supposing $f(a) \leq f(b)$, we have that for every $z \in \mathbb{R}$ with $f(a) < z < f(b)$, there is a $c \in X$ such that $f(c) = z$. This is because the image of $f$ must be a connected subset of $\mathbb{R}$, which we have just proven must be an interval.



**Countable decomposition of open subsets of $\mathbb{R}$:** Here's a neat/useful fact about $\mathbb{R}$: every open subset $U$ of $\mathbb{R}$ is the countable disjoint union of open intervals.

*Proof:* Partition $U$ by defining an equivalence relation $x \sim y$ iff $[\min{x,y}, \max{x,y}] \subseteq U$. (in words, two points $a$ and $b$ are equivalent precisely when the closed interval between them is contained in $U$). It's easy to prove this really is an equivalence relation.

Furthermore the equivalence classes are intervals because if $a \sim b$ with $a < b$, for any $x \in U$ with $a < x < b$, by definition $x \in [a, b] \subseteq U$, so $x$ is in the same equivalence class. 

What's more, they must be *open intervals*. Suppose $C$ is one of the equivalence classes, and suppose that $C$ is not open. So $C$ must look like one of these:

 - $(- \infty, b]$, $(a, b]$, $[a, b]$
 - $[b, \infty)$, $[b, c)$, $[b, c]$

Essentially, $b$ is an "inclusive end point" of some kind of interval. In the first case, this means that for all $\epsilon > 0$, there is an $x$ such that $b < x < b + \epsilon$ and $[b, x]$ is not contained in $U$. In the second case, for every $\epsilon > 0$ there is an $x$ such that $b - \epsilon < x < b$ and $[x, b]$ is not contained in $U$. In either case, $b$ is a boundary point of $U$ that is contained in $U$, contradicting that $U$ is open. So each equivalence class $C$ must be an open interval.

We've just shown that $U$ has a partition of open intervals. But it is a basic theorem that between any two real numbers is a rational number. So pick a rational number in each open interval. This gives an injective map from intervals to rationals, proving the collection of partition elements is countable.

$\Box$

## $n$-dimensional Euclidean space

An **interval** in $\mathbb{R}^n$, most generally, is a product of intervals in $\mathbb{R}$. An $n$-dimensional **closed box** is a set $\prod_1^n [a_i, b_i]$. Two closed boxes are said to be **non-overlapping** when they have disjoint interiors. The **volume** of a box $I = \prod_1^n [a_i, b_i]$ is defined to $\text{vol}(I) := \prod_1^n (b_i - a_i)$.

