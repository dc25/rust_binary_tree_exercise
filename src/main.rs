// originally: https://www.reddit.com/r/rust/comments/155l0n2/roast_my_binary_tree_please/
//
use std::fmt::Debug;

type NodePtr<T> = Option<Box<Node<T>>>;

struct Tree<T> {
    root: NodePtr<T>,
}

impl<T: Debug + PartialOrd> Tree<T> {
    fn new() -> Self {
        Self { root: None }
    }

    fn in_order_traversal(&self) {
        if let Some(node_ptr) = self.root.as_ref() {
            node_ptr.left.in_order_traversal();
            println!("{:?}", node_ptr.value);
            node_ptr.right.in_order_traversal();
        };
    }

    fn pre_order_traversal(&self) {
        if let Some(node_ptr) = self.root.as_ref() {
            println!("{:?}", node_ptr.value);
            node_ptr.left.pre_order_traversal();
            node_ptr.right.pre_order_traversal();
        };
    }

    fn post_order_traversal(&self) {
        if let Some(node_ptr) = self.root.as_ref() {
            node_ptr.left.post_order_traversal();
            node_ptr.right.post_order_traversal();
            println!("{:?}", node_ptr.value);
        };
    }

    fn reverse_order_traversal(&self) {
        if let Some(node_ptr) = self.root.as_ref() {
            node_ptr.right.reverse_order_traversal();
            println!("{:?}", node_ptr.value);
            node_ptr.left.reverse_order_traversal();
        };
    }

    fn add(&mut self, v: T) {
        match self.root.as_mut() {
            None => self.root = Node::new(v),
            Some(n) => if v < n.value {
                &mut n.left
            } else {
                &mut n.right
            }
            .add(v),
        }
    }
}

struct Node<T> {
    value: T,
    left: Tree<T>,
    right: Tree<T>,
}

impl<T:Debug + PartialOrd> Node<T> {
    fn new(value: T) -> NodePtr<T> {
        Some(Box::new(Self {
            value,
            left: Tree::new(),
            right: Tree::new(),
        }))
    }
}

fn main() {
    let mut tree = Tree::new();
    tree.add(10);
    tree.add(7);
    tree.add(6);
    tree.add(8);
    tree.add(11);

    tree.in_order_traversal();
    println!("\n");

    tree.pre_order_traversal();
    println!("\n");

    tree.post_order_traversal();
    println!("\n");

    tree.reverse_order_traversal();
    println!("\n");

    let mut strtree = Tree::new();
    strtree.add("10".to_string());
    strtree.add("7".to_string());
    strtree.add("6".to_string());
    strtree.add("8".to_string());
    strtree.add("11".to_string());

    strtree.in_order_traversal();
    println!("\n");

    strtree.pre_order_traversal();
    println!("\n");

    strtree.post_order_traversal();
    println!("\n");

    strtree.reverse_order_traversal();
    println!("\n");
}
