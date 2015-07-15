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

The **mesh size** of a partition $\mathcal{P}$ is defined to be $\max_i(a_i - a_{i-1})$, the length of the longest sub-interval. The mesh sizesize  of $\mathcal{P}$ is denoted by $\| \mathcal{P} \|$.


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

Furthermore, if $\mathcal{P}_1$ and $\mathcal{P}_2$ are partitions and $\mathcal{P}_2$ is a refinement of $\mathcal{P}_1$, then

$$l(f, \mathcal{P}_1) \leq l(f, \mathcal{P}_2)$$

and

$$U(f, \mathcal{P}_2) \leq U(f, \mathcal{P}_1)$$

*Proof sketch:* The idea is to prove it in the case where $\mathcal{P}_2$ has only one point more than $\mathcal{P}_1$ (i.e. the case where $\mathcal{P}_2$ is the result of starting with $\mathcal{P}_1$ and partitioning one of its intervals into two). Once you've proven this you can use induction. $\Box$

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

$\text{lower}(f)$ and $\text{upper}(f)$ are the collection of lower sums and of upper sums, respectively. What was proven above amounts to saying that $\text{lower}(f)$ is bounded above and $\text{upper}(f)$ is bounded below. Therefore we can define (by the least upper bound property) the **lower integral** of $f$ by

$$\downarrow \int_a^b f := \sup \text{lower}(f)$$

We can similarly define the **upper integral of $f$** to be

$$\uparrow \int_a^b f := \inf \text{upper}(f)$$

**Proposition:** The lower integral of $f$ is always less than or equal to the upper integral of $f$. In symbols:

$$\downarrow \int_a^b f \ \leq \ \uparrow \int_a^b f$$

*Proof:* To prove this, we can assume that

$$\uparrow \int_a^b f \ < \ \downarrow \int_a^b f $$

and attempt to derive a contradiction.

First, define

$$\epsilon := \ \downarrow \int_a^b f \ - \ \uparrow \int_a^b f$$

Then $\epsilon > 0$ by assumption, so since the lower integral is the supremum of $\text{lower}(f)$, we can find a partition $\mathcal{P}$ of $[a, b]$ such that the lower sum $l(f, \mathcal{P})$ satisfies

$$\begin{align}
\uparrow \int_a^b f \ &= \ \downarrow \int_a^b f - \epsilon \\
&< \ l(f, \mathcal{P}) \\
&< \ \downarrow \int_a^b f
\end{align}$$

By similar reasoning about $\text{upper}(f)$ we can find a partition $\mathcal{Q}$ such that

$$\uparrow \int_a^b f < U(f, \mathcal{Q}) < l(f, \mathcal{P})$$

Then $\mathcal{P} \cup \mathcal{Q}$ is a partition of $[a, b]$ that is refinement of both $\mathcal{P}$ and $\mathcal{Q}$, so

$$U(f, \mathcal{P} \cup \mathcal{Q}) \leq U(f, \mathcal{Q})$$

and

$$l(f, \mathcal{P}) \leq l(f, \mathcal{P} \cup \mathcal{Q})$$

Combining these with the the above inequality, we have proven

$$U(f, \mathcal{P} \cup \mathcal{Q}) < l(f, \mathcal{P} \cup \mathcal{Q})$$

which is a contradiction.

$\Box$

When $f$ is such that $\downarrow \int_a^b f \ = \ \uparrow \int_a^b f$, then $\int_a^b f$ defined by:

$$(D) \int_a^b f := \ \downarrow \int_a^b f \ = \ \uparrow \int_a^b f$$

is called the **Darboux integral** of $f$ over $[a, b]$. It can also be denoted by

$$(D) \int_a^b f(x) dx$$

as well. In this case $f$ is said to be **Darboux integrable** or just **integrable**.

**Theorem:** A function $f: [a, b] \to \mathbb{R}$ is Darboux integrable iff for every $\epsilon > 0$ there is a partition $\mathcal{P}$ of $[a, b]$ such that

$$U(f, \mathcal{P}) - l(f, \mathcal{P}) < \epsilon$$

