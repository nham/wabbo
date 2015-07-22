---
title: Basic real analysis, extended reals, limit superior and limit inferior
tags: math, real analysis
---

In this post I discuss a number of things, but I'm really just trying to develop the characterization of convergence for sequences of extended reals in terms of limit superior / limit inferior. This is a basic and powerful tool for proving the convergence of sequences of real numbers.

## Basic real analysis

The **monotone convergence theorem** says that every monotonically non-decreasing (non-increasing) sequence that is bounded above (below) converges to the least upper bound (greatest lower bound) of the set of terms. If we start out axiomatizing $\mathbb{R}$ with the least upper bound property, this can be proved as follows: the least upper bound $C$ of the term set exists, so we can find terms arbitrarily close to $C$. But the sequence is monotone, so once we find any term within an $\epsilon$, all other terms are as well. So it converges. The same proof works for non-increasing sequences bounded below.

**Peak point lemma:** Any sequence in $\mathbb{R}$ has a monotone subsequence. 

*Proof:* For any sequence $(x_n)$ let us say that $x_m$ is a "peak term" if it acts as an upper bound for the terms that follow ($x_m \geq x_n$ for all $n > m$). Then any sequence has either infinitely many or finitely many peak terms.

If there are infinitely many, we can form a subsequence out of them that must be monotone non-increasing. If there are finitely many of them. every term after the last peak term is strictly dominated by some term that comes later (since they all must fail to be peak terms), so we can find a monotone increasing subsequence. $\Box$

When $(x_n)$ is bounded, the peak point lemma has an interesting consequence: we have a bounded, monotone subsequence, hence convergent subsequence by the monotone convergence theorem. In other words, we have just proved the **Bolzano-Weierstrass** theorem for $\mathbb{R}$: every bounded sequence has a convergent subsequence.

What's the point of Bolzano-Weierstrass? Seems like a weird thing to care about on the surface, but this has to do with sequential compactness. BW says that for every closed, bounded subset $A$ of $\mathbb{R}$, every sequence in $A$ has a convergent subsequence. But $A$ must also contain the limit of the convergent subsequence since it is closed. In other words, $A$, as a topological subspace of $\mathbb{R}$, is sequentially compact (every sequence has a convergent subsequence).

Even if you don't see the fuss about compactness of closed, bounded subsets of $\mathbb{R}$, here's one important corollary: $\mathbb{R}$ is a complete metric space. The proof is as follows: Cauchy sequences are bounded, so they have a convergent subsequence, and Cauchy sequences with convergent subsequences are themselves convergent. So every Cauchy sequence converges. Slick, huh?

## Extended real numbers

The **extended real numbers** are the system of numbers you get by adding $\infty, -\infty$ to $\mathbb{R}$. Let's denote the set by $\bar{\mathbb{R}}$. We can extend the ordering on $\mathbb{R}$ so that

$$-\infty < x < \infty$$

for all finite $x$, making $\bar{\mathbb{R}}$ totally ordered.

The extended reals have the nice property that every non-empty subset $S$ has both a greatest lower bound and a least upper bound. If $S$ is bounded below/above then it is the same greatest lower bound/least upper bound that is guaranteed by the completeness axiom of $\mathbb{R}$. If $S$ is not bounded below/above, then the greatest lower bound/least upper bound is $- \infty / \infty$.

Sequence convergence in the extended reals is then just like in the reals, except now $\pm \infty$ may appear as terms in sequences, and they may also be a limit of sequences.

The **extended monotone convergence theorem** says that every monotone nondecreasing/nonincreasing sequence of extended reals converges to the least upper bound / greatest lower bound of the set of terms.

The **order limit theorem** for sequences of extended reals says that if $(a_n)$ and $(b_n)$ are sequences of extended reals and $a_n \leq b_n$ for all $n \geq N$ for some $N$, if the sequences converge then $\lim_{n \to \infty} a_n \leq \lim_{n \to \infty} b_n$.

The proof is not hard: if not, the limit $A$ of $(a_n)$ is strictly greater than $B := \lim_{n \to \infty} b_n$, then there must be some $C \in \mathbb{R}$ such that $B < C < A$. By the definition of sequence limits we can find a tail sequence of $(a_n)$ for which each term is strictly greater than $C$, and similarly a tail sequence of $(b_n)$ for which each term is strictly less than $C$. Take the "intersection" of these sequences and you can find an $N$ such that for each $n \geq N$, $b_n < C < a_n$, which contradicts the assumption that $a_j \leq b_j$ for all $j$ after some point in the sequence.

## Limit superior / limit inferior

For any sequence $(a_n)$ in $\bar{\mathbb{R}}$, we can form the sequences $(x_n)$ and $(y_n)$ with $x_n := \sup (a_k)_{k \geq n}$ and $y_n := \inf (a_k)_{k \geq n}$. These sequences have terms in $\bar{\mathbb{R}}$, and what's more they are monotone: $x_n \geq x_{n+1}$ and $y_n \leq y_{n+1}$. Hence $\lim_{n \to \infty} x_n$ and $\lim_{n \to \infty} y_n$ both exist, called the **limit superior** and **limit inferior**, respectively, and denoted by

