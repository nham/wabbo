---
title: Notes on probability, part 2
tags: math, probability
updated: 2015-05-24
---

[Last time][prob-1] we had just defined probability spaces. A [probability space][part1-prob-space] is a mathematical representation of an experiment or any situation where the outcome is not certain. The basic data consists of

 - the set of all possible outcomes, called the **sample space**.
 - a collection of subsets of the sample space whose probability we are able to measure, called the **events**
 - and an actual assignment of probability to each event, called the **probability measure**.

The next task is to analyze the basic properties of these things.

## Null and certain events, some notation {#null-and-certain-events}

But first, a definition and some notation. If $A$ is an event such that $\mathbb{P}(A) = 0$, then it is said to be a **null event**.

Also, it is common to notate the intersection $A \cap B$ of two events by just $A B$ (i.e. omitting the $\cap$). I use this below.

## Basic properties

In what follows, let $(\Sigma, \mathcal{E}, \mathbb{P})$ be a probability space.

If $A$ and $B$ are events such that $A \subseteq B$, then $\mathbb{P}(B - A) = \mathbb{P}(B) - \mathbb{P}(A)$. The proof is by the additivity since $B$ is the disjoint union of $A$ and $B - A$.

Since the probability of any event is non-negative, an immediate corollary is that $\mathbb{P}(A) \leq \mathbb{P}(B)$ whenever $A \subseteq B$.

Another corollary is that $\mathbb{P}(A^c) = 1 - \mathbb{P}(A)$. One way to think of this: the probability of the complement of $A$ is the complement of the probability of $A$.

## Inclusion/exclusion principle

