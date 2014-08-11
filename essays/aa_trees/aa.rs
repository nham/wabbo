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

fn main() {
}
