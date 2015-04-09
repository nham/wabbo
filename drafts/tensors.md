---
title: Notes on tensors
date: November 20, 2014
tags: foo, bar1, bar2
---

## Vector spaces of scalar-valued functions.

If $\mathbb{F}$ is a field, then hopefully you understand how the set $\mathbb{F}^n$ forms a vector space. As a review, the vectors of $\mathbb{F}^n$ are $n$-tuples of scalars in $\mathbb{F}$, like $(a_1, \ldots, a_n)$. Vector addition is defined *component-wise*, meaning for two different $n$-tuples in $\mathbb{F}^n$, we match up the same components and add them according to addition in the field $\mathbb{F}$. 

So letting $(a_1, \ldots, a_n)$ and $(b_1, \ldots, b_n)$ be any $n$-tuples of scalars, we have:

$$(a_1, \ldots, a_n) + (b_1, \ldots, b_n) = (a_1 + b_1, \ldots, a_n + b_n)$$

Scalar multiplication of an $n$-tuple also defined *component-wise* as the scaling of the components of the tuple individually:

$$c \cdot (a_1, \ldots, a_n) = (c a_1, \ldots, c a_n)$$

for any $c \in \mathbb{F}$.

If you check all the vector space axioms, these two definitions make everything work out.

An identical way of thinking about tuples is as functions $[n] \to \mathbb{F}$, where $[n]$ is the notation I'll be using for the set \{1, \ldots, n \}. If $f$ is such a function, then the $i$-th component is now found as the value $f(i)$. (If you want to be pedantic, you can construct a bijection between the set of all $n$-tuples taking values in $\mathbb{F}$ and the set of all functions $\{1, \ldots, n\} \to \mathbb{F}$.) 

Using this slight reformulation, we can define a vector space whose vectors are all functions $[n] \to \mathbb{F}$. Vector addition and scalar multiplication are still defined component-wise, it just looks slightly different. For any functions $f, g : [n] \to \mathbb{F}$, the addition is defined:

$$(f + g)(i) = f(i) + g(i)$$

for all $i \in [n]$. Scalar multiplication is similarly defined:

$$(c \cdot f)(i) = c f(i)$$

for all $i \in [n]$.

If you stare hard enough at the above definitions and at the definitions for the case of $n$-tuples, you should see that they are virtually the same definitions. The "components" of the function are the values of the function at each $i \in [n]$, and addition of functions is defined by adding components together. Similarly, scalar multiplication is defined as scaling each of the components.

We now generalize the above by considering a vector space not of functions $[n] \to \mathbb{F}$, but of functions $X \to \mathbb{F}$ for any *arbitrary* set $X$. Such functions can be thought of as "generalized tuples". There may not be any definite order to the set $X$, and $X$ may in fact contain uncountably many elements, so that it looks very much unlike a tuple. None of that matters, however, as the "component-wise" definitions that we used above work all the same.

To make that last paragraph concrete, we will denote by $\mathbb{F}^X$ the vector space of all functions $X \to \mathbb{F}$ with vector addition defined by $(f+g)(x) = f(x) + g(x)$ and $(c \cdot f)(x) = c f(x)$ for all $x \in X$.

We can also generalize the idea of a standard basis for $\mathbb{F}^n$ to a kind of standard basis for the space $\mathb{F}^X$. We do this by considering a slight variant of the [Kronecker delta function](http://en.wikipedia.org/wiki/Kronecker_delta). Define for all $z \in X$ a function $\delta_z : X \to \mathbb{F}$ with

$$
\delta_z(x) = 
\begin{cases}
1 & x = z \\
0 & \text{otherwise}
\end{cases}
$$

In words, the function $\delta_z$ reads "1" for $z$ and reads "0" for all other elements of $X$. Intuitively, $\delta_z$ is a "$z$-detector". Since the functions $\delta_z$ are functions $X \to \mathbb{F}$, they are elements of $\mathbb{F}^X$. It can be easily proved that any $f \in \mathbb{F}^X$ can be obtained as a linear combination of the $\delta_z$ functions, where the coefficient of $\delta_z$ is scalar $f(z)$. In symbols:

$$f = \sum_{x \in X} f(x) \cdot \delta_x$$

Proving this is a straightforward application of the definitions of addition and scalar multiplication in $\mathb{F}^X$ and the definitions of $\delta_x$'s. So the collection of $\delta_x$'s generates $\mathbb{F}^X$. Furthermore, it is an easy proof that this set is independent. So the collection of $\delta_x$'s is a basis for $\mathbb{F}^X$. I hope you see how they are completely analogous to the elements of the standard basis for $\mathbb{F}^n$.


## Subspaces of finite support and free vector spaces


## Direct products of vector spaces


## Quotient spaces


## Tensor products
