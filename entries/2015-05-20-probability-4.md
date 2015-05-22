---
title: Notes on probability, part 4: Odds, the factor principle, decibanage
tags: math, probability
---

Previously: [part 1][part1], [part 2][part2], [part 3][part3]

This post was inspired by a recently declassified text written by Alan Turing titled [The Applications of Probability to Cryptography][turing-prob-crypto]. In it he gives a brief overview of some notions from probability, and I liked a few of the sections enough to try to summarize them here.

## Odds

The odds of an event $A$ occurring is defined to be the ratio

$$\frac{\mathbb{P}(A)}{1 - \mathbb{P}(A)}$$

This is undefined when $\mathbb{P}(A) = 1$, but in what follows we will exclude events that are null or uncertain altogether, so don't worry about that. Denote the odds ratio of event $A$ by $O(A)$.

![$P/(1 - P)$ versus $P$](/images/plot_odds.png)

Note that the expression used to convert from an odds ratio to a probability is

$$\mathbb{P}(A) = \frac{O(A)}{1 + O(A)}$$

For example, an odds ratio of $\frac{m}{n}$ corresponds to an event probability of $\frac{m}{m+n}$. An odds ratio like this is usually written $m:n$ instead.

A few examples are probably needed to make the concept more familiar. An almost trivial example is the odds ratio of heads from a fair coin toss, which is $1:1$.

## The factor principle

You should know that [Bayes' theorem][part2] relates the conditional probabilities $\mathbb{P}(A | B)$ and $\mathbb{P}(B | A)$. Specifically, it says that

$$\mathbb{P}(A | B) = \frac{\mathbb{P}(A) \mathbb{P}(B | A)}{\mathbb{P}(B)}$$

We can obtain a slightly different and useful way of thinking about Bayes' theorem if we divide both sides by $\mathbb{P}(\neg A | B)$:

$$\frac{ \mathbb{P}(A | B) }{ \mathbb{P}(\neg A | B) } = \frac{ \mathbb{P}(A) \ \mathbb{P}(B | A) \ \mathbb{P}(B) }{ \mathbb{P}(B) \ \mathbb{P}(\neg A B) }$$

where I've used the definition of conditional probability to expand $\mathbb{P}(\neg A | B)$ on the right hand side. This equation simplifies to

$$\frac{\mathbb{P}(A | B)}{\mathbb{P}(\neg A | B)} = \frac{\mathbb{P}(A)}{\mathbb{P}(\neg A)} \frac{\mathbb{P}(B | A)}{\mathbb{P}(B | \neg A)}$$

Turing calls this equation the **factor principle**. We can simplify it further if we invent a notion of conditional odds ratio, $O(A | B) := \mathbb{P}(A | B)  / \mathbb{P}(\neg A | B)$. Then the equation can be written:

$$O(A | B) = O(A) \frac{\mathbb{P}(B | A)}{\mathbb{P}(B | \neg A)}$$

Compare this to Turing's version:

![Turing's formulation of the factor principle](/images/turing_factor_principle.png)\

Comparing this with my formulation above, $A$ is the theory, and $B$ is the data or the evidence. Turing calls the ratio on the right the *factor*, but I've also seen it called the *likelihood ratio*.


### An example

Turing provides quite a morbid example for how to use the factor principle:

 > Suppose that one man in five dies of heart failure, and that of the men who die of heart failure two in three die in their beds, but of the men who die from other causes only one in four die in their beds. (My facts are no doubt hopelessly inaccurate). Now suppose we know that a certain man died in his bed. What is the probability that he died of heart failure?

Let $H$ be the event that a man dies of heart failure, and $B$ the event that a man dies in their bed. We are given that

$$\begin{align}
\mathbb{P}(H) &= 1/5 \\
\mathbb{P}(B | H) &= 2/3 \\
\mathbb{P}(B | \neg H) &= 1/4
\end{align}$$

We would like to now find $\mathbb{P}(H | B)$. From the givens we have that $O(H) = 1/4$, and the factor is $8/3$, so the *a posteriori* odds ($O(H | B)$) are $2/3$.

One key point is to look at is whether the factor is less than or greater than 1. A factor greater than 1 means that the observed evidence increases our belief that the theory is true, and similarly a factor less than 1 decreases it.

## Log odds and decibanage

There is one last reformulation of Bayes' theorem that gives a useful perspective. Recall that in the factor principle, the factor is something we multiply our *a priori* odds by to obtain the a posteriori odds. Logarithms turn products into sums of course, so we obtain the following equation by taking the log (the base can be whatever, but here it is 10) of both sides:

$$\log O(A | B) = \log O(A) + \log \frac{\mathbb{P}(B | A)}{\mathbb{P}(B | \neg A)}$$

Quantities of the form $\log O(A)$ or $\log O(A | B)$ are (naturally enough) called *log odds*. We can also express this in terms of the [logit][wiki-logit] function:

$$\text{logit}(P) := \log \frac{P}{1 - P}$$

![$\text{logit}(P)$ versus $P$](/images/plot_log_odds.png)

The term

$$\log \frac{\mathbb{P}(B | A)}{\mathbb{P}(B | \neg A)}$$

is called the *banage in favor of the theory $B$*. A *ban* is a unit of evidence.

Why introduce this? If we have multiple pieces of evidence to consider (which is often the case), it can be more intuitive to work with sums than products.

TODO

[part1]: /entries/2015-04-27-probability-1.html
[part2]: /entries/2015-05-11-probability-2.html
[part3]: /entries/2015-05-19-probability-3.html
[turing-prob-crypto]: http://arxiv.org/abs/1505.04714
[wiki-logit]: http://en.wikipedia.org/wiki/Logit
