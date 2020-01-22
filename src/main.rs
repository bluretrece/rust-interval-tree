#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Interval {
    pub interval: (i32, i32)
}

pub type Child = Option<Box<Node>>;

#[derive(PartialEq, Debug)]
pub struct Node {
    interval: Interval,
    left: Child,
    right: Child,
    maxim: i32,
}

impl Interval {
    pub fn new(interval: (i32, i32)) -> Interval {
        Interval {
            interval
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct Tree {
    root: Child,
}

pub fn get_max(current_node: &Node, new_node: &Node) -> i32 {
    let mut new_max: i32 = if new_node.interval.interval.1 > current_node.maxim {
        new_node.interval.interval.1
    } else {
        current_node.maxim
    };
    new_max
}

impl Tree {
    pub fn new() -> Tree {
        Tree { root: None }
    }

    pub fn insert(&mut self, node: Node){
        match &mut self.root {
            Some(node_root) => {
                if node.interval.interval.0 <= node_root.interval.interval.0 {
                    node_root.maxim = get_max(&node_root, &node);
                    println!(
                        "Node inserted to the left: at interval [{}, {}]",
                        &node.interval.interval.0, &node.interval.interval.1
                    );
                    println!("Root maximun value updated: {}", &node_root.maxim);
                    node_root.left = Some(Box::new(node));

                } else {
                    if node.interval.interval.0 >= node_root.interval.interval.0 {
                        println!("Node inserted to the right at interval: [{}, {}]",
                        &node.interval.interval.0, &node.interval.interval.1);
                        node_root.right = Some(Box::new(node));

                    }
                }
            }
            _ => {
                println!(
                    "New root inserted: [{}, {}]",
                    node.interval.interval.1, node.interval.interval.0
                );
                self.root = Some(Box::new(node));
            }
        }
    }
}


pub fn interval_search(root: Result<&Tree, &Node>, interval: &Interval) -> Interval {
    //if root overlaps with given interval, return the root's interval
    if overlap(&root.unwrap().root.as_ref().unwrap().interval, &interval) {
        return root.unwrap().root.as_ref().unwrap().interval
    }

    match (&root.unwrap().root.as_ref().unwrap().left, &root.unwrap().root.as_ref().unwrap().right) {
        (&Some(ref left), _) if left.maxim >= interval.interval.0 => {
            interval_search(Err(&left), &interval)
        },
        (_, &Some(ref right)) => {
            interval_search(Err(&right), &interval)
        },
        _ => unimplemented!("What should we do here?"),
    }

    // if !root.left.is_none() && root.left.as_ref().expect("Child failed to unwrap").maxim >= interval.interval.0 {
    //     return interval_search(&root.left.as_ref().expect("Child failed to unwrap"), &interval) 
    // } 
    // else {
    //     return interval_search(&root.right.as_ref().expect("Child failed to unwrap"), &interval)
    // }
}

// Receives two node's intervals.
pub fn overlap(node_interval1: &Interval, interval2: &Interval) -> bool {
    node_interval1.interval.0 <= interval2.interval.1 && interval2.interval.0 <= node_interval1.interval.1
}

pub fn in_order_traversal(root: &Node) {
    if !root.left.is_none() {
        in_order_traversal(&root.left.as_ref().unwrap())
    }

    println!("{:?}", &root);


    if !root.right.is_none() {
        in_order_traversal(&root.right.as_ref().unwrap())
    }
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

    let mut intervals:Vec<Interval> = vec![Interval::new((5,20)), Interval::new((10,30)), Interval::new((7,6))];

    let mut tree = Tree::new();
    for i in 0..intervals.len() {
        tree.insert(Node::new(intervals[i]))
    }


    let mut new_tree = Tree::new();
    let new_interval = Interval { interval: (5, 20)};
    let new_interval2 = Interval { interval: (10, 30) };
    //Overlap interval
    let new_interval3 = Interval { interval: (7, 6)};
    let mut n1 = Node::new(new_interval);
    let mut n2 = Node::new(new_interval2);
    println!("{:?}", new_tree.insert(n1));
    println!("{:?}", new_tree.insert(n2));
    //println!("Overlap test: {:?}", overlap(&new_interval, &new_interval3));
    println!("Overlap test: {:?}", interval_search(Ok(&new_tree), &new_interval3));


}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn node_creation() {
        let new_interval = Interval { interval: (5, 15)};
        let new_node = Node::new(new_interval);
        assert_eq!(new_node.interval.interval.1, 1);
        assert_eq!(new_node.interval.interval.0, 5);
    }

    #[test]
    fn tree_creation() {
        let mut new_tree = Tree::new();
        assert_eq!(new_tree.root, None);
    }

    #[test]
    fn root_insert() {
        let mut new_tree = Tree::new();
        let new_interval = Interval { interval: (5, 15)};
        let n1 = Node::new(new_interval);
        new_tree.insert(n1);
        println!("{}", new_tree.root.interval.interval.1);
    }
}
