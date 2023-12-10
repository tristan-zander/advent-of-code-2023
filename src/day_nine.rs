use std::{cell::RefCell, rc::Rc, str::FromStr};

use itertools::Itertools;

use crate::Args;

const FILE_CONTENTS: &'static str = include_str!("../inputs/day_nine.txt");

#[derive(Debug, Clone)]
struct Node {
    pub value: i64,
    pub next: Option<Rc<RefCell<Node>>>,
}

#[derive(Debug, Clone)]
struct DayNine {
    pub nodes: Vec<Vec<Rc<RefCell<Node>>>>,
}

impl DayNine {
    pub fn next_numbers(&self) -> Vec<i64> {
        self.nodes
            .iter()
            .map(|line| Self::extrapolate_values(line.last().unwrap().to_owned(), false))
            .collect_vec()
    }

    pub fn previous_numbers(&self) -> Vec<i64> {
        self.nodes
            .iter()
            .map(|line| Self::extrapolate_values(line.first().unwrap().to_owned(), true))
            .collect_vec()
    }

    fn extrapolate_values(mut current: Rc<RefCell<Node>>, top_down: bool) -> i64 {
        let mut path = vec![current.to_owned()];

        while current.as_ref().borrow().next.is_some() {
            let next = current.as_ref().borrow().next.to_owned().unwrap();
            current = next;
            path.push(current.to_owned());
        }

        if top_down == false {
            let sum = path.iter().map(|node| node.borrow().value).sum::<i64>();
            println!("Sum: {}", sum);

            return sum;
        } else {
            let difference = path
                .iter()
                .map(|node| node.borrow().value)
                .rev()
                .reduce(|acc, x| {
                    println!("{}, {}", acc, x);
                    return x - acc;
                })
                .unwrap();

            println!("Difference: {}", difference);

            return difference;
        }
    }
}

impl FromStr for DayNine {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nodes = s
            .lines()
            .map(|l| {
                l.split_whitespace()
                    .map(|n| {
                        Rc::new(RefCell::new(Node {
                            value: n.parse().unwrap(),
                            next: None,
                        }))
                    })
                    .collect_vec()
            })
            .collect_vec();

        let mut this = Self { nodes };

        this.build_tree();

        Ok(this)
    }
}

impl DayNine {
    pub fn build_tree(&mut self) {
        for nodes in &mut self.nodes {
            Self::build_layer(nodes);
        }
    }

    fn build_layer(layer: &mut Vec<Rc<RefCell<Node>>>) {
        let mut new_nodes = Vec::with_capacity(layer.len() - 1);
        for i in 0..layer.len() - 1 {
            let [ref mut left, ref mut right, ..] = layer[i..=i + 1] else {
                unreachable!()
            };
            let (mut left, mut right) = (left.as_ref().borrow_mut(), right.as_ref().borrow_mut());
            let difference = right.value - left.value;

            let node = Rc::new(RefCell::new(Node {
                value: difference,
                next: None,
            }));

            left.next = Some(node.clone());
            right.next = Some(node.clone());

            new_nodes.push(node);
        }

        if new_nodes.iter().all(|n| n.as_ref().borrow().value == 0) {
            return;
        }

        Self::build_layer(&mut new_nodes);
    }
}

pub fn part_one(_args: Args) {
    let tree = FILE_CONTENTS.parse::<DayNine>().unwrap();
    println!("Sum: {}", tree.next_numbers().iter().sum::<i64>());
}
pub fn part_two(_args: Args) {
    let tree = FILE_CONTENTS.parse::<DayNine>().unwrap();
    println!("Sum: {}", tree.previous_numbers().iter().sum::<i64>());
}
