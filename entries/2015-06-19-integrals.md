---
title: The Riemann and Darboux integrals
---

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

What was proven above amounts to saying that $\text{lower}(f)$ is bounded above and $\text{upper}(f)$ is bounded below. Hence we can define the **lower integral** of $f$ by

$$\downarrow \int_a^b f := \sup \text{lower}(f)$$

We can similarly define the **upper integral of $f$** to be

$$\uparrow \int_a^b f := \inf \text{upper}(f)$$