$$\limsup_{n \to \infty} a_n := \lim_{n \to \infty} \{ \sup (a_k)_{k \geq n} \}$$

and

$$\liminf_{n \to \infty} a_n := \lim_{n \to \infty} \{ \inf (a_k)_{k \geq n} \}$$

respectively.

Using the extended monotone convergence theorem, we see that

$$\limsup_{n \to \infty} a_n = \inf \{\sup (a_n)_{n \geq k} : k \in \mathbb{N} \}$$

and

$$\liminf_{n \to \infty} a_n = \sup \{\inf (a_n)_{n \geq k} : k \in \mathbb{N} \}$$

In words: if we call the supremum of a tail sequence a "tail supremum" and the infimum of a tail sequence a "tail infimum", then the limit superior is the greatest lower bound of the set of all tail supremums, and the limit inferior is the least upper bound of the set of all tail infimums.

**Proposition:** If $(a_n)$ is a sequence of extended reals, then

$$\liminf_{n \to \infty} a_n \leq \limsup_{n \to \infty} a_n$$

*Proof:* A bit of notation will be convenient. Define

$$I_j := \inf (a_n)_{n \geq j}$$

and

$$S_j := \sup (a_n)_{n \geq j}$$

The fact that we want to prove shall fall out quickly if we prove that for any $j$ and $k$, we have:

$$I_j \leq S_k$$

Note first that $I_i \leq a_i \leq S_i$ for all $i$ holds for all $i$ by the definition of infimum and supremum.

Now, if $j \leq k$, then $I_j \leq I_k$ holds by the definition of infimum. From this we obtain:

$$I_j \leq I_k \leq S_k$$

On the other hand, if $k \leq j$, then similarly $S_j \leq S_k$, which implies that

$$I_j \leq S_j \leq S_k$$

Now, since $\liminf a_n$ is the least upper bound of the collection $\{I_j : j \in \mathbb{N}\}$ of tail infimums, and since every tail supremum $S_k$ is an upper bound of the same set (by what was just proved), then $\liminf a_n$ is a lower bound of $\{S_j : j \in \mathbb{N}\}$. But $\limsup a_n$ is the greatest lower bound of the same set, so we have established

$$\liminf_{n \to \infty} a_n \leq \limsup_{n \to \infty} a_n$$

$\Box$

**Proposition:** If $(a_n)$ and $(b_n)$ are sequences in $\bar{\mathbb{R}}$, and there is some $N$ such that $a_n \leq b_n$ for all $n \geq N$, then

$$\liminf_{n \to \infty} a_n \leq \liminf_{n \to \infty} b_n$$

and

$$\limsup_{n \to \infty} a_n \leq \limsup_{n \to \infty} b_n$$

Proof: For all $k \geq N$, any lower bound of $(a_n)_{n \geq k}$ is a lower bound for $(b_n)_{n \geq k}$, and similarly every upper bound of $(b_n)_{n \geq k}$ is an upper bound for $(b_n)_{n \geq k}$. So:

$$\inf (a_n)_{n \geq k} \leq \inf (b_n)_{n \geq k}$$

and

$$\sup (a_n)_{n \geq k} \leq \sup (b_n)_{n \geq k}$$

Hence the limits of these sequences (the limit inferiors and limit superiors) must obey a similar ordering. $\Box$

**Proposition:** $\lim_{n \to \infty} a_n$ exists iff $\limsup_{n \to \infty} a_n = \liminf_{n \to \infty} a_n$. In this case,

$$\lim_{n \to \infty} a_n = \limsup_{n \to \infty} a_n = \liminf_{n \to \infty} a_n$$

Proof: Let $S := \limsup_{n \to \infty} a_n$, $I := \liminf_{n \to \infty} a_n$.

We first assume $L := \lim_{n \to \infty} a_n$ exists. If $S > L$, some tail sequence is contained in a $\frac{S -L}{2}$-ball around $L$, meaning $\frac{S+L}{2}$ is an upper bound of some tail sequence, contradicting that $S$ is a lower bound of supremums of tail sequences.If $I < L$, then some tail sequence is contained in a $\frac{L-I}{2}$-ball around $L$, meaning $\frac{I+L}{2}$ is a lower bound of some tail sequence, which contradicts that $I$ is an upper bound of the infimums of tail sequences.

So far we have established that $S \leq L \leq I$. But we know that $I \leq S$, since $\inf (a_k)_{k \geq n} \leq \sup (a_k)_{k \geq n}$ for all $n$, so $I = S$.

Conversely, if $I = S$, then for any $\epsilon > 0$, $S + \epsilon > S$, so there is some tail sequence of $(a_n)$ whose supremum is less than $S + \epsilon$. Similarly, $S - \epsilon < S = I$, so there is some tail sequence whose infimum is greater than $S - \epsilon$. Combining these, there is some tail sequence contained entirely within the $\epsilon$-ball around $S$, so $S$ is the limit of $(a_n)$. $\Box$
e
