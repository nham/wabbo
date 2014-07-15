# x86 bootloaders are messy

#### 16 June 2014

Hacker School started last week, and for my first major project I decided I was going to write an operating system in Rust. More specifically, since I don't understand how operating systems work, I decided to try and port [xv6](http://pdos.csail.mit.edu/6.828/2012/xv6.html), an educational OS made by some people at MIT.

Since I am a rigorously-minded person interested in messy details, I decided to try to understand the booting process and write my own bootloader. It couldn't be too hard, right? It just involves checking some of x86 documentation and writing some assembly, nothing truly difficult. Whatever difficulty there would be, I was sure my investment will be repaid many times over by the Deep Insight into the design and operation of x86 CPUs that my investigations would yield. I was reasonably convinced that this would be a productive use of my time.

Dear reader, I am now skeptical of that assessment. This has been a frustrating exercise and I'm unsure what, precisely, I have gained. Below I outline everything that I've learned, so that you can see for yourself how futile it is to try to understand the intricacies of the boot sequence.


## Power on + BIOS magic
When you power on your PC, the [BIOS](http://en.wikipedia.org/wiki/BIOS) begins running. This is a program stored in firmware that tests your hardware and possibly does some kind of initialization. To be honest, I don't actually understand what the BIOS does. As far as I know, the BIOS is a wizard trapped in your computer that casts a few spells on your system to make sure it's ready for use.

(Also, if your computer is recent enough, it may not use BIOS. [UEFI](http://en.wikipedia.org/wiki/Unified_Extensible_Firmware_Interface) is a recent standard intended as a replacement for BIOS. I have made no attempt to understand how UEFI works, and QEMU, the CPU emulator I'm using, doesn't seem to support it out of the box, so to heck with it.)

After spellcasting is complete, the BIOS tries to find a bootable device. It does this by checking various media (hard drives, CD drives, floppy drives, USB drives, with order determined by BIOS settings) for the presence of a *boot sector*, which is a 512-byte chunk at the beginning of said media that contains code. Specifically, the BIOS checks for the presence of the magic number 0xaa55 at the last 2 bytes of this 512-byte chunk. Actually, since x86 is [little endian](http://en.wikipedia.org/wiki/Endianness), 510th and 511th bytes (zero-indexed) will show up as 0x55 and 0xaa, respectively. If this signature is found, the chunk is assumed to contain executable code, so the BIOS loads it into memory starting at address 0x7c00 and begins executing it.


## The bootloader
From what I am given to understand, 510 bytes does not usually suffice to load a kernel. Hence we need a *multi-stage* bootloader. With this approach, we will have a stage 1 bootloader that does a little bit of setup and then transfers control to the stage 2 bootloader, which finishes the job.

### Stage 1 bootloader
When an x86 CPU starts up, it is initially operating in **real mode**, which is a 16-bit mode. Essentially, x86 CPUs start out simulating an [Intel 8086](http://en.wikipedia.org/wiki/Intel_8086). One task that the bootloader for a 32-bit kernel must do is to switch the CPU into its 32-bit mode, which is called **protected mode**.

The reason that 32-bit mode is called **protected mode** is that all memory accesses use the x86's *segmentation hardware* as a mechanism for virtual memory. Before switching into protected mode, we also have to set up this hardware, which is called the **global descriptor table**. This is a bit of a pain because xv6, and from what I understand Linux and most other modern operating systems, do not really make use of segmentation, but instead use an approach to virtual memory called **paging**. Regardless, all memory accesses in protected mode are mediated by the segmentation hardware, so we really don't have any choice in the matter.

Another fun detail to consider is that interrupt handling is done differently in real mode and protected mode, which means that before we switch into protected mode we need to disable interrupt handling (lest we get an interrupt that was set up (by the BIOS? I don't really understand here) in 16-bit after we've switched into 32-bit mode). I'm waving my hands here because I haven't taken the time to understand this point closely. The remedy is simple though: just execute the instruction "cli" before we start the switch.

I won't go into details about how to set up the GDT because I don't want to write a small book about the x86 boot sequence, but in essence we will set up 3 entries in table: the first is a null descriptor, which is required for some reason, and the second is a code segment and the third is a data segment. The latter two segments are set up in accordance with "flat addressing", so a base address of 0x0 and a limit of 0xfffff$,



Sources:

 - xv6 [code](http://pdos.csail.mit.edu/6.828/2012/xv6/xv6-rev7.pdf) and [commentary](http://pdos.csail.mit.edu/6.828/2012/xv6/book-rev7.pdf)
 - [rv6](https://github.com/mahrz/rv6), a kernel in Rust based on xv6
 - [Intel 64 and IA-32 Architectures Software Developer's Manual Combined Volumes 3A, 3B, and 3C: System Programming Guide](http://www.intel.com/content/dam/www/public/us/en/documents/manuals/64-ia-32-architectures-software-developer-system-programming-manual-325384.pdf)
 - [Some notes on xv6](http://moss.cs.iit.edu/cs450/xv6notes.html)
 - ["Writing a Simple Operating System from Scratch"](http://www.cs.bham.ac.uk/~exr/lectures/opsys/10_11/lectures/os-dev.pdf)
