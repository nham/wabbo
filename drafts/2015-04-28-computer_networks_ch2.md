---
title: Notes on "Computer Networks: A Systems Approach", Ch. 2
tags: networking
---

## How to network

To even begin thinking about networking, you need some medium that electromagnetic waves can propagate in. Examples of such mediums include space (in the case of radio waves) or a length of wire or optical fiber. Such a physical medium is called a **link**.

Two major flavors of links are [point-to-point](http://en.wikipedia.org/wiki/Point-to-point_%28telecommunications%29) links (connections are between pairs of nodes) and [multiple-access](https://en.wikipedia.org/wiki/Channel_access_method) links.

Once you have solved the problems of making physical links, there are still five more problems to solve, according to Peterson and Davie:

 1. You have a medium for sending electromagnetic waves on, but you really want to transmit some bits. How can you encode the bits into an electromagnetic signal?
 2. We are concerned with packet-switched networks, which means that we are interested here in sending collections of bits called **packets** or **frames**. The **framing problem** is this: how does the sender convey where any given frame begins and ends?
 3. The physical medium is unreliable; frames can get corrupted. Can we detect when this corruption occurs?
 4. Can we engineer an arrangement that makes the link seem reliable even when it's not? This might involve correcting errors, or possibly other things.
 5. Finally, for multiple-access links, something needs to mediate access to it (to ensure that some collection of nodes don't monopolize it, for example?).


## The framing problem

The sender must send bits in such a way that the receiver can see where a frame begins and ends. Without this, the receiver just sees a stream of bits. Instead the receiver will see a stream of frames.


### Byte-oriented protocols (BISYNC, PPP, DDCMP)

#### Sentinel-based approaches

BISYNC

   8     8     8              8            8     16
-----------------------------------------------------
| SYN | SYN | SOH | Header | STX | Text | ETX | CRC |

 - uses special sentinel characters to indicate where a frame starts and ends.
 - there is a header section containing information used for "among other things, the link level reliable delivery algorithm"
 - there is a text section containing the message to be transmitted
 - also there are sentinels for start header, start text, end text.
 - after the end of frame sentinel there is a CRC section, Cyclic Redundancy Check
 - one complication is having to escape any occurrences of "frame end" sentinel (ETX?) in the text portion.

character stuffing is inserting extra characters (escapes in the case above) into the data portion of the frame.


Point-to-Point Protocol (PPP)

    8       8         8         16                16 or 32      8
-------------------------------------------------------------------
| Flag | Address | Control | Protocol | Payload | Checksum | Flag |


Flag = special start of frame character = 0111 1110 = 0x7e

Address and Control - "usually contain default values and so are uninteresting"

Protocol - used for demultiplexing. identifies the high-level protocol, like IP or IPX

default frame payload size = 1500 bytes. can be negotiated.

CHecksum size is 16 by default


PPP's frame format is weird in that its field sizes can be negotiated, rather than being fixed

LCP:

 - Link Control Protocol, works with PPP.
 - LCP sends control messages encapsulated in PPP frames.
    - messages are denoted with LCP identifier in PPP's Protocol fielD
 - this leads to PPP's frame format being changed based on information in the control messages.
 - involved in establishing a link between two peers when both sides detect that communication over the link is possible.

#### Byte-counting approach

Alternative to using sentinel characters is specifying how big your message is in the message. Receiver then counts the bytes to figure out when frame is done.

Approach is used by DECNET's DDCMP.

A danger is that transmission error can corrupt the count field. (Is true for sentinel approach too. Final ETX could be corrupted.)

If this happens during byte-counting approach, it will use up as many bytes as bad Count field says too, then by using CRC figures out the frame is bad. Called a **framing error**. It could botch back-to-back frames (or more? if erroneous count error is big enough?)

### Bit-oriented approaches (HDLC)
skip, TODO later

### Clock-based framing (SONET)
skip, TODO later



## Error detection

 - error correction goes along with error detection.
 - two approaches:
    - notify sender when message was corrupted so sender can retransmit
    - alternatively, use error-correcting codes to detect incorrect messages and reconstruct correct message

One of most common approaches is CRC. Is used in nearly all link-level protocols described above:  HDLC, DDCMP, also in CSMA (described later)

The basic idea of an error detection scheme: add redundant information to the frame.

 - one approach: transmit 2 (or N) copies of data, compare the two. If they differ, one or both are corrupted.
 - ^ is bad. n extra bits for a n-bit message.
 - other approaches can do much better. e.g. on Ethernet, frame carries up to 1500 bytes, and only needs 32-bit CRC to catch errors.
 - the extra bits that get added by the sender in the case of error detection are determined by some algorithm. Both sender and receiver know the algorithm ahead of time. Receiver can then use the procedure on the received message and compare with the result that was sent by the sender (e.g. a checksum)
 - the extra bits in general are referred to as **error-detecting codes**
 - in specific cases (usually when addition is involved), may be called a **checksum**
 - the Internet checksum is an error check that uses a summing algorithm.

### Two-dimensional parity

TODO, p. 93
