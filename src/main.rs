#[derive(PartialEq, Debug)]
pub struct Interval {
    pub x: i32,
    pub y: i32,
}

type Child = Option<Box<Node>>;

#[derive(PartialEq, Debug)]
pub struct Node {
    interval: Interval,
    left: Child,
    right: Child,
    maxim: i32,
}

impl Interval {
    pub fn new(x: i32, y: i32) -> Interval {
        Interval { x, y }
    }
}

#[derive(PartialEq, Debug)]
pub struct Tree {
    root: Child,
}

pub fn get_max(current_node: &Node, new_node: &Node) -> i32 {
    let mut new_max: i32 = if new_node.interval.y > current_node.maxim {
        new_node.interval.y
    } else {
        current_node.maxim
    };
    new_max
}
impl Tree {
    pub fn new() -> Tree {
        Tree { root: None }
    }

    pub fn insert(&mut self, node: Node) {
        match &mut self.root {
            Some(node_root) => {
                if node.interval.x <= node_root.interval.x {
                    node_root.maxim = get_max(&node_root, &node);
                    println!(
                        "Node inserted to the left: at [{}, {}]",
                        &node.interval.x, &node.interval.y
                    );
                    println!("Root maximun value updated: {}", &node_root.maxim);
                    node_root.left = Some(Box::new(node))
                } else {
                    if node.interval.x >= node_root.interval.x {
                        println!("Node inserted to the right");
                        node_root.right = Some(Box::new(node))
                    }
                }
            }
            _ => {
                println!(
                    "New root inserted: [{}, {}]",
                    node.interval.x, node.interval.y
                );
                self.root = Some(Box::new(node));
            }
        }
    }

    // TODO work on this function.
    pub fn overlapping_interval_search(&mut self, root_node: Tree, interval: Interval) -> Interval {
        match &root_node.root {
            None => unimplemented!(),
            Some(root) => {
                if overlap(&root, &interval) {
                    root.interval
                } else {
                    unimplemented!()
                }
            }
            _ => unimplemented!(),
        }
    }
}

// TODO work on this function.
pub fn overlap(node_interval1: &Node, interval2: &Interval) -> bool {
    node_interval1.interval.x <= interval2.y && interval2.x <= node_interval1.interval.y
}

impl Node {
    pub fn new(inter: Interval) -> Node {
        Node {
            interval: inter,
            left: None,
            right: None,
            maxim: 0,
        }
    }
}

fn main() {
    let mut new_tree = Tree::new();
    let new_interval = Interval { x: 15, y: 20 };
    let new_interval2 = Interval { x: 10, y: 30 };
    let n1 = Node::new(new_interval);
    let n2 = Node::new(new_interval2);
    println!("{:?}", new_tree.insert(n1));
    println!("{:?}", new_tree.insert(n2));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn node_creation() {
        let new_interval = Interval { x: 1, y: 5 };
        let new_node = Node::new(new_interval);
        assert_eq!(new_node.interval.x, 1);
        assert_eq!(new_node.interval.y, 5);
    }

    #[test]
    fn tree_creation() {
        let mut new_tree = Tree::new();
        assert_eq!(new_tree.root, None);
    }

    #[test]
    fn root_insert() {
        let mut new_tree = Tree::new();
        let new_interval = Interval { x: 1, y: 5 };
        let n1 = Node::new(new_interval);
        new_tree.insert(n1);
        println!("{}", new_tree.root.interval.x);
    }
}
