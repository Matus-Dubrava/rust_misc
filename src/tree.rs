#[derive(Debug)]
pub struct Node {
    value: i64,
    depth: usize,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(value: i64, depth: usize) -> Self {
        Self {
            value,
            depth,
            left: None,
            right: None,
        }
    }

    fn insert(&mut self, value: i64, depth: usize) -> Option<&Box<Node>> {
        if self.value > value {
            let left = &mut self.left;
            if let Some(left_node) = left {
                left_node.insert(value, depth + 1)
            } else {
                let new_node = Box::new(Node::new(value, depth));
                *left = Some(new_node);
                left.as_ref()
            }
        } else if self.value < value {
            let right = &mut self.right;
            if let Some(right_node) = right {
                right_node.insert(value, depth + 1)
            } else {
                let new_node = Box::new(Node::new(value, depth));
                *right = Some(new_node);
                right.as_ref()
            }
        } else {
            None
        }
    }

    fn traverse(&self) {
        if let Some(left) = &self.left {
            left.traverse();
        }
        println!("{}{}", "\t".repeat(self.depth), self.value);
        if let Some(right) = &self.right {
            right.traverse();
        }
    }
}

pub struct Tree {
    root: Option<Box<Node>>,
}

impl Tree {
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn insert(&mut self, value: i64) -> Option<&Box<Node>> {
        let root = &mut self.root;
        if let Some(root) = root {
            root.insert(value, 1)
        } else {
            let new_node = Box::new(Node::new(value, 0));
            *root = Some(new_node);
            root.as_ref()
        }
    }

    pub fn traverse(&self) {
        if let Some(root) = &self.root {
            root.traverse();
        }
    }
}

// example usage:
// fn main() {
//     let values = [10, 4, 5, 1, 20, 15, 25];
//     let mut tree = Tree::new();
//     for v in values {
//         if tree.insert(v).is_none() {
//             eprintln!("inserting duplicate value '{}'", v);
//         }
//     }
//     tree.traverse();
// }
