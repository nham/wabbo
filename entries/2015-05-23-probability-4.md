---
title: Notes on probability, part 4: Odds, the factor principle, decibanage
tags: math, probability
---

Previously: [part 1][part1], [part 2][part2], [part 3][part3]

This post was inspired by a recently declassified text written by Alan Turing titled [The Applications of Probability to Cryptography][turing-prob-crypto]. In it he gives a brief overview of some notions from probability, and I liked a few of the sections enough to try to summarize them here.

## Odds

The odds of an event $A$ occurring is defined to be the ratio

$$\frac{\mathbb{P}(A)}{1 - \mathbb{P}(A)}$$

This is undefined when $\mathbb{P}(A) = 1$, but in what follows we will exclude events that are null or certain altogether, so don't worry about that. Denote the odds ratio of event $A$ by $O(A)$.

![$P/(1 - P)$ versus $P$](/images/plot_odds.png)

Note that the expression used to convert from an odds ratio to a probability is

$$\mathbb{P}(A) = \frac{O(A)}{1 + O(A)}$$

For example, an odds ratio of $\frac{m}{n}$ corresponds to an event probability of $\frac{m}{m+n}$. An odds ratio like this is usually written $m:n$ instead.

A few examples are probably needed to make the concept more familiar. An almost trivial example is the odds ratio of heads from a fair coin toss, which is $1:1$. Another example rolling two dice and summing the results:

Sum  Probability   Odds
--- ------------- ------
  2      1/36       1:35
  3      2/36       1:17
  4      3/36       1:11
  5      4/36        1:8
  6      5/36       5:31
  7      6/36        1:5
  8      5/36       5:31
  9      4/36        1:8
 10      3/36       1:11
 11      2/36       1:17
 12      1/36       1:35


Two more examples: the probability of rolling 7 or higher in this $21/36$, which corresponds to odds of $7:5$, while the probability of rolling 9 or higher is $10/36$, which corresponds to odds of $5:13$.


## The factor principle

You should know that [Bayes' theorem][part2] relates the conditional probabilities $\mathbb{P}(A | B)$ and $\mathbb{P}(B | A)$. Specifically, it says that

$$\mathbb{P}(A | B) = \frac{\mathbb{P}(A) \mathbb{P}(B | A)}{\mathbb{P}(B)}$$

We can obtain a slightly different and useful way of thinking about Bayes' theorem if we divide both sides by $\mathbb{P}(\neg A | B)$:

$$\frac{ \mathbb{P}(A | B) }{ \mathbb{P}(\neg A | B) } = \frac{ \mathbb{P}(A) \ \mathbb{P}(B | A) \ \mathbb{P}(B) }{ \mathbb{P}(B) \ \mathbb{P}(\neg A B) }$$

where I've used the definition of conditional probability to expand $\mathbb{P}(\neg A | B)$ on the right hand side. This equation simplifies to

$$\frac{\mathbb{P}(A | B)}{\mathbb{P}(\neg A | B)} = \frac{\mathbb{P}(A)}{\mathbb{P}(\neg A)} \frac{\mathbb{P}(B | A)}{\mathbb{P}(B | \neg A)}$$

Turing calls this equation the **factor principle**. We can simplify it further if we invent a notion of conditional odds ratio, $O(A | B) := \mathbb{P}(A | B)  / \mathbb{P}(\neg A | B)$. Then the equation can be written:

$$O(A | B) = O(A) \frac{\mathbb{P}(B | A)}{\mathbb{P}(B | \neg A)}$$

Here is Turing's version:

![Turing's formulation of the factor principle](/images/turing_factor_principle.png)\

Comparing this with the formulation above, $A$ is the theory, and $B$ is the data or the evidence. Turing calls the ratio on the right the *factor*, but I've also seen it called the *likelihood ratio*. In the paper *Studies in the history of probability and statistics. XXXVII AM Turing's statistical work in World War II*, I. J. Good (who was Turing's assistant in statistics during World War II) calls this the **Bayes factor**.


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

Quantities of the form $\log O(A)$ or $\log O(A | B)$ are (naturally enough) called *log odds*. This can also be expressed in terms of the [logit][wiki-logit] function:

$$\text{logit}(P) := \log \frac{P}{1 - P}$$

![$\text{logit}(P)$ versus $P$](/images/plot_log_odds.png)

The term

$$\log \frac{\mathbb{P}(B | A)}{\mathbb{P}(B | \neg A)}$$

is called the *banage in favor of the theory $B$*. A [ban][wiki-ban] is a unit of evidence or information, which Turing says was named after "the famous town of Banbury" (I've never heard of it prior to now).

Why introduce this? If we have multiple pieces of evidence to consider (which is often the case), it can be more intuitive to work with sums than products.

One issue is that 1 ban is quite a lot of evidence. In order to have

$$\log \frac{\mathbb{P}(B | A)}{\mathbb{P}(B | \neg A)} = 1$$

the factor $\mathbb{P}(B | A) / \mathbb{P}(B | \neg A)$ must equal 10. Recall that in the example above with heart disease and men dying in their beds, the factor was less than 3. So it is often more convenient to use a smaller unit. For this reason Turing introduces the *deciban*, or one-tenth of a ban. I. J. Good says of the deciban:

 > A deciban or half-deciban is about the smallest change in weight of evidence that is directly perceptible to human intuition.

Given a factor $F$, how many decibans is it? Well, $F = 10^{k/10}$ would be $k$ decibans, so $10 \log F$ should give the number of decibans in general. Going back to our heart attack example, the factor gives $10 \log 8/3 \approx 4.26$ decibans of evidence for the theory that the man died of a heart attack.

[part1]: /entries/2015-04-27-probability-1.html
[part2]: /entries/2015-05-11-probability-2.html
[part3]: /entries/2015-05-19-probability-3.html
[turing-prob-crypto]: http://arxiv.org/abs/1505.04714
[wiki-logit]: http://en.wikipedia.org/wiki/Logit
[wiki-ban]: http://en.wikipedia.org/wiki/Ban_%28unit%29
