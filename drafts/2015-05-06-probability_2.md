---
title: Notes on probability, part 2
tags: math, probability
---

[Last time](2015-04-27-probability-1.html) we had just defined probability spaces. The next task is to analyze the basic properties of these things.

## Basic properties

In what follows, let $(\Sigma, \mathcal{E}, \mathbb{P})$ be a probability space.

If $A$ and $B$ are events such that $A \subseteq B$, then $\mathbb{P}(B - A) = \mathbb{P}(B) - \mathbb{P}(A)$. The proof is by the additivity since $B$ is the disjoint union of $A$ and $B - A$.

An immediate corollary is that $\mathbb{P}(A^c) = 1 - \mathbb{P}(A)$. One way to think of this: the probability of the complement of $A$ is the complement of the probability of $A$.

## Inclusion/exclusion principle

Now it is time for the famous [inclusion/exclusion](http://en.wikipedia.org/wiki/Inclusion%E2%80%93exclusion_principle), which states that

$$\mathbb{P}(A \cup B) = \mathb{P}(A) + \mathbb{P}(B) - \mathbb{P}(A \cap B)$$

for any events $A$ and $B$. One way to prove it is to first note that, by additivity of disjoint events:

$$\mathbb{P}(A \cup B) = \mathb{P}(A - B) + \mathbb{P}(B - A) + \mathbb{P}(A \cap B)$$

Now, since

$$\mathbb{P}(A - B) = \mathbb{P}(A \cup B) - \mathbb{P}(B)$$

and

$$\mathbb{P}(B - A) = \mathbb{P}(A \cup B) - \mathbb{P}(A)$$

we can substitute into the first equation. Simplifying yields our desired result.

The intuitive interpretation is that by adding up the probabilities of $A$ and $B$, we are double-counting the probability of $A \cap B$, which is why it must be subtracted.

There's a more general form of the inclusion/exclusion principle for unions of any finite number of events. For example, here's what it looks like with  three events, $A, B, C$.

$$\mathbb{P}(A \cup B \cup C) = \mathb{P}(A) + \mathbb{P}(B) + \mathbb{P}(C) - \mathbb{P}(A \cap B) - \mathbb{P}(A \cap C) - \mathbb{P}(B \cap C) + \mathbb{P}(A \cap B \cap C)$$

The proof of the general version is a simple proof by induction, so I'll omit it here.

Note the intuitive explanation for the case of three events: in adding up the probabilities of $A$, $B$ and $C$, we double count $A \cap B$, $A \cap C$, and $B \cap C$. In particular this means that the event $A \cap B \cap C$ was *triple counted*, but notice that by subtracting $\mathbb{P}(A \cap B)$,  $\mathbb{P}(A \cap C)$, and $\mathbb{P}(B \cap C)$ that we've subtracted the probability of $A \cap B \cap C$ three times. That is, we've subtracted *too much*, and now $\mathbb{P}(A \cap B \cap C)$ is missing entirely and must be added back in. This is the reason for the name "inclusion/exclusion principle": we bounce back and forth between removing events that were counted too many times and adding in events that weren't counted.


## Todo

Discuss limit of a sequence of probabilities for a nested sequence of events. Wasserman calls this (and he's right) the continuity of probability.

Define condition probability $\mathbb{P}(A | B)$ when $\mathbb{P}(B) > 0$.

If you have a partition of the sample space by events, with each partition having non-zero probability, you can decompose probability of an event using conditional probability.

Bayes theorem? It's just writing out the definition of conditional probability?

Independent events. Show how it relates to conditional probability.
