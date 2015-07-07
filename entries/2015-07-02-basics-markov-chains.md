---
title: The basics of Markov chains
tags: math, probability, stochastic processes
---

A **discrete-time stochastic process** is a sequence $(X_0, X_1, \ldots)$ of [random variables][wiki-random-variable] (see also [here][wabbo-random-variable]). The point of it seems to be for modeling something with state that evolves stochastically over time and is observed at discrete instants of time. The distribution of the random variable $X_t$ gives the various probabilities of being in various states at time $t$.

A **Markov chain** is a discrete-time stochastic process such that for any $t \in \mathbb{N}$:

$$\mathbb{P}(X_{t+1} = j | X_0 = i_0, \ldots, X_{t-1} = i_{t-1}, X_t = i) = \mathbb{P}(X_{t+1} = j | X_t = i)$$

This equation is sometimes called the **Markov property**. One way of putting it in words is that "the future is conditionally independent of the past given the present". A Markov chain describes a "memoryless" discrete-time system in that it forgets all of its past state and only depends on the present state.

We will assume some simplifications: that there is a single state space $S$ that all the random variables $X_t$ share, and that $S$ is countable (finite or countably infinite). Since any countable set can always be bijected with a subset of $\mathbb{N}$, we can furthermore assume that $S = \mathbb{N}$ without loss of generality.

The distribution of the initial random variable $X_0$ is called the **initial distribution** of the Markov chain.

It is useful to single out the class of **homogeneous** Markov chains, i.e. Markov chains such that for all $t$:

$$\mathbb{P}(X_{t+1} = j | X_t = i) = \mathbb{P}(X_1 = j | X_0 = i)$$

To make this condition more vivid, consider defining the matrix $P_t$ by setting the $(i, j)$-th cell to be:

$$(P_t)_{ij} := \mathbb{P}(X_{t+1} = j | X_t = i)$$

$P_t$ can be considered the **transition matrix at time $t$** of the Markov chain. Note that $(P_t)_{ij}$ gives the probability of the Markov chain that is in state $i$ at time $t$ to move to state $j$ at time $t+1$.

Note that all the entries in a transition matrix are nonnegative. You can also easily confirm that each row in a transition matrix sums to $1$. This means each row in the matrix can be thought of as probability distribution (probability mass function) on all possible next states.

With this notion of a transition matrix in hand, we can understand homogeneous Markov chains as precisely the Markov chains that have a single transition matrix used for all time. As a result, homogeneous chains are fairly easy to analyze:

**Theorem:** For any homogeneous Markov chain $(X_t)_{t \geq 0}$, the joint distribution of all state variables is completely determined by the initial distribution $P_0$ on the set of states $S$ and the transition matrix $P$.

*Proof:* Let $n \in \mathbb{N}$ be arbitrary, then

$$\begin{align}
\mathbb{P}(X_n = i_n, \ldots, X_0 = i_0)
 &= \mathbb{P}(X_0 = i_0) \prod_{k=1}^n \mathbb{P}(X_k = i_k | X_{k-1} = i_{k-1}) \\
 &= P_0(i_0) * \prod_{k=1}^n P_{i_{k-1} i_k}
\end{align}$$

where the first holds by the basic rules of conditional probability and the second holds by definition.

$\Box$

## The Chapman-Kolmogorov equation

Suppose $(X_t)$ is a homogeneous Markov chain. Define the *$n$-step transition probability* $P_{ij}^n$ to be:

$$P_{ij}^n := \mathbb{P}(X_n = j | X_0 = i)$$

Note that this definition depends on the Markov chain being homogeneous. In order for it to be well-defined in the non=homogeneous case, we would also need to specify the time $t$ we are starting from. In the homogeneous case, the starting time does not matter.

The Chapman-Kolmogorov equation says that for any $r \leq n$:

$$P_{ij}^n := \sum_{k=0}^{\infty} P_{ik}^r P_{kj}^{n-r}$$

*Proof:* It's not too difficult to see that

