struct Tree {
    root: Option<Box<Node>>,
}

impl Tree {
    fn new() -> Self {
        Self{root: None} 
    }

    fn in_order_traversal(&self) {
        self.root.as_ref().map(|node_ptr| {
            node_ptr.left.in_order_traversal();
            println!("{:?}", node_ptr.value);
            node_ptr.right.in_order_traversal();
        });

    }

    fn pre_order_traversal(&self) {
        self.root.as_ref().map(|node_ptr| {
            println!("{:?}", node_ptr.value);
            node_ptr.left.pre_order_traversal();
            node_ptr.right.pre_order_traversal();
        });
    }

    fn post_order_traversal(&self) {
        self.root.as_ref().map(|node_ptr| {
            node_ptr.left.post_order_traversal();
            node_ptr.right.post_order_traversal();
            println!("{:?}", node_ptr.value);
        });
    }

    fn reverse_order_traversal(&self) {
        self.root.as_ref().map(|node_ptr| {
            node_ptr.right.reverse_order_traversal();
            println!("{:?}", node_ptr.value);
            node_ptr.left.reverse_order_traversal();
        });
    }

    fn add(&mut self, v: i32) {
        match self.root.as_mut() {
            None => self.root = Some(Box::new(Node::new(v))),
            Some(n) => if v < n.value { &mut n.left } else { &mut n.right }.add(v),
        }
    }
}

struct Node {
    value: i32,
    left: Tree,
    right: Tree,
}

impl Node {
    fn new(value: i32) -> Self {
        Self{value, left: Tree{root:None}, right: Tree{root:None}}
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

}
