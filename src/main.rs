use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
struct Node {
    value: i32,
    left: Box<Option<Node>>,
    right: Box<Option<Node>>,
}

impl Node {
    fn add(&mut self, value: i32) {
        let mut height = 1;
        insert(self, value, height);
    }

    fn in_order_traversal(&self) {
        in_order(self);
    }

    fn pre_order_traversal(&self) {
        pre_order(self);
    }

    fn post_order_traversal(&self) {
        post_order(self);
    }

    fn reverse_order_traversal(&self) {
        reverse_order(self);
    }
}

fn in_order(node: &Node) {
    left_tree(node, in_order);
    println!("{:?}", node.value);
    right_tree(node, in_order);
}

fn reverse_order(node: &Node) {
    right_tree(node, reverse_order);
    println!("{:?}", node.value);
    left_tree(node, reverse_order);
}

fn pre_order<'a>(node: &'a Node) {
    println!("{:?}", node.value);
    left_tree(node, pre_order);
    right_tree(node, pre_order);
}

fn post_order<'a>(node: &'a Node) {
    left_tree(node, post_order);
    right_tree(node, post_order);
    println!("{:?}", node.value);
}

fn left_tree(node: &Node, f: fn(node: &Node)) {
    let left_node = node.left.as_ref();
    match left_node {
        Some(l) => f(l),
        None => {}
    }
}

fn right_tree(node: &Node, f: fn(node: &Node)) {
    let right_node = node.right.as_ref();
    match right_node {
        Some(r) => f(r),
        None => {}
    }
}

fn insert(node: &mut Node, value: i32, mut height: i32) -> &mut Node {
    height += 1;
    println!("{}", height);
    if value >= node.value {
        if node.right.is_none() {
            let right = node.right.insert(Node {
                value: value,
                left: Box::new(None),
                right: Box::new(None),
            });
            right
        } else {
            let right = node.right.get_or_insert(Node {
                value: 0,
                left: Box::new(None),
                right: Box::new(None),
            });
            insert(right, value, height)
        }
    } else {
        if node.left.is_none() {
            let left = node.left.insert(Node {
                value: value,
                left: Box::new(None),
                right: Box::new(None),
            });
            left
        } else {
            let left = node.left.get_or_insert(Node {
                value: 0,
                left: Box::new(None),
                right: Box::new(None),
            });
            insert(left, value, height)
        }
    }
}
fn main() {
    let node = Rc::new(RefCell::new(Node {
        value: 10,
        left: Box::new(None),
        right: Box::new(None),
    }));
    {
        node.borrow_mut().add(7);

        node.borrow_mut().add(6);

        node.borrow_mut().add(8);

        node.borrow_mut().add(11);
    }

    let node = node.borrow();

    node.in_order_traversal();
    println!("\n");
    node.pre_order_traversal();
    println!("\n");
    node.post_order_traversal();
    println!("\n");
    node.reverse_order_traversal();
}