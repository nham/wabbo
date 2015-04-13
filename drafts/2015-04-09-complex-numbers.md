---
title: The complex numbers
date: April 12, 2015
tags: math, complex numbers
---

Here I define the complex numbers and prove basic properties about them. The approach I use involves perhaps too much machinery for this to be an intuitive first introduction, but I like it because a) it's fairly compact and b) emphasizes that complex numbers can be thought of as vectors in the plane.

## Definition

Start with $\mathbb{R}^2$. It's a [vector space](http://en.wikipedia.org/wiki/Vector_space). To define a multiplication operation between vectors in $\mathbb{R}^2$, we will specify an [algebra](http://en.wikipedia.org/wiki/Algebra_over_a_field) on $\mathbb{R}^2$. That is, we're defining a function $\mathbb{R}^2 \times \mathbb{R}^2 \to \mathbb{R}^2$ that is [bilinear ](https://en.wikipedia.org/wiki/Bilinear_map).

(Perhaps the bilinearity condition is a bit mystifying, but if you think about it, any operation worthy of the name "multiplication" should at least be bilinear, since we want to be able to do $(a + b) \cdot c = a \cdot c + b \cdot c$, and also $(a \cdot b) \cdot c = a \cdot (b \cdot c)$)

Recall from linear algebra that for any vector spaces $V$ and $W$ and any basis $B$ for $V$, any mapping $B \to W$ uniquely determines a linear map $f: V \to W$ (such that $f$ takes on the same values for vectors in $B$). A similar fact is true for bilinear maps, except that it is *pairs* of basis vectors that determine the rest of the bilinear map.

With this in mind, we choose a basis for $\mathbb{R}^2$ and specify the multiplication between basis vectors. Let us define $\textbf{1} := e_1 \in \mathbb{R}^2$ and $\textbf{i} := e_2 \in \mathbb{R}^2$ ($e_1$ and $e_2$ are the [standard basis vectors](https://en.wikipedia.org/wiki/Standard_basis) of $\mathbb{R}^2$). Then the **complex numbers** $\mathbb{C}$ are defined to be the algebra $(\mathbb{R}^2, \cdot)$ where the multiplication is completely determined by these rules:

$$\begin{align}
\textbf{1} \cdot \textbf{1} &= \textbf{1} \\
\textbf{1} \cdot \textbf{i} &= \textbf{i} \\
\textbf{i} \cdot \textbf{1} &= \textbf{i} \\
\textbf{i} \cdot \textbf{i} &= -\textbf{1}
\end{align}$$

(More concisely, we define $\textbf{1}$ to be an identity for the multiplication, and define $\textbf{i}^2 = -1$)

This is a bit abstract. It'd be nice to have a concrete formula for complex multiplication. Luckily that is not too much trouble. To find what $(a + ib) \cdot (c + id)$ is, we just use the bilinearity of the multiplication:

$$(a + ib)(c + id) = ac + iad + ibc + i^2 bd = ac - bd + i(ad + bc)$$

## $\mathbb{C}$ is a field

The product turns out to be associative and commutative as well (check it if you wish), so $\mathbb{C}$ with vector addition and the algebra product is a [commutative ring](http://en.wikipedia.org/wiki/Commutative_ring).

If $\mathbb{C}$ turned out to be a [division algebra](http://en.wikipedia.org/wiki/Division_algebra) as well, then $\mathbb{C}$ would be a field, just like $\mathbb{R}$. In fact, we can do this.

First, define the **complex conjugate** of any $z = a + ib \in \mathbb{C}$ is defined to be

$$\bar{z} = a - ib$$

Let's also define the **norm** of $z$ to be

$$|z| := \sqrt{a^2 + b^2}$$

That is, the standard Euclidean norm on $\mathbb{R}^2$.

There's a useful relation between the norm and the conjugate: for any $z$,

$$z \bar{z} = |z|^2$$

Furthermore, if you look at this, you'll see it gives an inverse for any nonzero $z \in \mathbb{C}$:

$$z^{-1} = \frac{\bar{z}}{|z|^2}$$

This establishes that $\mathbb{C}$ is a field. 

## Real and imaginary parts

Here are a few more useful definitions: for any complex number $z := a + ib$, the **real part** of $z$ is defined to be

$$\text{Re}(z) := a$$

and the **imaginary part** of $z$ is

$$\text{Im}(z) := b$$

We can think of $\text{Re}$ and $\text{Im}$ as functions $\mathbb{C} \to \mathbb{R}$. It is quite easy to verify that $\text{Re}(z+w) = \text{Re}(z) + \text{Re}(w)$ and $\text{Im}(z+w) = \text{Im}(z) + \text{Im}(w)$.

Is the same true for multiplication? That is, does $\text{Re}(zw) = \text{Re}(z) + \text{Re}(w)$ (ditto for $\text{Im}$)? A quick glance at the formula for multiplication shows this is not true in general, since 

$$\text{Re}(zw) = ac - bd \neq ac$$

and

$$\text{Im}(zw) = ad + bc \neq bd$$

We can prove something else though: if $x \in \mathbb{C}$ is real, then

$$\text{Re}(xz) = xa = x \text{Re}(z)$$

and

$$\text{Im}(xz) = xb = x \text{Re}(z)$$

In other words, we've just proven $\text{Re}$ and $\text{Im}$ are real linear functions. (In fact, recall from linear algebra that every field is a (one-dimensional) vector space over itself. Furthermore, you can show that $\mathbb{C}$ is a vector space over $\mathbb{R}$. This means that $\text{Re}$ and $\text{Im}$ are linear maps from the vector space $(\mathbb{C}, \mathbb{R})$ to $(\mathbb{R}, \mathbb{R})$.

## Can $\mathbb{C}$ be totally ordered?

In comparing $\mathbb{C}$ with $\mathbb{R}$, we can ask: is $\mathbb{C}$ a [totally ordered](http://en.wikipedia.org/wiki/Total_order) field like $\mathbb{R}$ is? The answer is no: suppose $\mathbb{C}$ had some total order. Then we must have $i \leq 0$ or $i \geq 0$. If the latter, then $-1 = i^2 \geq 0$, which is surely nonsense. Similar logic proves that $i \leq 0$ could not hold either, so there can be no total order on $\mathbb{C}$.

## Square roots

In $\mathbb{R}$, non-negative reals have square roots. What about in $\mathbb{C}$? Well, it turns out that we can prove something nice indeed: *every* complex number $z$ has a unique pair of square roots, $\pm w$ for some $w \in \mathbb{C}$. (Note that this is different from $\mathbb{R}$: only nonnegative reals have square roots).

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
