use std::{collections::BTreeMap, ops, str::FromStr};

use crate::Args;

const FILE_CONTENTS: &'static str = include_str!("../inputs/day_one.txt");

pub fn part_one(_args: Args) {
    let sum = FILE_CONTENTS
        .split('\n')
        .filter_map(|c| {
            let mut chars = c.chars().filter(|c: &char| c.is_numeric());
            let next_digit = chars.next().unwrap();
            let data = if let Some(last_digit) = chars.rev().next() {
                [next_digit, last_digit]
            } else {
                [next_digit, next_digit]
            };
            return Some(data.iter().collect::<String>().parse::<i64>().unwrap());
        })
        .reduce(|acc, x| acc + x)
        .unwrap();
    println!("{:?}", sum);
}

struct NumberLike {
    pub inner: u64,
}

lazy_static::lazy_static! {
    static ref TREE: BTreeMap<char, TreeEntry> = build_tree();
}

impl FromStr for NumberLike {
    type Err = std::fmt::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        println!("---------------");
        println!("Starting {}", s);
        let _chars = s.chars();

        let mut node: &BTreeMap<_, _> = &TREE;
        let mut first = None;
        let mut last = None;
        let buf = s.chars().collect::<Vec<_>>();
        let mut next = 0;
        let len = buf.len();

        while next < len as u64 {
            let c = buf[next as usize];

            if c.is_numeric() {
                println!("{} is numeric", c);
                if first.is_none() {
                    first = Some(c.to_digit(10).unwrap() as u64);
                } else {
                    last = Some(c.to_digit(10).unwrap() as u64);
                }
                node = &TREE;
                next += 1;
                continue;
            }

            if !TREE.contains_key(&c) {
                println!("Invalid character {}", c);
            next += 1;
                continue;
            }

            let entry = node.get(&c).unwrap();
            println!("Found entry: {}", c);

            match entry {
                TreeEntry::InnerTree(btree) => {
                    let mut node: &BTreeMap<_, _> = &btree;
                    println!("Entry was tree");
                    node = btree;

                    let mut i = 0;
                    loop {
                        i += 1;
                        if (next as usize + i) >= len {
                            break;
                        }
                        let c = buf[(next as usize + i) as usize];
                        if !node.contains_key(&c) {
                            break;
                        }

                        let next_node = node.get(&c).unwrap();
                        match next_node {
                            TreeEntry::InnerTree(btree) => {
                                println!("Entry was tree");
                                node = btree;
                            }
                            TreeEntry::Number(num) => {
                                println!("Entry was number");
                                if first.is_none() {
                                    first = Some(*num);
                                } else {
                                    last = Some(*num);
                                }
                                break;
                            }
                        }
                    }
                }
                TreeEntry::Number(num) => {
                    println!("Entry was number");
                    if first.is_none() {
                        first = Some(*num);
                    } else {
                        last = Some(*num);
                    }
                }
            }

            next += 1;
        }

        if last.is_none() {
            last = first;
        }

        println!("Finishing: {}{}", first.unwrap(), last.unwrap());
        println!("---------------");

        return Ok(Self {
            inner: (first.unwrap() * 10) + last.unwrap(),
        });
    }
}

impl ops::Add for NumberLike {
    type Output = NumberLike;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            inner: self.inner + rhs.inner,
        }
    }
}

impl std::fmt::Display for NumberLike {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner)
    }
}

pub fn part_two(_args: Args) {
    let sum = FILE_CONTENTS
        .split('\n')
        .filter_map(|c| {
            let res = c.parse::<NumberLike>().unwrap();
            println!("{}", res);
            return Some(res);
        })
        .reduce(|acc, x| acc + x)
        .unwrap();
    println!("{:?}", sum.inner);
}

enum TreeEntry {
    Number(u64),
    InnerTree(BTreeMap<char, TreeEntry>),
}

fn build_tree() -> BTreeMap<char, TreeEntry> {
    let tree = BTreeMap::from([
        (
            'o',
            TreeEntry::InnerTree(BTreeMap::from([(
                'n',
                TreeEntry::InnerTree(BTreeMap::from([('e', TreeEntry::Number(1))])),
            )])),
        ),
        (
            't',
            TreeEntry::InnerTree(BTreeMap::from([
                (
                    'w',
                    TreeEntry::InnerTree(BTreeMap::from([('o', TreeEntry::Number(2))])),
                ),
                (
                    'h',
                    TreeEntry::InnerTree(BTreeMap::from([(
                        'r',
                        TreeEntry::InnerTree(BTreeMap::from([(
                            'e',
                            TreeEntry::InnerTree(BTreeMap::from([('e', TreeEntry::Number(3))])),
                        )])),
                    )])),
                ),
            ])),
        ),
        (
            'f',
            TreeEntry::InnerTree(BTreeMap::from([
                (
                    'o',
                    TreeEntry::InnerTree(BTreeMap::from([(
                        'u',
                        TreeEntry::InnerTree(BTreeMap::from([('r', TreeEntry::Number(4))])),
                    )])),
                ),
                (
                    'i',
                    TreeEntry::InnerTree(BTreeMap::from([(
                        'v',
                        TreeEntry::InnerTree(BTreeMap::from([('e', TreeEntry::Number(5))])),
                    )])),
                ),
            ])),
        ),
        (
            's',
            TreeEntry::InnerTree(BTreeMap::from([
                (
                    'i',
                    TreeEntry::InnerTree(BTreeMap::from([('x', TreeEntry::Number(6))])),
                ),
                (
                    'e',
                    TreeEntry::InnerTree(BTreeMap::from([(
                        'v',
                        TreeEntry::InnerTree(BTreeMap::from([(
                            'e',
                            TreeEntry::InnerTree(BTreeMap::from([('n', TreeEntry::Number(7))])),
                        )])),
                    )])),
                ),
            ])),
        ),
        (
            'e',
            TreeEntry::InnerTree(BTreeMap::from([(
                'i',
                TreeEntry::InnerTree(BTreeMap::from([(
                    'g',
                    TreeEntry::InnerTree(BTreeMap::from([(
                        'h',
                        TreeEntry::InnerTree(BTreeMap::from([('t', TreeEntry::Number(8))])),
                    )])),
                )])),
            )])),
        ),
        (
            'n',
            TreeEntry::InnerTree(BTreeMap::from([(
                'i',
                TreeEntry::InnerTree(BTreeMap::from([(
                    'n',
                    TreeEntry::InnerTree(BTreeMap::from([('e', TreeEntry::Number(9))])),
                )])),
            )])),
        ),
    ]);

    return tree;
}
