use std::fmt::Display;

type NodeRef<T> = Option<Box<Node<T>>>;

#[derive(Debug, Default, Clone)]
struct Node<T> {
    value: T,
    left: NodeRef<T>,
    right: NodeRef<T>
}

enum Action<T, U> {
    Call(T),
    Handle(U)
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

fn print_tree<T: Display>(root: &NodeRef<T>, level: usize) {
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

fn print_tree_nonrec<T: Display>(root: &NodeRef<T>) {
    use Action::*;
    let mut stack = Vec::<Action<(&NodeRef<T>, usize), (&T, usize)>>::new();
    stack.push(Call((root, 0)));
    while let Some(action) = stack.pop() {
        match action {
            Call((node, level)) => {
                if let Some(node) = node {
                    stack.push(Call((&node.right, level + 1)));
                    stack.push(Handle((&node.value, level)));
                    stack.push(Call((&node.left, level + 1)));
                }
            },
            Handle((value, level)) => {
                for _ in 0..level {
                    print!("  ");
                }
                println!("{}", value);
            }
        }
    }
}

fn invert_tree<T: Clone>(root: &NodeRef<T>) -> NodeRef<T> {
    match root {
        Some(r) => Some(Box::new(Node {
            value: r.value.clone(),
            left: invert_tree(&r.right),
            right: invert_tree(&r.left),
        })),
        None => None,
    }
}

fn main() {
    let mut counter: i32 = 1;
    let root = generate_tree(3, &mut counter);
    print_tree(&root, 0);
    println!("--------------------inverted tree--------------------");
    print_tree(&invert_tree(&root), 0);
    println!("------------------non recursive print----------------");
    print_tree_nonrec(&root);
}