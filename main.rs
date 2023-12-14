use std::fmt::Display;

type NodeRef<T> = Option<Box<Node<T>>>;

#[derive(Debug, Default, Clone)]
struct Node<T> {
    value: T,
    left: NodeRef<T>,
    right: NodeRef<T>
}

fn generate_tree(level: i32, counter: &mut i32) -> NodeRef<i32> {
    if level == 0 {
        None
    } else {
        let mut root = Node::default();
        root.value = *counter;
        *counter += 1;
        root.left = generate_tree(level - 1, counter);
        root.right = generate_tree(level - 1, counter);
        Some(Box::new(root))
    }
}

fn print_tree<T: Display> (root: &NodeRef<T>, level: usize) {
    match root {
        Some(n) => {
            print_tree(&n.left, level + 1);
            for _ in 0..level {
                print!("  ");
            }
            println!("{}", n.value);
            print_tree(&n.right, level + 1);
        },
        None => {}
    }
}

fn invert_tree<T: Clone> (root: &NodeRef<T>) -> NodeRef<T> {
    match root {
        Some(r) => {
            Some(Box::new(Node{
                value: r.value.clone(),
                left: invert_tree(&r.right),
                right: invert_tree(&r.left)
            }))
        },
        None => None
    }
}

fn main() {
    let mut counter: i32 = 1;
    let root = generate_tree(3, &mut counter);
    print_tree(&root, 0);
    println!("--------------------inverted tree--------------------");
    print_tree(&invert_tree(&root), 0);
}