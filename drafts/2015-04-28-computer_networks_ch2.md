---
title: Notes on "Computer Networks: A Systems Approach", Ch. 2
tags: networking
---

## How to network

To even begin thinking about networking, you need some medium that electromagnetic waves can propagate in. Examples of such mediums include space (in the case of radio waves) or a length of wire or optical fiber. Such a physical medium is called a **link**.

Once you have solved the problem of making links, there are still five more problems to solve, according to Peterson and Davie:

 1. You have a medium for sending electromagnetic waves on, but you really want to transmit bits. How can you encode the bits into an electromagnetic signal?
 2. We are concerned with packet-switched networks, which means that we are interested here in sending collections of bits called **packets** or **frames**. The **framing problem** is this: how does the sender convey where any given frame begins and ends?
 3. The physical medium is unreliable; frames can get corrupted. Can we detect when this corruption occurs?
 4. Can we engineer an arrangement that makes the link seem reliable even when it's not? This might involve correcting errors, or possibly other things.
 5. Finally, for multiple-access links, something needs to mediate access to it (to ensure that some collection of nodes don't monopolize it, for example?).


## The framing problem

The sender must send bits in such a way that the receiver can see where a frame begins and ends. Without this, the receiver just sees a stream of bits. Instead the receiver will see a stream of frames.