*Proof:* From the definition of infimum and supremum, we can find always find lower sums arbitrarily close to $\downarrow \int_a^b f$ and upper sums arbitrarily close to $\uparrow \int_a^b f$. More precisely, we can always find partitions $\mathcal{P}$ and $\mathcal{Q}$ such that

$$\downarrow \int_a^b f \ - l(f, \mathcal{P}) < \epsilon / 2$$

and

$$U(f, \mathcal{Q}) - \ \uparrow \int_a^b f < \epsilon / 2$$

for any any $\epsilon > 0$.

So when $f$ is Darboux integrable, $\downarrow \int_a^b f \ = \ \uparrow \int_a^b f$, and the two inequalities above simplify to

$$U(f, \mathcal{Q}) - l(f, \mathcal{P}) < \epsilon$$

This is not quite what we want, but we can use the refinement $\mathcal{P} \cup \mathcal{Q}$ and easily prove that

$$U(f, \mathcal{P} \cup \mathcal{Q}) - l(f, \mathcal{P} \cup \mathcal{Q}) < \epsilon$$

Conversely, if we can find partitions that make the corresponding upper and lower sums arbitrarily close, then the upper and lower integrals must be the same.

$\Box$

### An example

To illustrate what lower and upper Darboux sums look like, consider the polynomial

$$f(x) = x^3 - 2x^2 + 0.5x + 0.5$$

restricted to the interval $[-1, 2]$. Then for a partition of $[-1, 2]$ into 12 equal intervals, the lower and upper sums look like this:

![Darboux lower sum of $f$](/images/darboux_lower_sum.png)

![Darboux upper sum of $f$](/images/darboux_upper_sum.png)

Note that these images are misleading in that the vast majority of partitions will not have all sub-intervals of equal length. It was simply more convenient to do it this way.

## Riemann integral

First, a **tagged partition** of $[a, b]$ is a partition $\mathcal{P} = (a_0, \ldots, a_k)$ along with a collection of tags $(c_1, \ldots, c_k)$ such that $c_i \in [a_{i-1}, a_i]$ for all $i$.

The **Riemann sum** corresponding to any function $f: [a, b] \to \mathbb{R}$ and any tagged partition $(\mathcal{P}, \vec{c})$ is defined to be

$$\mathcal{R}(f, \mathcal{P}, \vec{c}) := \sum_1^k f(c_i) (a_i - a_{i-1})$$

Then $f$ is said to be **Riemann integrable** when the limit

$$\lim_{\| (\mathcal{P}, \vec{c}) \| \to 0} \mathcal{R}(f, \mathcal{P}, \vec{c})$$

exists. Formally, $f$ is Riemann integrable if there exists an $L \in \mathbb{R}$ such that

for any $\epsilon > 0$, there is a $\delta > 0$ such that whenever

$$\| (\mathcal{P}, \vec{c}) \| < \delta$$

holds, then

$$|\mathcal{R}(f, \mathcal{P}, \vec{c}) - L| < \epsilon$$

is also true.

$L$ in the above is called the **Riemann integral** of $f$ over $[a, b]$. This will be denoted by $(R) \int_a^b f$ or $(R) \int_a^b f(x) dx$.

### An example

To illustrate what Riemann sums look like, we will again consider the polynomial

$$f(x) = x^3 - 2x^2 + 0.5x + 0.5$$

restricted to the interval $[-1, 2]$. Here's a Riemann sum of $f$ with 12 equal intervals:

![Riemann sum of $f$ with 12 equal intervals](/images/riemann_sum_12.png)

You can clearly see the error in this sum: some of the bars exceed the function, while others do not go far enough (some do both!).

As the number of intervals is increased, the sums will tend to become closer and closer to the area under the curve:

![Riemann sum of $f$ with 40 equal intervals](/images/riemann_sum_40.png)

You can imagine that the sum would be quite close if a trillion intervals were used instead.

Note, as above, that most partitions will not have all sub-intervals of equal length. You should try to imagine Riemann sums whose partition intervals are wildly uneven. All that matters is that the length of the *longest* interval goes to zero in the limit.

## Proof of equivalence

