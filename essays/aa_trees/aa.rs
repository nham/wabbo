#![crate_name = "toy_aa"]
#![crate_type = "dylib"]

use std::fmt::{mod, Show};
use std::mem::{replace, swap};

type Link<T> = Option<Box<T>>;

struct Node<K, V> {
    key: K,
    value: V,
    left: Link<Node<K, V>>,
    right: Link<Node<K, V>>,
    level: uint
}

pub struct Tree<K, V> {
    root: Link<Node<K, V>>,
}

impl<K: Ord, V> Tree<K, V> {
    pub fn new() -> Tree<K, V> {
        Tree { root: None }
    }

    fn is_bst(&self) -> bool {
        match self.root {
            None => true,
            Some(ref r) => r.is_bst()
        }
    }

    fn is_aa(&self) -> bool {
        match self.root {
            None => true,
            Some(ref r) => r.is_aa()
        }
    }

    pub fn find<'a>(&'a self, key: &K) -> Option<&'a V> {
        match self.root {
            None => None,
            Some(ref r) => r.find(key)
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        match self.root {
            None => {
                self.root = Some(box Node::new(key, value));
                None
            },
            Some(ref mut r) => r.insert(key, value)
        }
    }
}


impl<K: Ord, V> Node<K, V> {
    pub fn new(key: K, value: V) -> Node<K, V> {
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

    pub fn is_bst(&self) -> bool {
        self.left.as_ref().map_or(true, |n| n.is_bst() && *n.max() < self.key)
        && self.right.as_ref().map_or(true, |n| n.is_bst() && *n.min() > self.key)
    }

    pub fn find<'a>(&'a self, key: &K) -> Option<&'a V> {
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


    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        let replaced =
        {
            let link = match key.cmp(&self.key) {
                Equal => {
                    self.key = key;
                    return Some(replace(&mut self.value, value))
                },
                Less => &mut self.left,
                Greater => &mut self.right,
            };

            match *link {
                None => {
                    *link = Some(box Node::new(key, value));
                    None
                },
                Some(ref mut b) => {
                    b.insert(key, value)
                },
            }
        };

        self.skew();
        self.split();
        replaced

    }


    // To be an AA tree, it must be a binary search tree and, for all nodes n:
    //   - if n is missing a child, its level must be 1
    //   - the left child must have a level one less than n's level
    //   - the right child must have a level equal to or one less than n's level
    //   - the right child's right child must not have the same level as n's level
    fn is_aa(&self) -> bool {
        let lvl = self.level;

        self.is_bst()
            && !((self.left.is_none() || self.right.is_none()) && self.level != 1)
            && self.left.as_ref().map_or(true, |n| n.is_aa())
            && self.no_red_left_child()
            && self.right.as_ref().map_or(true,
                |n| n.is_aa() && (n.level == lvl || n.level + 1 == lvl)
                    && !(n.level == lvl && !n.no_red_right_child()))
    }

    fn no_red_left_child(&self) -> bool {
        match self.left {
            None => true,
            Some(ref n) => n.level + 1 == self.level,
        }
    }

    fn no_red_right_child(&self) -> bool {
        match self.right {
            None => true,
            Some(ref n) => n.level + 1 == self.level,
        }
    }


    // Remove red left child by rotating right
    /*
         a      b
        /        \
       b    =>    a
        \        /
         c      c

      provided that a.level == b.level
    */
    fn skew(&mut self) {
        if self.left.is_some() && self.left.get_ref().level == self.level {
            let mut save = self.left.take_unwrap();
            swap(&mut self.left, &mut save.right); // save.right now None
            swap(self, &mut *save);
            self.right = Some(save);
        }
    }

    // Remove red right child of red right child by rotating left and increasing
    // level of the parent
    /*
        a            b
         \          / \
          b    =>  a   c
         / \        \
        d   c        d

      provided that a.level == c.level
    */
    fn split(&mut self) {
        if self.right.as_ref().map_or(false,
          |x| x.right.is_some() && x.right.get_ref().level == self.level) {
            let mut save = self.right.take_unwrap();
            swap(&mut self.right, &mut save.left); // save.left now None
            save.level += 1;
            swap(self, &mut *save);
            self.left = Some(save);
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


impl<K: Show, V: Show> Show for Node<K, V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.key, self.value)
    }
}

fn print_node_depth<K: Show, V: Show>(node: &Link<Node<K,V>>, depth: uint) {
    let mut pre = "".to_string();
    if depth > 0 {
        for i in range(0, depth) {
            pre = pre + "   ";
        }
    }

    match *node {
        Some(ref n) => {
            println!("{}{}^{}", pre, n, n.level);
            print_node_depth(&n.left, depth + 1);
            print_node_depth(&n.right, depth + 1);
        },
        None => println!("{}-", pre),
    }
}

pub fn print_tree<K: Show + Ord, V: Show>(tree: &Tree<K, V>) {
    print_node_depth(&tree.root, 0);
    println!("Is AA: {}", tree.is_aa());
    println!("------------");
}


mod test {
    use super::Tree;
    use std::rand;
    use std::rand::distributions::{IndependentSample, Range};

    #[test]
    fn test_find() {
        let mut t = Tree::new();
        assert_eq!(t.find(&1u), None);
        t.insert(1u, 'j');
        assert_eq!(t.find(&1u), Some(&'j'));

    }

    // testing whether we can find all the things we inserted
    #[test]
    fn test_insert() {
        let mut t: Tree<uint, u8> = Tree::new();
        for (i, c) in range(0u, 10).zip(range(b'a', b'z')) {
            t.insert(i, c);
        }

        for (ref i, ref c) in range(0u, 10).zip(range(b'a', b'z')) {
            assert_eq!(t.find(i), Some(c));
        }

        assert_eq!(t.find(&10u), None);
    }

    fn insert_n_check_aa(n: uint, between: Range<uint>, rng: &mut rand::TaskRng) {
        let mut t = Tree::new();

        for _ in range(0u, n) {
            let a = between.ind_sample(rng);
            println!("{}", a);
            t.insert(a, ());
        }

        assert!(t.is_aa());
    }

    // testing whether, after inserting 20 random keys, is_aa() returns true
    #[test]
    fn test_insert_is_aa() {
        let mut rng = rand::task_rng();
        let between = Range::new(0u, 100_000);

        for _ in range(0u, 300) {
            insert_n_check_aa(20, between, &mut rng);
        }
    }

    // testing whether, after inserting 20 random keys, is_aa() returns true,
    // but this time some of the keys are repeated
    #[test]
    fn test_insert_dups_is_aa() {
        let mut rng = rand::task_rng();
        let between = Range::new(0u, 15);

        for _ in range(0u, 300) {
            insert_n_check_aa(20, between, &mut rng);
        }
    }
}
