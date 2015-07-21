---
title: Basic real analysis
---

Some notes on real analysis that I don't have a better place for.

The **monotone convergence theorem** says that every monotonically non-decreasing (non-increasing) sequence that is bounded above (below) converges to the least upper bound (greatest lower bound) of the set of terms. If we start out axiomatizing $\mathbb{R}$ with the least upper bound property, this can be proved as follows: the least upper bound $C$ of the term set exists, so we can find terms arbitrarily close to $C$. But the sequence is monotone, so once we find any term within an $\epsilon$, all other terms are as well. So it converges. The same proof works for non-increasing sequences bounded below.

**Peak point lemma:** Any sequence in $\mathbb{R}$ has a monotone subsequence. 

*Proof:* For any sequence $(x_n)$ let us say that $x_m$ is a "peak term" if it acts as an upper bound for the terms that follow ($x_m \geq x_n$ for all $n > m$). Then any sequence has either infinitely many or finitely many peak terms.

If there are infinitely many, we can form a subsequence out of them that must be monotone non-increasing. If there are finitely many of them. every term after the last peak term is strictly dominated by some term that comes later (since they all must fail to be peak terms), so we can find a monotone increasing subsequence. $\Box$

When $(x_n)$ is bounded, the peak point lemma has an interesting consequence: we have a bounded, monotone subsequence, hence convergent subsequence by the monotone convergence theorem. In other words, we have just proved the **Bolzano-Weierstrass** theorem for $\mathbb{R}$: every bounded sequence has a convergent subsequence.

What's the point of Bolzano-Weierstrass? Seems like a weird thing to care about on the surface, but this has to do with sequential compactness. BW says that for every closed, bounded subset $A$ of $\mathbb{R}$, every sequence in $A$ has a convergent subsequence. But $A$ must also contain the limit of the convergent subsequence since it is closed. In other words, $A$, as a topological subspace of $\mathbb{R}$, is sequentially compact (every sequence has a convergent subsequence).

Even if you don't see the fuss about compactness of closed, bounded subsets of $\mathbb{R}$, here's one important corollary: $\mathb{R}$ is a complete metric space. The proof is as follows: Cauchy sequences are bounded, so they have a convergent subsequence, and Cauchy sequences with convergent subsequences are themselves convergent. So every Cauchy sequence converges. Slick, huh?

The **extended real numbers** are the set formed by adding two elements, $- \infty$ and $\infty$, to $\mathbb{R}$. You can do some slick things with the extended reals by extending the ordering on $\mathbb{R}$ to the new elements. The ordering is quite sensibly defined as

$$- \infty < x < \infty$$

for all $x \in \mathbb{R}$.

Sequence convergence in the extended reals is just like in the reals, except now $\pm \infty$ may appear in sequences, and they may also be a limit of sequences.

The **extended monotone convergence theorem** is that every monotone nondecreasing/nonincreasing sequence of extended reals is bounded above/below, and so converges, either to the supremum/infimum for a finite bound, or to $\pm \infty$.

The **order limit theorem** for sequences of extended reals says that if $(a_n)$ and $(b_n)$ are sequences of extended reals and $a_n \leq b_n$ for all $n \geq N$ for some $N$, if the sequences converge then $\lim_{n \to \infty} a_n \leq \lim_{n \to \infty} b_n$.

The proof is simple: if not, the limit $A$ of $(a_n)$ is strictly greater than $B := \lim_{n \to \infty} b_n$, which means we can find a point $k$ after which all terms of $(a_n)$ are strictly greater than terms of $(b_n)$, which is contrary to assumption.

