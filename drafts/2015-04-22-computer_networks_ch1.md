---
title: "Notes from *Computer Networks: A Systems Approach*, Ch. 1"
---

Much of the below is just quotes and me paraphrasing parts of the book.

 > Suppose you want to build a computer network, one that has the potential to grow to global proportions and to support applications as diverse as teleconferencing, video on demand, electronic commerce, distributed computing, and digital libraries. What available technologies would serve as the underlying building blocks, and what kind of software architecture would you design to integrate these building blocks into an effective communication service?

-- p. 1

The book aims to answer this question.

Some other, non-computer, networks:

 - voice telephone network
 - cable TV network.

How different? Computer networks more general (nodes are general-purpose programmable computers, network can carry any kind of digital data not just, for example, voice signals). The other networks are, however, more optimized for their narrow function.

 > Today's computer networks are increasingly taking over the functions previously performed by single-use networks.

-- p. 2


## Applications of computer networks

WWW is singled out as "the Internet application that catapulted the Internet from a somewhat obscure tool used mostly by scientists and engineers to the mainstream phenomenon that it is today."

They point out that many people unfortunately confuse the two or don't know the difference. The Web is pretty diverse though, so maybe it's not really an "application" itself, more like a platform that is suitable for building applications on.

 > Users view pages full of textual and graphical objects and click on objects that they want to learn more about, and a corresponding new page appears.

-- p.3

Maybe an explanation from [Wikipedia](http://en.wikipedia.org/wiki/Hypermedia) is appropriate here:

 > **Hypermedia**, an extension of the term hypertext, is a nonlinear medium of information which includes graphics, audio, video, plain text and hyperlinks. 

The Web (HTTP and related technologies like browsers, HTML/CSS/Javascript languages and libraries) provides an environment in which such hypermedia documents are possible to make.

HTTP objects each have a Uniform Resource Locator (URL). For example, Wikipedia's page on HTTP has a URL:

    http://en.wikipedia.org/wiki/Hypertext_Transfer_Protocol

For another example, here is the URL for the title image of google.com today, which is a Google Doodle for Earth Day 2015.

    https://www.google.com/logos/doodles/2015/earth-day-2015-5638584300208128.3-hp.gif

The URL indicates that the file is an animated gif (ends in .gif). Other objects can have .html endings (indicating an HTML document), or no ending at all, like Wikipedia's page on HTTP.

HTML's design is such that HTML document are HTTP objects that include other HTTP objects, like images or runnable code. Dozens of HTTP requests can be made when viewing a single web page (for example, one with lots of images).

For an HTTP client to talk to an HTTP server:

 - up to six messages to translate server name into IP address
 - three messages to setup TCP connection between browser and server
 - four messages to send HTTP GET request and for server to respond with the page
 - four messages to tear down TCP connection

This was a basic example. There are other possibilities. For example a [blog post from 5 days ago](http://blog.chromium.org/2015/04/a-quic-update-on-googles-experimental.html) noted that Chromium (Google's browser) has been routing some traffic bound for Google using the expermintal QUIC transport-layer protocol instead of TCP. So HTTP can work over other transport protocols.


Another application: streaming audio and video.

 - video on demand
 - internet radio

Major differences:

 - Most of the time, little sense in downloading whole video at once, as is done for other objects like images or pages. So the audo and video should be "streamed", with data that makes up audio/video being received and then immediately being played/displayed.
 - For streaming audio/video, skips in the content are usually a game over. A page can be read in bits and pieces though. Maybe an image takes a long time to fetch and retrieve. Often okay because you can start reading text while image comes in.

Another application: real-time audio and video. Tight timing constraints since you're trying to have a real-time voice conversation. Too much delay between 1) thing happening and 2) participants seeing it happen makes it not very effective. Different requirement than video on demand, where it is allowed to "buffer" and download a significant part of the video before displaying anything (useful if download speed is poor).

Videoconferencing: widespread now as a result of Skype, Microsoft Lync and similar products.

Key point of all of this: downloading Web page and doing videoconferencing are two quite different applications built on top of the internet. That the Internet supports both is a marvelous thing. The book, of course, will look at how the Internet manages to do this.


## Requirements

How might these three classes of people:

 - application programmer
 - network operator
 - network designer

each list their requirements for a network? Well:

 - application programmer might list services needed for their application. Examples include:

    - "guaranteed that each message the application sends will be delivered without error within a certain amount of time"
    - "ability to switch gracefully among different connections to the network as the user moves around"

 - network operator might list characteristics that make the network easy or difficult to manage and operate. Some examples are:

    - which faults can be isolated (and how easily)
    - which new devices can be added to the network

 - network designer might list "properties of a cost-effective design":

    - that network resources are efficiently utilized
    - ...and fairly allocated to different users
    - Performance issues are also important


### Scalable connectivity

 > [A] network must provide connectivity among a set of computers.

-- p. 8

Sometimes a limited network is useful, for example a corporate network. Other networks, like the Internet, are explicitly open and (attempt to be) global. They are "designed to grow in a way that allows them the potential to connect all the computers in the world."

They say that a system "designed to support growth to an arbitrarily large size" is said to *scale*.


#### Connectivity

Occurs at many different levels.

 - lowest level: two or more computers directly connected by some physical medium like coaxial cable or optical fiber. The physical medium connecting the computers is called a *link*, computers often called *nodes*.
