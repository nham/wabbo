# Toy floating point

## Strings of symbols

If we have a collection $S$ of symbols, then a **string** of symbols is just a finite sequence $x_1 x_2 \cdots x_{n-1} x_n$, where each $x_i \in S$.

## Positional number systems

If we have some fixed integer $b$, where $b > 1$, then any nonnegative integer can be obtained by a sum

$$a_{n-1} b^{n-1} + a_{n-2} b^{n-2} + \cdots + a_1 b + a_0 = \sum_{i=0}^{n-1} a_i b^i$$

for some $a_0, \ldots, a_{n-1}$, where each $a_i$ is an integer such that $0 \leq a_i < b$. 

Let's turn this into a definition: for any fixed integer $b$ with $b > 1$, then for any nonnegative integer $k$, the tuple $(a_{n-1}, a_{n-2}, \ldots, a_1, a_0)$ is a **representing tuple** for $k$ with respect to **base** $b$ provided that

 - each $a_i$ is in the range $0 \leq a_i < b$
 - $k = \sum_{i=0}^{n-1} a_i b^i$

It is easy to see that in any base, any nonnegative integer has infinitely many representing tuples: if $(a_{n-1}, \ldots, a_0)$ is a representing tuple, then so is $(0, a_{n-1}, \ldots, a_0)$ (since this just adds $0 * b^n = 0$ to the sum). If we disallow such "leading zeroes" in our representing tuples, however, then each nonnegative integer has a unique representing tuple (we must take care to allow the tuple $(0)$, as it is the most convenient way to represent zero).

Let us introduce a quick notational abbreviation: we will write representing tuples in a base $b$ as $(a_{n-1} a_{n-2} \cdots a_1 a_0)_b$. Here we have dropped the commas from the tuple representation and also added a subscript $b$ to indicate which base we are working in.

A **positional number system** is a system for writing numbers with respect to some base. For each integer $i$ in the range $0 \leq i < b$, we introduce a symbol $s_i$, so that the string of symbols $s_{i_1} \cdots s_{i_n}$ is associated with the representing tuple $(i_1 \cdots i_n)_b$, which is associated with some actual nonnegative integer (as established above).

For example, our own number system is the decmal system. The base is ten, and we use the symbols "0", "1", "2", "3", "4", "5", "6", "7", "8", "9" to represent the ten smallest nonnegative integers. Another example is the binary system, which is base two and uses the symbols "0" and "1".

## Two's complement

It turns out that numbers are pretty useful. Computers are pretty useful too, and being able to use numbers on computers is super useful. So we have a need to represent numbers on computers. Computers use binary, so it seems that we can repurpose the binary number system for use in computers. A slight adjustment is needed. TODO

Let's talk about bit strings. A **bit string** is some finite tuple $(x_0, \ldots, x_{n-1})$ where each $x_i$ is $0$ or $1$. Each such $x_i$ is called a **bit**. A bit string of length $n$ is a tuple of bits of length $n$, for example $(x_0, \ldots, x_{n-1})$. We will represent bit string of length $n$ by $x_0 x_1 \cdots x_{n-1}$.

So a bit string is just some finite sequence of bits. We can do a lot of things with a sequence of bits. One thing we can do is use them to represent nonnegative integers. The **$n$-bit unsigned integer type** consists of all bit strings of length $n$, where the strings are interpreted as representing nonnegative integers. For example, with the 8-bit unsigned integer type, the string $00000000$ represents the integer $0$, the string $11111111$ represents the integer $255$, and the string $01010001$ represents the integer $81$.

An $n$-bit unsigned integer type consists of all possible n-bit strings where the strings are taken to represent non-negative integers. In this type there are $2^n$ distinct values, with the string $0 \cdots 0$ representing the integer $0$ and the string $1 \cdots 1$ representing the integer $2^n - 1$. A common example is the 8-bit unsigned integer type, which can represent values 0 through 255.
