use std::{
    cell::RefCell,
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    rc::Rc,
};

use crate::Args;

const FILE_CONTENTS: &'static str = include_str!("../inputs/day_seventeen.txt");

fn input() -> Vec<Vec<u32>> {
    FILE_CONTENTS
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

#[derive(PartialEq, PartialOrd, Eq, Clone, Debug)]
struct State {
    /// The cost plus the distance to the end
    priority: u64,
    /// The actual heat lost
    cost: u64,
    /// How many steps have happened since the last turn?
    forward_steps: u8,
    position: (usize, usize),
    previous: Option<Rc<RefCell<State>>>,
}

impl std::hash::Hash for State {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.position.hash(state);
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.priority.cmp(&other.priority)
    }
}

fn distance(position: (usize, usize), end_position: (usize, usize)) -> u64 {
    let distance = (position.0.abs_diff(end_position.0) + position.1.abs_diff(end_position.1)) as u64;
    distance + (distance / 4) * 2 
}

fn neighbors(input: &[Vec<u32>], state: &State) -> Vec<State> {
    let state_ptr = Rc::new(RefCell::new(state.clone()));
    if state.previous.is_none() {
        // Just move right and down. This is the start node.
        return vec![
            State {
                position: (1, 0),
                priority: u64::MAX,
                cost: u64::MAX,
                forward_steps: 0,
                previous: Some(state_ptr.clone()),
            },
            State {
                position: (0, 1),
                priority: u64::MAX,
                cost: u64::MAX,
                forward_steps: 0,
                previous: Some(state_ptr),
            },
        ];
    }

    let max_x = input[0].len() - 1;
    let max_y = input.len() - 1;

    let state = state_ptr.borrow();
    let prev = state.previous.clone().unwrap();
    let prev = prev.borrow();
    let difference = (
        state.position.0 as isize - prev.position.0 as isize,
        state.position.1 as isize - prev.position.1 as isize,
    );

    let mut neighbors = vec![];
    if state.forward_steps != 3 {
        // Move forward
        let forward = (
            state.position.0 as isize + difference.0,
            state.position.1 as isize + difference.1,
        );
        if forward.0 <= max_x as isize
            && forward.0 >= 0
            && forward.1 <= max_y as isize
            && forward.1 >= 0
        {
            neighbors.push(State {
                position: (forward.0 as usize, forward.1 as usize),
                priority: u64::MAX,
                cost: u64::MAX,
                forward_steps: state.forward_steps + 1,
                previous: Some(state_ptr.clone()),
            });
        }
    }

    if difference.0 > 0 {
        // Move up and down.
        let up = (state.position.0, state.position.1 as isize - 1);
        if up.1 <= max_y as isize && up.1 >= 0 {
            neighbors.push(State {
                position: (up.0, up.1 as usize),
                priority: u64::MAX,
                cost: u64::MAX,
                forward_steps: 0,
                previous: Some(state_ptr.clone()),
            });
        }
        let down = (state.position.0, state.position.1 as isize + 1);
        if down.1 <= max_y as isize && down.1 >= 0 {
            neighbors.push(State {
                position: (down.0, down.1 as usize),
                priority: u64::MAX,
                cost: u64::MAX,
                forward_steps: 0,
                previous: Some(state_ptr.clone()),
            });
        }
    } else {
        // Move left and right.
        let left = (state.position.0 as isize - 1, state.position.1);
        if left.0 >= 0 && left.0 <= max_x as isize {
            neighbors.push(State {
                position: (left.0 as usize, left.1),
                priority: u64::MAX,
                cost: u64::MAX,
                forward_steps: 0,
                previous: Some(state_ptr.clone()),
            });
        }
        let right = (state.position.0 as isize + 1, state.position.1);
        if right.0 >= 0 && right.0 <= max_x as isize {
            neighbors.push(State {
                position: (right.0 as usize, right.1),
                priority: u64::MAX,
                cost: u64::MAX,
                forward_steps: 0,
                previous: Some(state_ptr.clone()),
            });
        }
    }

    neighbors
}

fn shortest_path(input: &[Vec<u32>]) -> Option<u64> {
    let mut heap = BinaryHeap::<Reverse<State>>::new();
    let ending_position = (input[0].len() - 1, input.len() - 1);
    heap.push(Reverse(State {
        cost: 0,
        position: (0, 0),
        forward_steps: 0,
        previous: None,
        priority: 0,
    }));
    let mut visited = HashMap::<(usize, usize), State>::new();
    let mut possible_solutions = Vec::new();

    while let Some(current) = heap.pop() {
        let current = current.0;

        if current.position == ending_position {
            possible_solutions.push(current);
            continue;
        }

        println!("Position: ({}, {})", current.position.0, current.position.1);
        let mut neighbors = neighbors(input, &current);
        for neighbor in &mut neighbors {
            neighbor.cost = current.cost + input[neighbor.position.1][neighbor.position.0] as u64;
            neighbor.priority = neighbor.cost + (distance(neighbor.position, ending_position));
            if let Some(s) = visited.get_mut(&current.position) {
                // Already gone through this node
                if s.cost <= current.cost {
                    continue;
                }
                s.previous = Some(Rc::new(RefCell::new(neighbor.clone())));
            }

            heap.push(Reverse(neighbor.clone()));
        }
        if let Some(s) = visited.get_mut(&current.position) {
            if s.cost <= current.cost {
                continue;
            }
            s.previous = Some(Rc::new(RefCell::new(current.clone())));
            visited.insert(current.position, current);
        } else {
            visited.insert(current.position, current);
        }

        for y in 0..input.len() {
            for x in 0..input[0].len() {
                if let Some(v) = visited.get(&(x, y)) {
                    print!("{:>5}", v.cost);
                } else {
                    print!("{}", "  .  ");
                }
            }
            print!("\n");
        }
    }


    for y in 0..input.len() {
        for x in 0..input[0].len() {
            if let Some(v) = visited.get(&(x, y)) {
                print!("{:>5}", v.cost);
            } else {
                print!("{}", "  .  ");
            }
        }
        print!("\n");
    }

    println!("Priority");
    
    for y in 0..input.len() {
        for x in 0..input[0].len() {
            if let Some(v) = visited.get(&(x, y)) {
                print!("{:>5}", v.priority);
            } else {
                print!("{}", "  .  ");
            }
        }
        print!("\n");
    }
    
    println!("{:#?}", possible_solutions.iter().min().unwrap());

    None
}

pub fn part_one(_args: Args) {
    let input = input();
    shortest_path(&input);
}
pub fn part_two(_args: Args) {}
