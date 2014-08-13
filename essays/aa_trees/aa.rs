use std::mem::{replace, swap};

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


impl<K: Ord, V> Node<K, V> {
    fn new(key: K, value: V) -> Node<K, V> {
        Node { key: key, value: value, left: None, right: None, level: 1 }
    }

    fn max(&self) -> &K {
        match self.right {
            None => &self.key,
            Some(ref n) => n.max(),
        }
    }

    fn min(&self) -> &K {
        match self.left {
            None => &self.key,
            Some(ref n) => n.min(),
        }
    }

    fn is_bst(&self) -> bool {
        self.left.as_ref().map_or(true, |n| n.is_bst() && *n.max() < self.key)
        && self.right.as_ref().map_or(true, |n| n.is_bst() && *n.min() > self.key)
    }

    fn find<'a>(&'a self, key: &K) -> Option<&'a V> {
        let ch = match key.cmp(&self.key) {
            Equal => return Some(&self.value),
            Less => & self.left,
            Greater => & self.right,
        };

        match *ch {
            None => None,
            Some(ref b) => b.find(key),
        }
    }


    fn insert(&mut self, key: K, value: V) -> Option<V> {
        let ch = match key.cmp(&self.key) {
            Equal => {
                self.key = key;
                return Some(replace(&mut self.value, value))
            },
            Less => &mut self.left,
            Greater => &mut self.right,
        };

        match *ch {
            None => {
                *ch = Some(box Node::new(key, value));
                None
            },
            Some(ref mut b) => {
                let inserted = b.insert(key, value);
                skew(b);
                split(b);
                inserted
            },
        }

    }

    /*

       In the below, 'o' represents black, '*' represents red.

       First, note that deleting a red leaf requires no rebalancing at all.

       ---

       There's only 1 possible AA tree of size 2:

       o
        \
         *

       Deleting the root or the leaf requires no rebalancing.
       
       ---

       For size 3 trees, we only have:

         o
        / \
       o   o

        - deleting left leaf means we have to decrease the level of root. this suffices to rebalance
        - deleting right leaf means we have to decrease level of root but then make left leaf the 
          new root (and former root the child of new root)
        - deleting the root means right child is new root, left child becomes new root's right child

       ---

       For size 4 trees, we have either

         o       o
        / \     / \
       o   o   o   o
        \           \
         *           *

       There are 4 classes of deletion:

        - delete red leaf (see above)
        - delete red node's parent: make red a child of its former grandparent
        - delete root: in the first case, red needs to become root, so increase its level and make it 
          the new root. in the second case, right child of root needs to become new root.
        - delete other child: in the first case, red needs to become root and the old root needs to 
          be right child of new root. in the second case, decrease level of old root, increase level
          of right child and make old root the child of the right child (which is the new root)
       

       ---

       For size 5 trees, we have one of

         o        o
        / \      / \
       o   o    o   *
        \   \      / \
         *   *    o   o

       ===

       One fundamental operation we need is "promote the max from the left subtree (or min from right subtree!)
       this is used when we delete a root, for example. this seems essentially whenever we delete a node that
       has two children.
 
     */
}


// Remove left horizontal link by rotating right
/*
     a      b
    /        \
   b    =>    a
    \        /
     c      c

  provided that a.level == b.level
*/
fn skew<K: Ord, V>(node: &mut Box<Node<K, V>>) {
    if node.left.is_some() && node.left.get_ref().level == node.level {
        let mut save = node.left.take_unwrap();
        swap(&mut node.left, &mut save.right); // save.right now None
        swap(node, &mut save);
        node.right = Some(save);
    }
}


// Remove dual horizontal link by rotating left and increasing level of
// the parent
/*
    a            b
     \          / \
      b    =>  a   c
     / \        \
    d   c        d

  provided that a.level == c.level
*/
fn split<K: Ord, V>(node: &mut Box<Node<K, V>>) {
    if node.right.as_ref().map_or(false,
      |x| x.right.is_some() && x.right.get_ref().level == node.level) {
        let mut save = node.right.take_unwrap();
        swap(&mut node.right, &mut save.left); // save.left now None
        save.level += 1;
        swap(node, &mut save);
        node.left = Some(save);
    }
}

fn main() {
}
