---
title: Notes on hash functions and Bloom filters
---

## Preface

Here are some attempts to learn about hash functions and Bloom filters.

## Hash functions

Suppose we have a function $h: U \to V$, 

where 

 - $U$ is some set whose elements are called **keys** and 
 - $V$ is some finite set.

For any $k \in U$, $h(k)$ is the **hash value** or **hash** of $k$. You could call $U$ the **key space** and $V$ the **hash space**.


Now consider the procedure where

 1. we choose keys at random, each key having an equal probability of being chosen
 2. after a key is chosen, we hash it.

In this way we obtain a probability distribution on the hashes in $V$. $h$ is then said to be an **ideal hash function** if the distribution on $V$ is the [uniform distribution](http://en.wikipedia.org/wiki/Uniform_distribution_(discrete)). Also called a **simple uniform hash function**.

A **hash function** is one that is "approximately" an ideal hash function, for some definition of "approximately".
