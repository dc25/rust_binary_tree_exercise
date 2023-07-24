use std::{cell::RefCell, rc::Rc};

type NodePtr=Box<Option<Node>>;

#[derive(Debug)]
struct Node {
    value: i32,
    left: NodePtr,
    right: NodePtr,
}

impl Node {
    fn add(&mut self, value: i32) {
        let mut height = 1;
        insert(self, value, height);
    }

    fn in_order_traversal(&self) {
        (*self.left).as_ref().map(|n| n.in_order_traversal());
        println!("{:?}", self.value);
        (*self.right).as_ref().map(|n| n.in_order_traversal());
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
}
