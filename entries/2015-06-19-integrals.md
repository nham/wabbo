---
title: The Riemann and Darboux integrals
tags: math, analysis
---

## Preface

This post is largely based on pp. xv-xvii in Donald L. Cohn's book *Measure Theory*, 2nd ed. The goal is to define and compare the Riemann and Darboux integrals and establish their equivalence.

## Prerequisites

A function $f: X \to \mathbb{R}$ being **bounded** means that there exist numbers $m, M \in \mathbb{R}$ such that

$$m \leq f(a) \leq M$$

for all $a \in X$.

Recall that $[a, b]$ denotes the subset of the real line

$$\{ x \in \mathbb{R} : a \leq x \leq b\}$$

In other words, it is the line segment starting at $a$ and ending at $b$. This is often called a **closed interval** (since it includes both its endpoints).

A **$k$-partition** of a closed interval $[a, b]$ is a tuple of real numbers

$$(a_0, \ldots, a_k)$$

such that

$$a = a_0 < a_1 < \ldots < a_k = b$$

If $(a_i)_0^j$ and $(b_i)_0^k$ are two partitions of $[a, b]$ and each $a_i$ appears as a $b_j$, then $(b_i)_0^k$ is said to be a **refinement** of $(a_i)_0^j$.

For any subset $X$ of $\mathbb{R}$, $c$ is said to be an **upper bound** for $X$ if $c \geq x$ for all $x \in X$, and a **lower bound** for $X$ if $c \leq x$ for all $x \in X$

An upper bound (lower bound) $c$ for $X$ is said to be a **least upper bound** (**greatest lower bound**) for $X$ if every other upper bound (lower bound) $d$ for $X$ was such that $c \leq d \ $ ($d \leq c$). The least upper bound is sometimes called the **supremum**, and the greatest lower bound is sometimes called the **infimum**, and these are written $\sup X$ and $\inf X$, respectively.

Finally, one of the defining properties of the real line is that every non-empty subset of $\mathbb{R}$ that is bounded above has a *least* upper bound, and every non-empty subset that is bounded below has a *greatest* lower bound. This is sometimes known as the [least upper bound property][lub-prop] of the real numbers (the corresponding property for greatest lower bounds is obtainable from the least upper bound property).

[lub-prop]: https://en.wikipedia.org/wiki/Least-upper-bound_property

## Darboux integral

Let $[a, b]$ be any closed interval in $\mathbb{R}$. Then let:

 - $f: [a, b] \to \mathbb{R}$ be any bounded function.
 - $\mathcal{P} = (a_0, \ldots, a_k)$ be some partition of $[a, b]$.

For $i \in 1 \ldots k$, we can define numbers $m_i, M_i$ by

$$m_i := \inf \{ f(x) : x \in [a_{i-1}, a_i]\}$$
$$M_i := \sup \{ f(x) : x \in [a_{i-1}, a_i]\}$$

The reasons these definitions are valid are that $f$ is bounded on $[a, b]$, so it is surely bounded on $[a_{i-1}, a_i]$ for each $i$ (since each is a subset of $[a, b]$), which means the image set of $f$ on $[a_{i-1}, a_i]$ has both an infimum and a supremum.

Note that the numbers $m_i$ and $M_i$ are entirely determined by $f$ and $\mathcal{P}$, so we can define the **lower sum** and **upper sum** of $f$ and $\mathcal{P}$, respectively, by

$$l(f, \mathcal{P}) = \sum_1^k m_i(a_i - a_{i-1})$$
$$U(f, \mathcal{P}) = \sum_1^k M_i(a_i - a_{i-1})$$

Notice that

$$l(f, \mathcal{P}) \leq U(f, \mathcal{P})$$

holds for every $f$ and $\mathcal{P}$ (because $m_i \leq M_i$ for every $i$).

Furthermore, if $\mathcal{P}_1$ and $\mathcal{P}_2$ are partitions with $\mathcal{P}_2$ a refinement of $\mathcal{P}_1$, then

$$l(f, \mathcal{P}_1) \leq l(f, \mathcal{P}_2)$$

