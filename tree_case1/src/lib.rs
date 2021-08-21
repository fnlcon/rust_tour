use rand::thread_rng;
use std::collections::LinkedList;

#[derive(Clone, Debug)]
struct Node(String, LinkedList<Node>);

impl<'a> Node {
    pub fn name(&'a self) -> &'a str {
        let Node(name, _) = self;
        &name[..]
    }
}

impl AsRef<Node> for Node {
    fn as_ref(&self) -> &Node {
        return self;
    }
}

enum Path {
    Attribute(String),
    Node(String, bool),
}

fn update<'a, I: Iterator<Item = &'a Path>>(node: Node, path: &mut I) -> Node {
    match path.next() {
        None => node,
        Some(Path::Node(node_name, iteration)) => {
            let Node(name, children) = node;
            let exists = children.iter().any(|node| node.name() == &node_name[..]);
            let children = if exists {
                children
                    .iter()
                    .map(|node| {
                        let node = node.to_owned();
                        if node.name() == &node_name[..] {
                            node
                        } else {
                            update(node, path)
                        }
                    })
                    .collect()
            } else {
                let new_node = Node(node_name.to_owned(), Default::default());
                let node = update(new_node, path);

                let mut children = LinkedList::from(children);
                children.push_back(node);
                children
            };

            Node(name, children)
        }
        _ => {
            todo!()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{update, Node, Path};
    use rand::{thread_rng, Rng};
    use std::borrow::{Borrow, BorrowMut};
    use std::collections::LinkedList;

    #[test]
    fn test_node() {
        let node = Node("/".to_owned(), LinkedList::default());

        let Node(name, _) = node;

        assert_eq!(name, "/");
    }

    #[test]
    fn test_update() {
        let path: Vec<Path> = "/Request/Date".split("/").map(|n| Path::Node(n.to_owned(), false)).collect();
        let mut node = Node("/".to_owned(), LinkedList::default());
        node = update(node, &mut path.iter());


    }
}
