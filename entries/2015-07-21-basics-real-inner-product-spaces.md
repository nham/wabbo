---
title: Basics of real inner product spaces
tags: math
---

A **real orthogonal space** is a real vector space $V$ equipped with a symmetric, bilinear form $\phi: V^2 \to V$ called the **scalar product**. 

Two vectors $v, w \in V$ are said to be **orthogonal** when $\phi(v, w) = 0$, and two subsets $A, B \subseteq V$ are said to be **mutually orthogonal** if every pair $(a, b) \in A \times B$ is orthogonal.

We are usually interested in real orthogonal spaces where the scalar product is *positive-definite*, that is when for all $v \in V$, $\phi(v, v) \geq 0$, and $\phi(w, w) = 0$ iff $w = 0$. Such a space is called a **real inner product space**, (alternatively, **Euclidean vector space**). We will denote the scalar product of a real inner product space by $\phi(v, w) := v \cdot w$.

**Non-degenerateness:** The scalar product of any Euclidean vector space $V$ must be non-degenerate, in the sense that for any $v \in V$, $v \cdot w = 0$ for every $w \in V$ then $v = 0$.

*Proof:* In particular, we have $v \cdot v = 0$, and $v \neq 0$ contradicts positive-definiteness.

$\Box$

Positive-definiteness of the inner product means you can define a **norm** on any real inner product space, which is any map $\psi: V \to \mathbb{R}$ satisfying

The positive-definiteness of a real inner product space $V$ enables one to define a *norm*: for all $v \in V$, a map $V \to \mathbb{R}$ defined by $v \mapsto |v| := \sqrt{v \cdot v}$. It satisfies these properties:

 - *positive-definite*: $|v| \geq 0$ for all $v \in V$ and $|v| = 0$ iff $v = 0$ (follows from definition)
 - *homogeneous*: $| \alpha v| = |\alpha| |v|$ for all $\alpha \in \mathbb{R}$ and $v \in V$. Proof: $| \alpha v| = \sqrt{\alpha^2 v \cdot v} = |\alpha| |v|$

There's another, crucial property, but the proof uses a few essential facts about real inner product spaces:

**Pythagorean property:** $|a + b|^2 = |a|^2 + |b|^2$ whenever $a, b$ are orthogonal members of a real inner product space.

*Proof*: $|a + b|^2 = |(a + b) \cdot (a + b)| = ||a|^2 + |b|^2 + 2(a \cdot b)| = |a|^2 + |b|^2$ 

$\Box$

**Cauchy-Schwarz inequality:** For any real inner product space $V$, $|a \cdot b| \leq |a| |b|$ for all $a, b \in V$. Further more, equality holds precisely when $a$ and $b$ are linearly dependent.

Proof: If $a$ and $b$ are linearly dependent, then without loss of generality assume $b = \gamma a$ for some $\gamma \in \mathbb{R}$. We have $|a \cdot b| = |a \cdot \gamma a| = |\gamma| |a \cdot a| = |\gamma| |a|^2 = |a| |b|$.

Now assume $a$ and $b$ are linearly independent. In particular this implies $b \neq 0$, so we can form

$z := a - \frac{a \cdot b}{b \cdot b} b$

This is actually the part of $a$ perpendicular to $b$, as is proved by:

$z \cdot b = a \cdot b - \frac{a \cdot b}{b \cdot b}(b \cdot b) = 0$

So we have $a = z + \frac{a \cdot b}{b \cdot b} b$, meaning that by the Pythagorean property:

$|a|^2 = |z|^2 + |\frac{a \cdot b}{b \cdot b}|^2 |b|^2 \geq |\frac{a \cdot b}{b \cdot b}|^2 |b|^2$

This implies $|a \cdot b|^2 \leq |a|^2 |b|^2$, which implies $|a \cdot b| \leq |a| |b|$. Also, if we have equality, then $|z| = 0$, so $a = \frac{a \cdot b}{b \cdot b} b$, meaning $a$ and $b$ are actually linearly dependent, contrary to assumption. $\Box$

With the Cauchy-Schwarz inequality in hand, one can prove that inner product spaces obey the *triangle inequality*:

**Triangle inequality:** For all $v, w \in V$, $|v + w| \leq |v| + |w|$.

*Proof:*

$$\begin{align}
  |x + y|^2 &= (x + y) \cdot (x + y) \\
            &= x \cdot x + y \cdot y + 2(x \cdot y) \\
            &\leq x \cdot x + y \cdot y + 2|x||y| \\
            &= (|x| + |y|)^2 \\
  \end{align}$$

$\Box$

A vector space with a norm that satisfies the three properties of positive-definiteness, homogeneity and the triangle inequality is said to be a **normed vector space**. We have just proven that given any real inner product space $V$, the function $V \to \mathbb{R}$ defined by $v \mapsto \sqrt{v \cdot v}$ is a normed vector space.

The useful thing about a normed vector space is that it induces a metric on the vector space by 

$$d(v, w) := |v - w|$$

You can easily verify that it satisfies all metric axioms, turning the space into a metric space.
