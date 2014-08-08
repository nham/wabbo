# Implementing search trees via AA trees in Rust

A **binary search tree** is a binary tree where for every node n, for every node j in the left subtree of n, j's key is less than n's key, and for every node k in the right subtree of n, k's key is greater than n's key. (It's also possible to amend this definition to make either the left- or right-subtree allow node keys that are less-than-or-equal or greater-than-or-equal than the node. This is useful when allowing duplicate keys).

A **red-black tree** is a binary search tree with the following properties:

 1. Each node in the tree is assigned a color, either red or black
 2. In addition to a color, any internal (non-leaf) node has a key, a value, and pointers to its children. Also, the set of all keys has some (total) ordering on it.
 3. Leaf nodes have neither key nor value, and must be black
 4. The root node is black
 5. Every red node has two children, and both are black
 6. For any node n and for any descendents d and e of n, the path n -> d and the path n -> e have the same number of black nodes

Note: we could simplify this and use only keys instead of both keys and values, but we are interested in implementing *maps* or *associative arrays* using red-black trees, so we leave them in.

The last property gives us something interesting: every node n has a well-defined **black-height**, which is the number of black nodes in any path from n to a leaf node. We will actually define and use a slightly different property: the **level** of a node n is the number of black nodes in any path from n to a leaf node *excluding node n itself*. So if n is a node, then when n is black, level(n) = black-height(n) - 1, whereas when n is red, level(n) = black-height(n).

An **AA tree** or **Andersson tree** is a red-black tree which obeys an additional property:

 - Every red node is a right child

Let's see how we can implement an AA tree in Rust. To model the nodes of an AA tree, we might use an `enum`, which allows one to define algebraic data types in Rust:

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

These type definitions match much of the discussion above:

 - a node is either a leaf or is internal
 - leaves have no data (no children, no key or values)
 - internal nodes have a color, a key, a value, and pointers to its child nodes.

This definition meshes with rules 1-3, but we must ensure that the other rules are obeyed as well. However, it turns out that that the above definition is not so easy to work with, so rather than explicitly modeling the color of each node, it is both sufficient and convenient to use the level property instead. Also, since leaf nodes have a level of zero by definition, we no longer need to distinguish between leaf and internal nodes. Here is our new definition:

    type Link<T> = Option<Box<T>>;

    struct Node<K, V> {
        key: K,
        value: V,
        left: Link<Node<K, V>>,
        right: Link<Node<K, V>>,
        level: uint
    }

A major change is that we no longer directy represent leaf nodes. Every node has both a key and a value, and if a pointer is missing in the actual struct, we really mean that there's a pointer to a leaf node that we are simply neglecting to actually store in memory. Also, instead of a `color` field we simply have a `level`. I glossed over it above, so you might be wondering how this data allows us to rebalance our tree according to the red-black rules. The key is that any red node must be a child of a black node by (5), so by the definition of "level" a red node has the same level as its parent. So we can detect red nodes simply by checking whether a node's level is the same as its parent's level.