This proof is taken from [Daniele Grandini's lecture notes][grandini-notes].

**Lemma:** If $\mathcal{P}$ is a partition of $[a, b]$, then for every $\epsilon > 0$ there is a $\delta$ such that every partition $\mathcal{Q}$ with $\| \mathcal{Q} \| < \delta$ has

$$U(f, \mathcal{Q}) - U(f, \mathcal{P} \cup \mathcal{Q}) < \epsilon$$

and

$$l(f, \mathcal{P} \cup \mathcal{Q}) - l(f, \mathcal{Q}) < \epsilon$$

*Proof:* TODO

$\Box$

**Theorem:** A function $f: [a, b] \to \mathbb{R}$ is Darboux integrable iff it is Riemann integrable, and when $f$ is integrable the two integrals coincide:

$$(D) \int_a^b f = (R) \int_a^b f$$

*Proof:* First suppose that $f$ is Darboux integrable, and let the Darboux integral be $I$. Then for any $\epsilon > 0$ we can find a partition $\mathcal{P}$ such that

$$U(f, \mathcal{P}) - l(f, \mathcal{P}) < \epsilon / 2$$

We can expand this into two inequalities, which may seem pointless but you'll see why it's useful below:

$$\begin{equation}
- \epsilon = - \epsilon / 2 - \epsilon / 2 < l(f, \mathcal{P}) - \epsilon / 2 - U(f, \mathcal{P})
\label{eq:one}
\end{equation}$$

and

$$\begin{equation}
U(f, \mathcal{P}) + \epsilon / 2 - l(f, \mathcal{P}) < \epsilon / 2 + \epsilon / 2 = \epsilon
\label{eq:two}
\end{equation}$$


Now, by the lemma previously proved, there is a $\delta$ such that every partition $\mathcal{Q}$ with $\| \mathcal{Q} \| < \delta$ has

$$U(f, \mathcal{Q}) - U(f, \mathcal{P} \cup \mathcal{Q}) < \epsilon / 2$$

and

$$l(f, \mathcal{P} \cup \mathcal{Q}) - l(f, \mathcal{Q}) < \epsilon / 2$$


We can of course find a partition $\mathcal{Q}$ such that $\| \mathcal{Q} \| < \delta$. This means that

$$U(f, \mathcal{Q}) < U(f, \mathcal{P} \cup \mathcal{Q}) + \epsilon / 2 \leq U(f, \mathcal{P}) + \epsilon / 2$$

and

$$l(f, \mathcal{Q}) > l(f, \mathcal{P} \cup \mathcal{Q}) - \epsilon / 2 \geq l(f, \mathcal{P}) - \epsilon / 2$$

Now, for any fixed partition, the Riemann sum associated with any set of tags for that partition is bounded above and below by upper and lower Darboux sums for the partition (pretty much by the definitions). So for any tags $\vec{c}$ for the partition $\mathcal{Q}$, we have

$$\mathcal{R}(f, \mathcal{Q}, \vec{c}) \leq U(f, \mathcal{Q})$$

$$\mathcal{R}(f, \mathcal{Q}, \vec{c}) \geq l(f, \mathcal{Q})$$

Here's where the "pointless" inequalities from above come back into play: we have just proven that:

$$l(f, \mathcal{P}) - \epsilon/2 < \mathcal{R}(f, \mathcal{Q}, \vec{c}) < U(f, \mathcal{P}) + \epsilon / 2$$

for any tagged partition $(\mathcal{Q}, \vec{c})$ with $\| (\mathcal{Q}, \vec{c}) \| < \delta$.

Furthermore, we clearly have $l(f, \mathcal{P}) \leq I \leq U(f, \mathcal{P})$. Combining all of these with the two inequalities, we have that

$$- \epsilon < \mathcal{R}(f, \mathcal{Q}, \vec{c} - I < \epsilon$$

for any tagged partition $(\mathcal{Q}, \vec{c})$ with $\| (\mathcal{Q}, \vec{c}) \| < \delta$.

This establishes that $f$ is Riemann integrable with a Riemann integral of $I$.

Converse: TODO

$\Box$

[grandini-notes]: http://math.unm.edu/~daniele/RiemannDarbouxandStieltjes.pdf
