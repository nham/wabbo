To even begin thinking about networking, you need some medium that electromagnetic waves can propagate in. Examples of such mediums include space (in the case of radio waves) or a length of wire or optical fiber. Such a physical medium is called a **link**.

Once you have solved the problem of making links, there are still five more problems to solve.

 1. You have a medium for sending electromagnetic waves on, but you really want to transmit bits. How can you encode the bits into an electromagnetic signal?
 2. Customarily a *sequence* of bits is what you want to send. So when you send one sequence of bits followed by another, how is the receiver to delineate when one sequence stops and another starts? Basically, the problem is to break up the bits send into **frames** or **packets**.
 3. The physical medium is unreliable. Frames can get corrupted. Can we detect when this corruption occurs?
 4. Can we engineer an arrangement that makes the link seem reliable even if it's not? This might involve correcting errors, or possibly other things.
 5. Finally, for multiple-access links, something needs to mediate access to it (to ensure that some collection of nodes don't monopolize it, for example?).
