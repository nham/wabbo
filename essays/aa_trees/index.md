---
title: Introduction to AA trees, with implementation in Rust
---
<img src="header_aa.svg" class="center"></img>

## Binary search trees

You should know what a [tree](https://en.wikipedia.org/wiki/Tree_(data_structure)) is. (We have to start somewhere). Each node in the trees that we will be considering will have some data attached to it. We are only interested in the cases where all the node data is of the same type.

A **binary search tree** is a binary tree whose node data, which we will call the node's *key*, has some [ordering](http://en.wikipedia.org/wiki/Total_order) to it, and where for every node $n$:

 - every node in the left subtree of $n$ has a key less than $n$'s key
 - every node in the right subtree of $n$ has a key greater than $n$'s key.

Binary search trees are useful because they can enable efficient lookups: if the key we're searching for is strictly less than the current node, we know that we only have to search the left subtree of the current node, since everything in the right subtree is bigger than the current node (and hence bigger than the key). So binary search trees can enable us to eliminate large portions of the search space with a single comparison.

Note that I said "can enable", not "enable". It is possible to create degenerate binary search trees which have slow lookups. For example, if we insert keys $1, 2, 3, 4, 5$ in that order, what we get is a binary search tree that looks like this:

<img src="bst.svg" class="center"></img>

This is effectively a linked list, and when we search for a key we have to search all nodes in the tree. Such a tree is called **unbalanced**.

TODO: Talk about unbalanced BSTs, define a balanced binary search tree.

## AA trees

A **red-black tree** is a kind of binary search tree. Each node in the tree has a color, either red or black, and there are two kinds of nodes:

 - leaf node: always black, contains no data
 - internal node: can be red or black, has both a key and a value (the set of all keys must have an ordering on it, just like with binary search trees)

In addition, the following rules must be adhered to:

 1. The root node is black
 2. Every red node has two children, and both are black
 3. For any node $n$ and for any descendents $d$ and $e$ of $n$, the path $n \to d$ and the path $n \to e$ have the same number of black nodes

The last rule gives us something interesting: it ensures that every node $n$ has a well-defined **black-height**, which is the number of black nodes in any path from $n$ to a leaf node. We will actually define and use a slightly different notion: the **level** of a node $n$ is the number of black nodes in any path from $n$ to a leaf node *excluding node $n$ itself*. So if $n$ is a node, then when $n$ is black, $level(n) = black-height(n) - 1$, whereas when $n$ is red, $level(n) = black-height(n)$.

Red-black trees are one instance of a class of data structures called [self-balancing binary search trees](http://en.wikipedia.org/wiki/Self-balancing_binary_search_tree). What this means is that when we insert or remove a node, the tree will automatically re-organize itself so that it's more balanced. By doing a little bit of re-balancing work every time we modify the tree, we can ensure that searches will be reasonably fast.

We could now define a data structure for red-black trees and implement the required  methods that maintain the red-black tree properties, but it turns out that it is a little bit tricky to do this! We will instead study and implement a simplified version of the red-black tree called an AA tree.

An **AA tree** or **Andersson tree** is a red-black tree which obeys an additional property:

 - Every red node is a right child

AA trees are named after Arne Andersson, who introduced them in a [paper](http://user.it.uu.se/~arnea/ps/simp.pdf) in 1993. This data structure permits a simpler implementation of general red-black trees because there are less cases to consider due to the fact that red left children are disallowed.

Let's see how we can implement an AA tree in [Rust](http://www.rust-lang.org). To model the nodes of an AA tree, we might use an `enum`, which allows one to define algebraic data types in Rust:

```rust
enum Node<K, V> {
    Leaf,
    Internal(InternalNode<K, V>)
}

struct InternalNode<K, V> {
    color: Color,
    key: K,
    val: V,
    left: Box<Node<K, V>>,
    right: Box<Node<K, V>>,
}

enum Color {
    Red,
    Black
}
```

(For the unfamiliar: `Box` is Rust's type for "owned pointers". You might be interested in learning more [here](http://doc.rust-lang.org/guide-pointers.html#boxes))

These type definitions match much of the discussion above:

 - a node is either a leaf or is internal
 - leaves have no data (no children, no key or values, no color (well, they have a color, but it's always black, so we don't need to specify it))
 - internal nodes have a color, a key, a value, and pointers to its child nodes.

This definition seems to be correct! Unfortunately this form is somewhat difficult to work with. We can take a different approach by noting that, given the level of each node in the tree, we can completely recover the color information: a node is red if and only if it has the same level as its parent. Also, we don't actually need to create leaf nodes in our code: if we just allow internal nodes to have optional pointers to children, then a missing pointer means there should be a leaf node below. If we define a new kind of binary tree where nodes consist of the following data:

 - key
 - value
 - level (nonnegative integer)
 - optional pointers to left and right children

Then we can ensure that such a tree corresponds to an AA tree by making these rules hold:

 1. nodes that are missing at least one child have level 1
 2. for any node $n$ with a left child $k$, $level(n) = level(k) + 1$
 3. for any node $n$ with a right child $k$, $level(n) - level(k) = 0$ or $1$
 4. for any node $n$, if $n$ has a parent $p$ and $level(n) = level(p)$, then for any child $k$ of $n$, $level(n) = level(k) + 1$

Rule 1, in particular, says that leaf nodes have level 1. Rule 2 says that no red node is a left child, and rule 4 says that no child of a red node is red. So any node that obeys these 4 rules clearly corresponds to an AA tree. Similarly, we can transform any AA tree to this form by throwing away the colors and leaf nodes.

Now that we have a more convenient representation of AA trees, let's define a type for AA trees in Rust:


```rust
type Link<T> = Option<Box<T>>;

struct Node<K, V> {
    key: K,
    value: V,
    left: Link<Node<K, V>>,
    right: Link<Node<K, V>>,
    level: uint
}

struct Tree<K, V> {
    root: Link<Node<K, V>>,
}
```

The `Tree` root is optional to allow for the possibility of empty trees.

### `find` method

Now we need to write some code so that we can *do things*. One thing we'd like to do is retrieve the value corresponding to a key if it exists in the tree. The retrieval method, which we'll call `find`, is the same for AA trees as it is for any binary search tree. In  pseudocode:

```
def find(key):
    if key == node.key:
        return node.value
    else if key < node.key:
        if node.left exists:
            return node.left.find(key)
        else:
            can't find key
    else:
        if node.right exists:
            return node.right.find(key)
        else:
            can't find key
```

Here's how it looks in Rust:


```rust
impl<K: Ord, V> Node<K, V> {
    fn find(&self, key: &K) -> Option<&V> {
        match key.cmp(&self.key) {
            Equal => Some(&self.value),
            Less =>
                match self.left {
                    None => None,
                    Some(ref b) => b.find(key),
                },
            Greater =>
                match self.right {
                    None => None,
                    Some(ref b) => b.find(key),
                },
        }
    }
}
```
Let's dissect this line by line:

```rust
impl<K: Ord, V> Node<K, V> {
```

This starts an `impl` block for `Node<K, V>`. Actually, it's a bit more specific than that. It defines two type parameters, `K` and `V`, and then stipulates a *trait bound* of `Ord` on the `K` parameter. [Ord](http://static.rust-lang.org/doc/master/std/cmp/trait.Ord.html) is a trait for types that have a total ordering on them, which is precisely the property we required of our keys in the definition of binary search trees.


```rust
    fn find(&self, key: &K) -> Option<&V> {
```

This defines a method named `find` that acts on instances of `Node<K, V>` (where `K` implements `Ord`). The method takes a single parameter, `key`, of type `&K`, which is a reference to a `K`. The other "parameter" you see there is the `&self` parameter, which specifies that the `find` method takes a borrow of the `Node<K, V>` instance that it's called on.

```rust
        match key.cmp(&self.key) {
```

Rust's `match` statement is basically a souped up `while` statement that can perform pattern matching. Here we're matching on the value `key.cmp(&self.key)`. The `cmp` method is the [sole method](http://static.rust-lang.org/doc/master/std/cmp/trait.Ord.html#tymethod.cmp) of the `Ord` trait, meaning it is available to all types that implement `Ord`. (Actually, that's backwards: in order to make a type implement `Ord` you have to implement the `cmp` method for that type.) `cmp` returns an [Ordering](http://static.rust-lang.org/doc/master/std/cmp/type.Ordering.html), whose definition is quite simply:

```rust
pub enum Ordering {
    Less,
    Equal,
    Greater,
}
```

So the `cmp` call is determining whether the key passed into the `find` method is less than, greater than or equal to the current node's key.


```rust
            Equal => Some(&self.value),
```

If the `cmp` call resulted in `Equal`, we can rejoice since we've found the key we're looking for! So we just return a reference to current node's value. Also, we're wrapping this value up in `Some`, which is one of the data constructors of the `Option` type.


```rust
            Less =>
                match self.left {
                    None => None,
                    Some(ref b) => b.find(key),
                },
```

This isn't a single line, but it makes sense to consider it all at once. If `key` is less than the node's key, we need to search the left subtree, if it exists. If the left subtree does not in fact exist, we return `None` because the key cannot be found. Otherwise we make a recursive call to `find` on the left child. the `ref b` inside the `Some` pattern match is needed because we only want to capture a reference to the left subtree.

The remainder is exactly the same, but with `Greater` and `self.right` instead.

### `insert` method

A binary search tree's `insert` method is quite similar to `find`, but it takes both a key and a value and doesn't return anything:

```
def bst_insert(key, value):
    if key == node.key:
        replace node.value with value
    else if key < node.key:
        if node.left exists:
            node.left.insert(key, value)
        else:
            node.left = new node with key and value
    else:
        if node.right exists:
            node.right.insert(key, value)
        else:
            node.right = new node with key and value
```

We essentially walk down the tree until we either find the key (in which case we overwrite the old `node.value` with the new value), or until we find a missing child, which indicates that the key is not in the tree and that we must create a new node.

We need to modify this code so that every insert into an AA tree results in something that is still an AA tree.
