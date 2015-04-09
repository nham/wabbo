---
title: The complex numbers
date: April 9, 2015
tags: math
---

Start with $\mathbb{R}^2$. It's a [vector space](http://en.wikipedia.org/wiki/Vector_space). To define a multiplication between vectors in $\mathbb{R}^2$, we will define an [algebra](http://en.wikipedia.org/wiki/Algebra_over_a_field) on $\mathbb{R}^2$.

The **complex numbers** $\mathbb{C}$ are defined to be the algebra $(\mathbb{R}^2, \cdot)$ where $1 := e_1$ is unit for the product, and for $i := e_2$, $i \cdot i = -1$. This uniquely determines a bilinear mapping (because $\{1, i\}$ is a basis for $\mathbb{R}^2$).

The product turns out to be associative and commutative as well (check it if you wish), so $\mathbb{C}$ with vector addition and the algebra product is a commutative ring.

If $\mathbb{C}$ turned out to be a [division algebra](http://en.wikipedia.org/wiki/Division_algebra) as well, then $\mathbb{C}$ would be a field, just like $\mathbb{R}$. We can prove this is true. But not immediately. Let's define some things first.

First, the **complex conjugate** of any $z = a + ib \in \mathbb{C}$ is defined to be

$$\bar{z} = a - ib$$

Let's also define the **norm** of $z$ to be

$$|z| := \sqrt{a^2 + b^2}$$

That is, the standard Euclidean norm on $\mathbb{R}^2$.

There's a useful relation between the norm and the conjugate: for any $z$,

$$z \bar{z} = |z|^2$$

Furthermore, if you look at this, you'll see it gives an inverse for any nonzero $z \in \mathbb{C}$:

$$z^{-1} = \frac{\bar{z}}{|z|^2}$$

This establishes that $\mathbb{C}$ is a field. 

In comparing $\mathbb{C}$ with $\mathbb{R}$, we can ask: is $\mathbb{C}$ a [totally ordered](http://en.wikipedia.org/wiki/Total_order) field like $\mathbb{R}$ is? The answer in the negative. Suppose $\mathbb{C}$ had some total order. Then we must have $i \leq 0$ or $i \geq 0$. If the latter, then $-1 = i^2 \geq 0$, which is surely nonsense. Similar logic proves that $i \leq 0$ could not hold either, so there can be no total order on $\mathbb{C}$.

In $\mathbb{R}$, non-negative reals have square roots. What about in $\mathbb{C}$? Well, it turns out that we can prove something nice indeed: *every* complex number $z$ has a unique pair of square roots, $\pm w$ for some $w \in \mathbb{C}$.

First, we consider any real number (any $z = (c, d) \in \mathbb{C}$ such that $d = 0$). Nonnegative reals already have unique square roots. For $x \in \mathbb{R}$ such that $x < 0$, squaring 

$$\pm \sqrt{-x} i$$

yields $-x \cdot (-1) = x$. So to summarize, non-negative reals have other non-negative reals as their square roots, and negative reals have purely imaginary numbers as their square roots. (Clearly the converse is true as well: any complex number with a nonnegative real square root must be nonnegative real, and any complex number with a purely imaginary square root must be a negative real).

Finally, suppose that $z = (c, d)$ is not purely real. We give an algebraic definition for its square root $w = (a, b)$, where $a$ and $b$ are defined by

$$a = \sqrt{\frac{c + \sqrt{c^2 + d^2}}{2}}$$

$$b = \frac{d}{|d|} \sqrt{\frac{-c + \sqrt{c^2 + d^2}}{2}}$$

It is a routine matter to prove that $w$ so-defined is a square root.

It remains to prove that square roots are unique. But this is easy: if $w$ and $x$ are two square roots for $z$, then $w^2 = z = x^2$, so $w^2 - x^2 = 0$ and $(w - x)(w + x) = 0$. We must therefore either have 

$$w - x = 0$$

or

$$w + x = 0$$

(because a field has no [zero divisors](http://en.wikipedia.org/wiki/Zero_divisor))

But this implies

$$w = \pm x$$
