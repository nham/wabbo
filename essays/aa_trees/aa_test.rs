extern crate toy_aa;

use toy_aa::{Tree, print_tree};

fn main() {

    let mut t = Tree::new();
    print_tree(&t);

    t.insert('e', 5u);
    print_tree(&t);

    t.insert('b', 88u);
    print_tree(&t);

    t.insert('d', 11u);
    print_tree(&t);

    let mut t = Tree::new();
    t.insert(7u, ());
    t.insert(8u, ());
    t.insert(9u, ());
    t.insert(6u, ());

    print_tree(&t);

}
