This post assumes knowledge of basic group theory. Groups are not technically a requirement for these definitions, but I use them as examples to make what follows slightly more concrete.

This is just my notes on a very small part of  John Baez and Mike Stay's paper [Physics, Topology, Logic and Computation: A Rosetta Stone][rosetta]. You should probably read that instead of this post.

I use post-fix function notation throughout ($xf$ instead of $f(x)$ and $f;g$ instead of $g \circ f$) because I find it to be clearest for this kind of math.

## Categories

A (locally small) **category** is a collection (not necessarily a set!) of *objects* along with, for any two objects $X$ and $Y$, a set $hom(X, Y)$ of *arrows* or *morphisms*, such that

 - any two morphisms $f: X \to Y$ and $g: Y \to Z$ can can be *composed* to form a morphism $f ; g: X \to Z$. The composition operation is associative.
 - every object $X$ has a distinguished *identity morphism* $1_X: X \to X$ which acts as a unit in the sense that $1_X ; f = f$ and $g ; 1_X = g$ for any $f: X \to Y$ and any $g: Z \to X$.

Each $hom(X, Y)$ is called the *homset*.

If this definition is too much to wrap your head around, you may want to consider the definition of a [quiver][quiver-wiki] instead. A **quiver** is a directed graph that permits loops (edges that start and end at the same node) and multiple edges between the same two nodes. A category can then be defined as a quiver that has a) an associative composition operator and b) a special loop for each object that acts as a unit under composition.

Why care about this? A very partial answer is this quote from Baez and Stay:

 > A category is the simplest framework where we can talk about systems (objects) and processes (morphisms)

A morphism $f: X \to Y$$ in a category is said to be an **isomorphism** when there is a $g: Y \to X$ such that $f ; g = 1_X$ and $g ; f = 1_Y$. In this case, $g$ must be unique and is called the **inverse** of $f$ (and is sometimes denoted $f^{-1}$), since for any inverse $h: Y \to X$ we have

$$h = h ; 1_X = h ; f ; g = 1_Y ; g = g$$

The definition is symmetric, so each inverse $f^{-1}$ is also an isomorphism. It is also true that every identity morphism $1_X$ is an isomorphism on $X$. One last important fact about isomorphisms is that they are closed under composition: if $f: X \to Y$ and $g: Y \to Z$ are both isomorphisms, then $f;g$ is also an isomorphism with inverse $g^{-1} ; f^{-1}$.

Gathering up all the facts from the last paragraph, the relation on objects defined by $X \sim Y$ iff there exists an isomorphism $X \to Y$ is an equivalence relation. I think this is important because in category theory we are often concerned not with proving that two things are equal, but that they are isomorphic (i.e. that they belong to the same isomorphism class).

## Functors

If $C$ and $D$ are categories, then a **functor** $F: C \to D$ maps objects and morphisms in $C$ to objects and morphisms in $D$ such that

 - identities are preserved: for every object $X$ in $C$, $F(1_X) = 1_{F(X)}$
 - compositions are preserved: for any morphisms $f: X \to Y$ and $g: Y \to Z$, $F(f;g) = F(f) ; F(g)$

[rosetta]: http://math.ucr.edu/home/baez/rosetta.pdf
[quiver-wiki]: https://en.wikipedia.org/wiki/Quiver_%28mathematics%29
