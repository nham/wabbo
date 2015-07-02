---
title: The basics of Markov chains
---

A **discrete-time stochastic process** is a sequence $(X_0, X_1, \ldots)$ of [random variables][wiki-random-variable] (see also [here][wabbo-random-variable]). The point of it seems to be for modeling something with state that evolves stochastically over time and is observed at discrete instants of time. The distribution of the random variable $X_t$ gives the various probabilities of being in various states at time $t$.

A **Markov chain** is a discrete-time stochastic process such that for any $t \in \mathbb{N}$:

$$\mathbb{P}(X_{t+1} = j | X_0 = i_0, \ldots, X_{t-1} = i_{t-1}, X_t = i) = \mathbb{P}(X_{t+1} = j | X_t = i)$$

This equation is sometimes called the **Markov property**. One way of putting it in words is that "the future is conditionally independent of the past given the present". A Markov chain describes a "memoryless" discrete-time system in that it forgets all of its past state and only depends on the present state.

We will assume some simplifications: that there is a single state space $S$ that all the random variables $X_t$ share, and that $S$ is countable (finite or countably infinite). Since any countable set can always be bijected with a subset of $\mathbb{N}$, we can furthermore assume that $S = \mathbb{N}$ without loss of generality.

The distribution of the initial random variable $X_0$ is called the **initial distribution** of the Markov chain.

It is useful to single out the class of **homogeneous** Markov chains, i.e. Markov chains such that for all $t$:

$$\mathbb{P}(X_{t+1} = j | X_t = i) = \mathbb{P}(X_1 = j | X_0 = i)$$

To make this condition more vivid, consider defining the matrix $P_t$ by

$$(P_t)_{ij} := \mathbb{P}(X_{t+1} = j | X_t = i)$$

$P_t$ can be considered the **transition matrix at time $t$** of the Markov chain. Then the $(i, j)$-th cell in $P_t$ gives the probability of the Markov chain that is in state $i$ at time $t$ to move to state $j$ at time $t+1$.

Note that all the entries in a transition matrix are nonnegative. You can also easily confirm that each row in a transition matrix sums to $1$.

Homogeneous Markov chains are exactly the Markov chains that have a single transition matrix used for all time. As a result, homogeneous chains are fairly easy to analyze:

**Theorem:** For any homogeneous Markov chain $(X_t)_{t \geq 0}$, the joint distribution of all state variables is completely determined by the initial distribution $P_0$ on the set of states $S$ and the transition matrix $P$.

*Proof:* Let $n \in \mathbb{N}$ be arbitrary, then

$$\begin{align}
\mathbb{P}(X_n = i_n, \ldots, X_0 = i_0)
 &= \mathbb{P}(X_0 = i_0) \prod_{k=1}^n \mathbb{P}(X_k = i_k | X_{k-1} = i_{k-1}) \\
 &= P_0(i_0) * \prod_{k=1}^n P_{i_{k-1} i_k}
\end{align}$$

where the first holds by the basic rules of conditional probability and the second holds by definition.

$\Box$

## Chapman-Kolmogorov equation

Suppose $(X_t)$ is a homogeneous Markov chain. Define the *$n$-step transition probability* $P_{ij}^n$ to be:

$$P_{ij}^n := \mathbb{P}(X_n = j | X_0 = i)$$

Note that this definition depends on the Markov chain being homogeneous. In order for it to be well-defined in the non=homogeneous case, we would also need to specify the time $t$ we are starting from. In the homogeneous case, the starting time does not matter.

The Chapman-Kolmogorov equation says that for any $r \leq n$:

$$P_{ij}^n := \sum_{k=0}^{\infty} P_{ik}^r P_{kj}^{n-r}$$

*Proof:* It's not too difficult to see that

$$P_{ik}^r P_{kj}^{n-r} = \mathbb{P}(X_n = j, X_r = k | X_0 = i)$$

Consequently, we have:

$$P_{ij}^n := \sum_{k=0}^{\infty} \mathbb{P}(X_n = j, X_r = k | X_0 = i)$$

$\Box$


[wiki-random-variable]: https://en.wikipedia.org/wiki/Random_variable
[wabbo-random-variable]: 2015-05-19-probability-3.html#random-variable