Now it is time for the famous [inclusion/exclusion](http://en.wikipedia.org/wiki/Inclusion%E2%80%93exclusion_principle), which states that

$$\mathbb{P}(A \cup B) = \mathbb{P}(A) + \mathbb{P}(B) - \mathbb{P}(A \cap B)$$

for any events $A$ and $B$. One way to prove it is to first note that, by additivity of disjoint events:

$$\mathbb{P}(A \cup B) = \mathbb{P}(A - B) + \mathbb{P}(B - A) + \mathbb{P}(A \cap B)$$

Now, since

$$\mathbb{P}(A - B) = \mathbb{P}(A \cup B) - \mathbb{P}(B)$$

and

$$\mathbb{P}(B - A) = \mathbb{P}(A \cup B) - \mathbb{P}(A)$$

we can substitute into the first equation. Simplifying yields our desired result.

The intuitive interpretation is that by adding up the probabilities of $A$ and $B$, we are double-counting the probability of $A \cap B$, which is why it must be subtracted.

There's a more general form of the inclusion/exclusion principle for unions of any finite number of events. For example, here's what it looks like with  three events, $A, B, C$.

$$\mathbb{P}(A \cup B \cup C) = \mathbb{P}(A) + \mathbb{P}(B) + \mathbb{P}(C) - \mathbb{P}(A B) - \mathbb{P}(A C) - \mathbb{P}(B C) + \mathbb{P}(A B C)$$

The proof of the general version is a simple proof by induction, so I'll omit it here.

Note the intuitive explanation for the case of three events: in adding up the probabilities of $A$, $B$ and $C$, we double count $A \cap B$, $A \cap C$, and $B \cap C$. In particular this means that the event $A \cap B \cap C$ was *triple counted*, but notice that by subtracting $\mathbb{P}(A B)$,  $\mathbb{P}(A C)$, and $\mathbb{P}(B C)$ that we've subtracted the probability of $A \cap B \cap C$ three times. That is, we've subtracted *too much*, and now $\mathbb{P}(A B C)$ is missing entirely and must be added back in. This is the reason for the name "inclusion/exclusion principle": we bounce back and forth between removing events that were counted too many times and adding in events that weren't counted.


## The continuity of probability measures

In Wasserman's book *All of Statistics*, he calls the following property the *continuity of probability*: If $(A_n)$ is an increasing sequence of events (in the sence that $A_n \subseteq A_{n+1}$ for all $n$), then

$$\mathbb{P}(\bigcup_1^{\infty} A_n) = \lim_{n \to \infty} \mathbb{P}(A_n)$$

Similarly, if $(A_n)$ is instead a decreasing sequence of events, then

$$\mathbb{P}(\bigcap_1^{\infty} A_n) = \lim_{n \to \infty} \mathbb{P}(A_n)$$

Here's the proof of the increasing case: you can partition $\bigcup_1^{\infty} A_n$ into disjoint events $B_1, B_2, \ldots$ defined by

$$B_1 := A_1$$

$$B_{n+1} := A_{n+1} - \bigcup_1 A_n$$

These $B_n$'s are disjoint by definition, and it is easy to verify they union to $\bigcup_1^{\infty} A_n$, so

$$\mathbb{P}(\bigcap_1^{\infty} A_n) = \sum_1^{\infty} \mathbb{P}(B_n)$$

Due to the way we've defined the $B_n$'s it is simple to verify that $\mathbb{P}(A_n) = \sum_1^n \mathbb{P}(B_n)$, which completes the proof.

Why is this called continuity? Recall that continuous functions between metric spaces have a *sequential characterization*: a function $f: X \to Y$ between metric spaces $X$ and $Y$ is continuous at $a \in X$ iff for every sequence $(x_n)$ in $X$ that converges to $a$, the "image sequence" $(f(x_n))$ converges to $f(a)$ in $Y$. Essentially, $f$ is continuous at $a$ precisely when the image of the sequence limit is the limit of the image sequence for any sequence $(x_n) \to a$.

In order to apply this pattern to the present situation, think of $\bigcup_1^{\infty} A_n$ ($\bigcap_1^{\infty} A_n$) as the limit of the increasing (decreasing) sequence $(A_n)$. That's all that this result says.

## Conditional probability

It is often useful to assess the probability of an event $A$ *conditional on some other event $B$ having occurred*. In other words, we assume that we conducted a random trial and obtained some outcome $\omega \in B$, and then we would like to know what is the probability that $\omega$ is also in $A$.

To do this, for any given non-null event $B$, we define the **conditional probability** of $A$ given $B$ to be

$$\mathbb{P}(A | B) := \frac{\mathbb{P}(A B)}{\mathbb{P}(B)}$$

One way to visualize this is as zooming in on the event $B$ so that it's the whole sample space. Then $\mathbb{P}(A | B)$ is the (probability of the) fraction of $B$ that overlaps with $A$.


## An application of conditional probability

First, note that given any events $A$ and $B$, if $B$ is non-null then we must have

$$\mathbb{P}(A B) = \mathbb{P}(B) \mathbb{P}(A | B)$$

And similarly when the roles of $A$ and $B$ are reversed.

Here's one useful application of conditional probability. Given a partition $B_1, \ldots, B_n$ of the sample space such that each $B_i$ is an event and is not null, for any event $A$ we have

$$\mathbb{P}(A) = \sum_1^n \mathbb{P}(A | B_i) \mathbb{P}(B_i)$$

This is because

$$\sum_1^n \mathbb{P}(A | B_i) \mathbb{P}(B_i) = \sum_1^n \mathbb{P}(A B_i)$$

The result follows from this because the sets $A B_i$ partition $A$ ($\bigcup_1^n A B_i = A \cap (\bigcup_1^n B_i) = A \cap \Omega = A$, and they are disjoint since the $B_i$'s are).


## Independent events

Let us say two events $A$ and $B$ are **independent** whenever

$$\mathbb{P}(A B) = \mathbb{P}(A) \mathbb{P}(B)$$

Note that for any null event $A$, every event $B$ forms an independent pair of events with $A$ (because $A B \subseteq A$, so $0 \leq \mathbb{P}(A B) \leq 0$).

If $A$ and $B$ are independent events, then whenever $B$ is non-null we must have (see above) that

$$\mathbb{P}(A) = \mathbb{P}(A | B)$$

So that's the interpretation for an independent pair of events: knowing that $B$ occurred does not change the probability that $A$ occurred.


## Bayes' Theorem

If both $A$ and $B$ are non-null, then Bayes' theorem states that

$$\mathbb{P}(A | B) = \frac{\mathbb{P}(B | A) \mathbb{P}(A)}{\mathbb{P}(B)}$$

The proof is just expanding the equation

$$\mathbb{P}(A B) = \mathbb{P}(A B)$$

by using conditional probability in two different ways.

In this case, $\mathbb{P}(A)$ ($\mathbb{P}(B)$) is generally called the **prior probability of $A$ ($B$)**, and $\mathbb{P}(A | B)$ ($\mathbb{P}(B | A)$) is called the **posterior probability of $A$ given $B$ ($B$ given $A$)**.


## Conclusion

I'm looking at doing random variables in the next post, provided I didn't miss anything major above.


[prob-1]: /entries/2015-04-27-probability-1.html
[part1-prob-space]: /entries/2015-04-27-probability-1.html#probability-spaces
