# Implementing AA trees and red-black trees in Rust

A **red-black tree** is a binary tree with the following properties:

 - Each node in the tree is assigned a color, either red or black
 - In addition to a color, any internal (non-leaf) node has a key, a value, and pointers to its children. Also, the set of all keys has some (total) ordering on it.
 - Leaf nodes have neither key nor value, and must be black
 - The root node is black
 - Every red node has two children, and both are black
 - For any node n and for any descendents d and e of n, the path n -> d and the path n -> e have the same number of black nodes 

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

These type definitions mirror much of the discussion above: a node is either a leaf or is internal, leaves have no data, internal nodes have a color, a key, a value, and pointers to its child nodes.

To ensure that the other properties hold, the idea is that we will apply transformations to the tree as necessary whenever we insert or remove nodes.
