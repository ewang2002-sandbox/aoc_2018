use std::collections::{VecDeque};

#[allow(dead_code)]
pub fn execute(input: &Vec<String>) -> (usize, usize) {
    let header: Vec<usize> = input[0]
        .split(" ")
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    let mut header_deque: VecDeque<usize> = VecDeque::from(header);

    // Construct the "tree"
    let tree = Tree::new(&mut header_deque);

    return (part1(&tree), part2());
}

// https://adventofcode.com/2018/day/8
//
// --- Day 8: Memory Maneuver ---
// The sleigh is much easier to pull than you'd expect for something its weight. Unfortunately,
// neither you nor the Elves know which way the North Pole is from here.
//
// You check your wrist device for anything that might help. It seems to have some kind of
// navigation system! Activating the navigation system produces more bad news: "Failed to start
// navigation system. Could not read software license file."
//
// The navigation system's license file consists of a list of numbers (your puzzle input). The
// numbers define a data structure which, when processed, produces some kind of tree that can be
// used to calculate the license number.
//
// The tree is made up of nodes; a single, outermost node forms the tree's root, and it contains
// all other nodes in the tree (or contains nodes that contain nodes, and so on).
//
// Specifically, a node consists of:
//
// A header, which is always exactly two numbers:
// The quantity of child nodes.
// The quantity of metadata entries.
// Zero or more child nodes (as specified in the header).
// One or more metadata entries (as specified in the header).
// Each child node is itself a node that has its own header, child nodes, and metadata. For
// example:
//
//  2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2
//  A----------------------------------
//      B----------- C-----------
//                       D-----
// In this example, each node of the tree is also marked with an underline starting with a letter
// for easier identification. In it, there are four nodes:
// - A, which has 2 child nodes (B, C) and 3 metadata entries (1, 1, 2).
// - B, which has 0 child nodes and 3 metadata entries (10, 11, 12).
// - C, which has 1 child node (D) and 1 metadata entry (2).
// - D, which has 0 child nodes and 1 metadata entry (99).
// The first check done on the license file is to simply add up all of the metadata entries. In
// this example, that sum is 1+1+2+10+11+12+2+99=138.
//
// What is the sum of all metadata entries?

pub fn part1(tree: &Tree) -> usize {
    return tree.sum_of_meta_data();
}

pub fn part2() -> usize {
    return 0;
}

struct Node {
    child_nodes: Vec<Node>,
    metadata_entries: Vec<usize>
}

pub struct Tree {
    root_node: Node
}

impl Tree {
    /// Creates a new Tree from the given input.
    ///
    /// # Parameters
    /// * `input` - The puzzle input.
    ///
    /// # Returns
    /// The new `Tree`.
    pub fn new(input: &mut VecDeque<usize>) -> Self {
        let mut node = Node {child_nodes: vec![], metadata_entries: vec![]};
        Tree::process_input(input, &mut node);
        return Tree {root_node: node};
    }

    /// Populates the tree with the nodes.
    ///
    /// # Parameters
    /// * `input` - The input vector.
    /// * `curr_node` - The current node. Note that this node refers to the *current* node to
    /// deal with. This does *not* mean the previous (parent) node.
    fn process_input(input: &mut VecDeque<usize>, curr_node: &mut Node) {
        // No more input to process.
        if input.is_empty() {
            return;
        }

        let mut num_child_nodes = input.pop_front().expect("[a]");
        let mut num_meta_data = input.pop_front().expect("[b]");

        // If no child nodes, then put metadata in the current node and leave
        if num_child_nodes == 0 {
            while num_meta_data > 0 {
                curr_node.metadata_entries.push(input.pop_front().expect("[c]"));
                num_meta_data -= 1;
            }

            return;
        }

        // Otherwise, create a new node for each child node then attach to the current node.
        while num_child_nodes > 0 {
            let mut new_node = Node {child_nodes: vec![], metadata_entries: vec![]};
            Tree::process_input(input, &mut new_node);
            curr_node.child_nodes.push(new_node);
            num_child_nodes -= 1;
        }

        // Then, add the meta data to the current node.
        while num_meta_data > 0 {
            curr_node.metadata_entries.push(input.pop_front().expect("[d]"));
            num_meta_data -= 1;
        }
    }

    /// Gets the sum of all metadata entries.
    ///
    /// # Returns
    /// The sum of all metadata entries.
    pub fn sum_of_meta_data(&self) -> usize {
        return Tree::p_sum_meta_data(&self.root_node);
    }

    fn p_sum_meta_data(node: &Node) -> usize {
        let mut node_sum: usize = 0;
        for n in &node.child_nodes {
            node_sum += Tree::p_sum_meta_data(&n);
        }

        return node.metadata_entries.iter().sum::<usize>() + node_sum;
    }
}