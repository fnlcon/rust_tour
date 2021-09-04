use rand::thread_rng;
use std::collections::LinkedList;

#[derive(PartialEq, Debug, Clone)]
enum NodeType {
    String(Option<String>),
    Currency(String),
}

#[derive(Debug, Clone)]
struct NodeMeta {
    r#type: Option<NodeType>,
    iteration: bool,
    attribute: bool,
    mandatory: String,
}

impl Default for NodeMeta {
    fn default() -> Self {
        NodeMeta {
            r#type: None,
            iteration: false,
            attribute: false,
            mandatory: "".to_string(),
        }
    }
}

impl From<&str> for NodeMeta {
    // expect NC|||M
    fn from(text: &str) -> Self {
        let spec: Vec<&str> = text.split("|").collect();
        assert_eq!(spec.len(), 4);
        let r#type = match spec[0] {
            "NC" | "C" => {
                Some(NodeType::String(Some(spec[2].to_owned())))
            }
            "BigDecimal" | "Currency" => {
                Some(NodeType::Currency(spec[2].to_owned()))
            }
            _ => None
        };
        NodeMeta {
            r#type,
            iteration: false,
            attribute: false,
            mandatory: spec[3].to_owned(),
        }
    }
}

#[derive(Clone, Debug)]
struct Attribute(String);

#[derive(Clone, Debug)]
struct Node(String, NodeMeta, LinkedList<Attribute>, LinkedList<Node>);

impl<'a> Node {
    pub fn name(&'a self) -> &'a str {
        let Node(name, _, _, _) = self;
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

impl From<&str> for Path {
    fn from(s: &str) -> Self {
        if s.starts_with("#") {
            Path::Attribute(s[1..s.len()].to_owned())
        } else if s.ends_with("[]") {
            Path::Node(s[0..(s.len() - 2)].to_owned(), true)
        } else {
            Path::Node(s[..].to_owned(), false)
        }
    }
}

fn update<'a, I: Iterator<Item=&'a Path>>(node: Node, path: &mut I) -> Node {
    fn update_children(children: LinkedList<Node>) -> LinkedList<Node> {
        children.iter()
            .map(|node| {
                let node = node.to_owned();
                if node.name() == &node_name[..] {
                    update(node, path)
                } else {
                    node
                }
            })
            .collect()
    }

    fn append_child(children: LinkedList<Node>, node: Node) -> LinkedList<Node> {
        let mut children = LinkedList::from(children);
        children.push_back(node);
        children
    }

    match path.next() {
        None => node,
        Some(Path::Node(node_name, iteration)) => {
            let Node(name, meta, attributes, children) = node;
            let exists = children.iter().any(|node| node.name() == &node_name[..]);
            let children = if exists {
                update_children(children)
            } else {
                let new_node = Node(node_name.to_owned(), Default::default(), Default::default(), Default::default());
                let node = update(new_node, path);
                append_child(children, node)
            };

            Node(name, meta, attributes, children)
        }

        Some(Path::Attribute(attribute)) => {
            match node {
                Node(name, meta, attributes, children) => {
                    let mut attributes = LinkedList::from(attributes);
                    attributes.push_back(Attribute(attribute[..].to_owned()));
                    Node(name, meta, attributes, children)
                }
            }
        }

        _ => {
            todo!()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{update, Node, Path, NodeMeta, NodeType};
    use rand::{thread_rng, Rng};
    use std::borrow::{Borrow, BorrowMut};
    use std::collections::LinkedList;
    use std::fs::File;

    #[test]
    fn test_node() {
        let node = Node("/".to_owned(), Default::default(), Default::default(), Default::default());

        let Node(name, _, _, _) = node;

        assert_eq!(name, "/");
    }

    #[test]
    fn test_update() {
        let path: Vec<Path> = "/Request/Date".split("/").map(|n| Path::Node(n.to_owned(), false)).collect();
        let mut node = Node("/".to_owned(), Default::default(), Default::default(), Default::default());
        node = update(node, &mut path.iter());
    }

    #[test]
    fn test_node_meta_from() {
        let node_meta: NodeMeta = "NC||12|M".into();

        assert_eq!(node_meta.r#type.unwrap(), NodeType::String(Some("12".into())));
        assert_eq!(node_meta.iteration, false);
        assert_eq!(node_meta.attribute, false);
        assert_eq!(node_meta.mandatory, "M".to_string());
    }

    #[test]
    fn test_iteration() {
        let mut node = Node("/".to_owned(), Default::default(), Default::default(), Default::default());

        let node = std::fs::read_to_string("/Users/renxunxiao/repos/rust_tour/test.txt").unwrap()
            .lines()
            .into_iter()
            .map(|line| line.split("/").map(From::from).skip(1).collect::<Vec<Path>>())
            .fold(node, |acc, node| {
                update(acc, &mut node.iter())
            });

        println!("{:#?}", &node)
    }
}
