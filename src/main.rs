pub mod tree;

use crate::tree::Tree;

fn main() {
    let values = [10, 4, 5, 1, 20, 15, 25];
    let mut tree = Tree::new();
    for v in values {
        if tree.insert(v).is_none() {
            eprintln!("inserting duplicate value '{}'", v);
        }
    }
    tree.traverse();
}
