use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::rc::Rc;

fn main() {
    challenge_1();
}

fn challenge_1() {
    let input = read_input("input");
    println!(
        "Challenge 1: Total number of orbits = {}",
        number_of_orbits(&input)
    );
}

fn read_input(filename: &str) -> String {
    let mut buffer = String::new();
    File::open(filename)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();
    buffer
}

fn number_of_orbits(s: &str) -> u32 {
    let root = Node::parse(s);
    let mut orbits = 0;
    let visit = Box::new(|node: RcNode| {
        orbits += number_of_ancestors(Rc::clone(&node));
    });
    walk(root, visit);
    orbits
}

type RcNode = Rc<RefCell<Node>>;

#[derive(Default, Debug)]
struct Node {
    label: String,
    parent: Option<RcNode>,
    children: Vec<RcNode>,
}

impl Node {
    fn parse(input: &str) -> RcNode {
        let mut nodes: HashMap<&str, RcNode> = Default::default();
        for line in input.trim().lines() {
            let parts: Vec<_> = line.trim().split(')').collect();
            let parent_label = parts[0];
            let child_label = parts[1];
            match (nodes.get(parent_label), nodes.get(child_label)) {
                (Some(parent), Some(child)) => {
                    parent.borrow_mut().children.push(Rc::clone(&child));
                    child.borrow_mut().parent = Some(Rc::clone(&parent));
                }
                (Some(parent), None) => {
                    let child = Node::make(child_label, Some(Rc::clone(&parent)), Vec::new());
                    parent.borrow_mut().children.push(Rc::clone(&child));
                    nodes.insert(child_label, Rc::clone(&child));
                }
                (None, Some(child)) => {
                    let parent = Node::make(parent_label, None, vec![Rc::clone(&child)]);
                    child.borrow_mut().parent = Some(Rc::clone(&parent));
                    nodes.insert(parent_label, Rc::clone(&parent));
                }
                (None, None) => {
                    let parent = Node::make(parent_label, None, Vec::new());
                    let child = Node::make(child_label, Some(Rc::clone(&parent)), Vec::new());
                    parent.borrow_mut().children.push(Rc::clone(&child));
                    nodes.insert(parent_label, Rc::clone(&parent));
                    nodes.insert(child_label, Rc::clone(&child));
                }
            }
        }
        for (_, node) in nodes {
            if node.borrow().parent.is_none() {
                return node;
            }
        }
        unreachable!();
    }

    fn make(label: &str, parent: Option<RcNode>, children: Vec<RcNode>) -> RcNode {
        Rc::new(RefCell::new(Node {
            label: label.to_owned(),
            parent,
            children,
        }))
    }

    #[allow(dead_code)]
    fn parent_label(&self) -> Option<String> {
        match &self.parent {
            Some(v) => Some(v.borrow().label.clone()),
            None => None,
        }
    }
}

fn walk<F>(node: RcNode, mut visit: Box<F>) -> Box<F>
where
    F: FnMut(RcNode),
{
    visit(Rc::clone(&node));
    for child in &node.borrow().children {
        visit = walk(Rc::clone(&child), visit);
    }
    visit
}