and

$$U(f, \mathcal{P}_2) \leq U(f, \mathcal{P}_1)$$

I won't spell out the whole prove, but the idea is to prove it in the case where $\mathcal{P}_2$ has one point more than $\mathcal{P}_1$. From there you use induction.

Here's the fun part: for every $i$ we have

$$m_i \leq M$$

This is true because $M$ is an upper bound of the image set $f([a_{i-1}, a_i])$, and $m_i$ is by definition the *least* upper bound.

Similar reasoning shows that for every $i$,

$$m \leq M_i$$

Using these inequalities, one can now prove

$$l(f, \mathcal{P}) \leq \sum_1^k M (a_i - a_{i-1}) = M(b - a)$$

and

$$m(b - a) = \sum_1^k m(a_i - a_{i-1}) \leq U(f, \mathcal{P})$$

We have just established a significant fact. To see why, consider fixing $f$ and defining

$$\text{lower}(f) := \{ l(f, \mathcal{P}) : \mathcal{P} \text{ is a partition of } [a, b] \}$$
$$\text{upper}(f) := \{ U(f, \mathcal{P}) : \mathcal{P} \text{ is a partition of } [a, b] \}$$

What was proven above amounts to saying that $\text{lower}(f)$ is bounded above and $\text{upper}(f)$ is bounded below. Hence we can define (by the least upper bound property) the **lower integral** of $f$ by

$$\downarrow \int_a^b f := \sup \text{lower}(f)$$

We can similarly define the **upper integral of $f$** to be

$$\uparrow \int_a^b f := \inf \text{upper}(f)$$

Next it seems prudent to prove that the lower integral of $f$ is always less than or equal to the upper integral of $f$. In symbols:

$$\downarrow \int_a^b f \ \leq \ \uparrow \int_a^b f$$

To prove this, we can assume that

$$\uparrow \int_a^b f \ < \ \downarrow \int_a^b f $$

and attempt to derive a contradiction.

Proof: TODO

When $f$ is such that $\downarrow \int_a^b f \ = \ \uparrow \int_a^b f$, then $\int_a^b f$ defined by:

$$\int_a^b f := \ \downarrow \int_a^b f \ = \ \uparrow \int_a^b f$$

is called the **Darboux integral** of $f$ over $[a, b]$. It is also often denoted by

$$\int_a^b f(x) dx$$

as well. In this case $f$ is said to be **Darboux integrable** or just **integrable**.

## Riemann integral

First, a **tagged partition** of $[a, b]$ is a partition $\mathcal{P} = (a_0, \ldots, a_k)$ along with a collection of tags $(x_1, \ldots, x_k)$ such that $x_i \in [a_{i-1}, a_i]$ for all $i$.

The **mesh** of a partition $\mathcal{P}$ is defined to be $\max_i(a_i - a_{i-1})$, the length of the longest sub-interval. The mesh is denoted by $\| \mathcal{P} \|$.

The **Riemann sum** corresponding to any function $f: [a, b] \to \mathbb{R}$ and any tagged partition $\mathcal{P} = ((a_i)_0^k, (x_i)_1^k)$ is defined to be

$$\mathcal{R}(f, \mathcal{P}) := \sum_1^k f(x_i) (a_i - a_{i-1})$$

Then $f$ is said to be **Riemann integrable** when the limit

$$\lim_{\| \mathcal{P} \| \to 0} \mathcal{R}(f, \mathcal{P})$$

exists. Formally, $f$ is Riemann integrable if there exists an $L \in \mathbb{R}$ such that

for any $\epsilon > 0$, there is a $\delta > 0$ such that whenever

$$\| \mathcal{P} \| < \delta$$

holds, then

$$|\mathcal{R}(f, \mathcal{P}) - L| < \epsilon$$

is also true.

$L$ in the above is called the **Riemann integral** of $f$ over $[a, b]$. This will be denoted by $\int_a^b f$ or $\int_a^b f(x) dx$ since, as we will see below, the Darboux integral and Riemann integral are equivalent.

## Proof of equivalence

TODO
