# x86 bootloaders are messy

#### 16 June 2014

Hacker School started last week, and for my first major project I decided I was going to write an operating system in Rust. More specifically, since I don't understand how operating systems work, I decided to try and port [xv6](http://pdos.csail.mit.edu/6.828/2012/xv6.html), an educational OS made by some people at MIT.

Since I am a rigorously-minded person interested in messy details, I decided to try to understand the booting process and write my own bootloader. It couldn't be too hard, right? It just involves checking some of x86 documentation and writing some assembly, nothing truly difficult. Whatever difficulty there would be, I was sure my investment will be repaid many times over by the Deep Insight into the design and operation of the x86 architecture that my investigations would yield. I was reasonably convinced that this would be a productive use of my time.

Dear reader, I am now skeptical of that assessment. This has been a frustrating exercise and I'm unsure what, precisely, I have gained. Below I outline everything that I've learned, so that you can see for yourself how futile it is to try to understand the boot sequence.


## Power on + BIOS magic
When you power on your PC, the [BIOS](http://en.wikipedia.org/wiki/BIOS) begins running. This is a program stored in firmware that tests your hardware and possibly does some kind of initialization. To be honest, I don't actually understand what the BIOS does. As far as I know, the BIOS is a wizard trapped in your computer that casts a few spells on your system to make sure it's ready for use.

(Also, if your computer is recent enough, it may not use BIOS. [UEFI](http://en.wikipedia.org/wiki/Unified_Extensible_Firmware_Interface) is a recent standard intended as a replacement for BIOS. I have made no attempt to understand how UEFI works.)

After spellcasting is complete, the BIOS tries to find a bootable device. It does this by checking various media (hard drives, CD drives, floppy drives, USB drives, with order determined by BIOS settings) for the presence of a *boot sector*, which is a 512-byte chunk at the beginning of said media that contains code. Specifically, the BIOS checks for the presence of the magic number 0xaa55 at the last 2 bytes of this 512-byte chunk. Actually, since x86 is [little endian](http://en.wikipedia.org/wiki/Endianness), 510th and 511th bytes (zero-indexed) will show up as 0x55 and 0xaa, respectively. If this signature is found, the chunk is assumed to contain executable code, so the BIOS loads it into memory starting at address 0x7c00 and begins executing it.


## The bootloader




Sources:

 - xv6 [code](http://pdos.csail.mit.edu/6.828/2012/xv6/xv6-rev7.pdf) and [commentary](http://pdos.csail.mit.edu/6.828/2012/xv6/book-rev7.pdf)
 - [rv6](https://github.com/mahrz/rv6), a kernel in Rust based on xv6
 - [Intel 64 and IA-32 Architectures Software Developer's Manual Combined Volumes 3A, 3B, and 3C: System Programming Guide](http://www.intel.com/content/dam/www/public/us/en/documents/manuals/64-ia-32-architectures-software-developer-system-programming-manual-325384.pdf)
 - [Some notes on xv6](http://moss.cs.iit.edu/cs450/xv6notes.html)
 - ["Writing a Simple Operating System from Scratch"](http://www.cs.bham.ac.uk/~exr/lectures/opsys/10_11/lectures/os-dev.pdf)
