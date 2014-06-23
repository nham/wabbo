# 23 Jun 2014
## Declaring Defeat {#declaring_defeat}
I just wanted make a brief note here that I am declaring defeat with regards to the main project I wanted to work on at Hacker School, which was a operating system written in Rust. Initially I had the preposterous idea that I was going to write my own bootloader and have it jump into a kernel written in Rust. I wrote a [bootloader](https://github.com/nham/porifera), but was unable to get the "jumping into Rust" part working due to lack of knowledge of the Rust compiler and a lack of documentation for said compiler. I decided that I would have better luck attempting to use [rustboot](https://github.com/pczarn/rustboot), a Rust bootloader that someone else had written.

Unfortunately, I am so far unable to get even that working. It does not compile on my machine. Up to now, I have spent almost all of my time reading reading reading (man pages, specs, tutorials, books, Stack Overflow comments, blog posts) and hardly writing any code. It seems most wise to declare defeat and just write an operating system in C.

If I might permit myself to be optimistic for a moment: this is not so great a defeat because I actually had two goals for Hacker School: write an operating system and program more in Rust. It is only in trying to achieve both of these goals in the same project that I have failed.
