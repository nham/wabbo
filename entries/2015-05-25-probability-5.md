---
title: Notes on probability, part 5: Random variables
tags: math, probability
---

In [part 3][part3] we had just defined random variables as measurable functions $\Omega \to \mathbb{R}$. (In general we could consider measurable functions $(\Omega, \mathcal{A}) \to (E, \mathcal{E})$ to be $(E, \mathcal{E})$-valued random variables. Usually, however, it is most convenient to take $(E, \mathcal{E})$ to be the Borel space on $\mathbb{R}$.)

The simplest example of a random variable is probably an **indicator random variable** $1_A$ for an event $A$, defined by

$$1_A(\omega) := \text{if } \omega \in A \text{ then } 1 \text{ else } 0$$

To prove this is a random variable, it is necessary to show that $1_A^{pre}((- \infty, a])$ is an event for all $a \in \mathbb{R}$. This is easily shown by case analysis:

$$1_A^{pre}((- \infty, a]) = \begin{cases}
    \emptyset & a < 0 \\
    A^c & 0 \leq a < 1 \\
    \Omega & \text{otherwise}
\end{cases}$$


It may be the case that a random variable $X$ has a [countable][wiki-countable] (finite or countably infinite) image set, in which case the variable is said to be **discrete**. Random variables that are not discrete are called *continuous*. For simplicity we will focus on the discrete case below.

If $X$ is a discrete random variable, let $a_1, a_2, \ldots$ be a numbering of the values that $X$ can take on. Let us further say that $a_k < a_{k+1}$ for all $k$. Then $X^{pre}((- \infty, a_k]) = X^{pre}(\{a_1, \ldots, a_k\}$) is an event for every $k$. Thus

$$X^{pre}(\{a_1, \ldots, a_k\}) - X^{pre}(\{a_1, \ldots, a_{k-1}\}) = X^{pre}(a_k)$$

is also an event. That is, for every value $a_k$ that the random variable $X$ takes on, we can find the probability $\mathbb{P}(X(\omega) = a_k)$. The function defined by

$$a_k \mapsto \mathbb{P}(X(\omega) = a_k)$$

is called the **probability mass function** of $X$. Recall that we supposed, at the start, that $X$ was discrete. Continuous random variables do not have probability mass functions.



[part3]: /entries/2015-05-19-probability-3.html
[wiki-countable]: http://en.wikipedia.org/wiki/Countable_set