$$P_{ik}^r P_{kj}^{n-r} = \mathbb{P}(X_n = j, X_r = k | X_0 = i)$$

Consequently, we have:

$$\sum_{k=0}^{\infty} \mathbb{P}(X_n = j, X_r = k | X_0 = i) = \mathbb{P}(X_n = j | X_0 = i) =: P_{ij}^n$$

since the sum is over all possible values of $X_r$.

$\Box$


## Reducibility

For any homogeneous Markov chain $(X_t)_{t \geq 0}$, say that **state $j$ is accessible from state $i$** whenever $P_{ij}^n$ for some $n \geq 0$. We will say that states $i$ and $j$ **communicate** whenever $j$ is accessible from $i$ and $i$ is accessible from $j$. This state of affairs is denoted by $i \leftrightarrow j$.

You can prove that $\leftrightarrow$ is an equivalence relation as follows:

 - $i \leftrightarrow i$ for any $i$ since $P_{ii}^0 = 1$.
 - $i \leftrightarrow j$ implies $j \leftrightarrow i$ pretty much directly from the definition.
 - finally, if $i \leftrightarrow j$ and $j \leftrightarrow k$, this means there are $n, m \geq 0$ such that $P_{ij}^n, P_{jk}^m > 0$. As a result, we have:

    $$P_{ik}^{n+m} = \sum_{r=0}^{\infty} P_{ir}^n P_{rk}^m \geq P_{ij}^n P_{jk}^m > 0$$

    which establishes the transitivity of $\leftrightarrow$.

Because of the $\leftrightarrow$ equivalence relation, the state space of a Markov chain can be thought of as partitioned into *communication classes*, i.e. the equivalence classes of $\leftrightarrow$. Once the Markov chain transitions to a state in one class, it can never reach a state in a different class.

A Markov chain is said to be **irreducible** if there is a single communication class. This means given any starting state $i$ and any other state $j$, there is always a non-zero probability of transitioning from $i$ to $j$ in some finite number of steps. Otherwise the chain is said to be **reducible**.

## Periodicity

For any homogeneous Markov chain we can define, for each $i$, the set

$$\mathcal{T}(i) := \{ n \geq 1 :  P_{ii}^n > 0 \}$$

$\mathcal{T}(i)$ is the set of $n$ such that there is a non-zero chance of starting from state $i$ and reaching it again after exactly $n$ time steps. If there is a greatest common divisor $d \geq 1$ of all $n \in \mathcal{T}(i)$, then state $i$ is said to have **period $d$**. When $d = 1$ the state is said to be **aperiodic**.

It turns out that periods can be thought of not just as a property of a state, but of a communication class:

**Theorem:** $i \leftrightarrow j$ implies that $i$ and $j$ have the same period.

*Proof:* By hypothesis $i \leftrightarrow j$, which means there exist $m, n$ such that $P_{ij}^m, P_{ji}^n > 0$. Hence

$$P_{ii}^{m+n} \geq P_{ij}^m P_{ji}^n \geq 0$$

This means that $d_i := \gcd \mathcal{T}(i)$ divides $m+n$.

Furthermore, for any $k \in \mathcal{T}(j)$,

$$P_{ii}^{m+k+n} \geq P_{ij}^m P_{jj}^k P_{ji}^n \geq 0$$

This implies that $d_i$ divides $m + k + n$, and hence also divides $(m + k + n) - (m + n) = k$.

We have just established that $d_i$ is a common divisor of $\mathcal{T}(j)$, which implies that

$$d_i \leq d_j$$

A similar argument establishes that $d_j$ is a common divisor of $\mathcal{T}(i)$, which implies

$$d_j \leq d_i$$

And hence that states $i$ and $j$ have the same period.

$\Box$

A Markov chain is said to be **aperiodic** if all of its states are aperiodic. Note that an irreducible Markov chain is aperiodic whenever one of its states is aperiodic (irreducibility implies all states have the same period).

[wiki-random-variable]: https://en.wikipedia.org/wiki/Random_variable
[wabbo-random-variable]: 2015-05-19-probability-3.html#random-variable