fn number_of_ancestors(node: RcNode) -> u32 {
    if node.borrow().parent.is_none() {
        return 0;
    }
    let node = Rc::clone(&node);
    let node = node.borrow();
    number_of_ancestors(Rc::clone(node.parent.as_ref().unwrap())) + 1
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]

    use super::*;

    #[test]
    fn reads_input() {
        let input = read_input("input");
        assert!(input.starts_with("MQD)G37\nMPH)V45"));
    }

    #[test]
    fn builds_tree_for_single_case() {
        let root = Node::parse("A)B");
        let root = root.borrow();
        assert_eq!(root.label, "A".to_owned());
        assert_eq!(root.parent.is_some(), false);
        assert_eq!(root.children.len(), 1);
        let node_b = root.children[0].borrow();
        assert_eq!(node_b.label, "B".to_owned());
        assert_eq!(node_b.parent_label(), Some("A".to_owned()));
        assert_eq!(node_b.children.len(), 0);
    }

    #[test]
    fn builds_tree_for_B_C_orbits_A() {
        let root = Node::parse("A)B\nA)C");
        let root = root.borrow();
        assert_eq!(root.label, "A".to_owned());
        assert_eq!(root.parent.is_some(), false);
        assert_eq!(root.children.len(), 2);
        let node_b = root.children[0].borrow();
        assert_eq!(node_b.label, "B".to_owned());
        assert_eq!(node_b.parent_label(), Some("A".to_owned()));
        assert_eq!(node_b.children.len(), 0);
        let node_c = root.children[1].borrow();
        assert_eq!(node_c.label, "C".to_owned());
        assert_eq!(node_c.parent_label(), Some("A".to_owned()));
        assert_eq!(node_c.children.len(), 0);
    }

    #[test]
    fn builds_tree_for_C_orbits_B_orbits_A() {
        let root = Node::parse("A)B\nB)C");
        let root = root.borrow();
        assert_eq!(root.label, "A".to_owned());
        assert_eq!(root.parent.is_some(), false);
        assert_eq!(root.children.len(), 1);
        let node_b = root.children[0].borrow();
        assert_eq!(node_b.label, "B".to_owned());
        assert_eq!(node_b.parent_label(), Some("A".to_owned()));
        assert_eq!(node_b.children.len(), 1);
        let node_c = node_b.children[0].borrow();
        assert_eq!(node_c.label, "C".to_owned());
        assert_eq!(node_c.parent_label(), Some("B".to_owned()));
        assert_eq!(node_c.children.len(), 0);
    }

    #[test]
    fn builds_tree_where_node_created_as_child_before_parent() {
        let root = Node::parse("B)C\nA)B");
        let root = root.borrow();
        assert_eq!(root.label, "A".to_owned());
        assert_eq!(root.parent.is_some(), false);
        assert_eq!(root.children.len(), 1);
        let node_b = root.children[0].borrow();
        assert_eq!(node_b.label, "B".to_owned());
        assert_eq!(node_b.parent_label(), Some("A".to_owned()));
        assert_eq!(node_b.children.len(), 1);
        let node_c = node_b.children[0].borrow();
        assert_eq!(node_c.label, "C".to_owned());
        assert_eq!(node_c.parent_label(), Some("B".to_owned()));
        assert_eq!(node_c.children.len(), 0);
    }

    #[test]
    fn builds_tree_where_child_and_parent_created_seperately_before_linked() {
        // A - B - C - D
        let root = Node::parse(
            r#"
            A)B
            C)D
            B)C
            "#,
        );
        let root = root.borrow();
        assert_eq!(root.label, "A".to_owned());
        assert_eq!(root.parent.is_some(), false);
        assert_eq!(root.children.len(), 1);
        let node_b = root.children[0].borrow();
        assert_eq!(node_b.label, "B".to_owned());
        assert_eq!(node_b.parent_label(), Some("A".to_owned()));
        assert_eq!(node_b.children.len(), 1);
        let node_c = node_b.children[0].borrow();
        assert_eq!(node_c.label, "C".to_owned());
        assert_eq!(node_c.parent_label(), Some("B".to_owned()));
        assert_eq!(node_c.children.len(), 1);
        let node_d = node_c.children[0].borrow();
        assert_eq!(node_d.label, "D".to_owned());
        assert_eq!(node_d.parent_label(), Some("C".to_owned()));
        assert_eq!(node_d.children.len(), 0);
    }

    #[test]
    fn calculates_orbits_for_single_case() {
        assert_eq!(number_of_orbits("A)B"), 1);
    }

    #[test]
    fn calculates_orbits_for_2_orbits_1() {
        assert_eq!(number_of_orbits("A)B\nA)C"), 2);
    }

    #[test]
    fn calculates_orbits_for_3_orbits_1() {
        assert_eq!(number_of_orbits("A)B\nA)C\nA)D"), 3);
    }

    #[test]
    fn calculates_orbits_for_C_orbits_B_orbits_A() {
        assert_eq!(number_of_orbits("A)B\nB)C"), 3);
    }

    #[test]
    fn calculates_orbits_when_root_isnt_first() {
        assert_eq!(number_of_orbits("B)C\nA)B"), 3);
    }

    #[test]
    fn calculates_orbits_for_example() {
        let orbits = number_of_orbits(
            r#"
            COM)B
            B)C
            C)D
            D)E
            E)F
            B)G
            G)H
            D)I
            E)J
            J)K
            K)L
            "#,
        );
        assert_eq!(orbits, 42);
    }
}
