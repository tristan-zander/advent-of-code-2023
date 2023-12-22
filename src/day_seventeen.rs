use std::{
    cell::RefCell,
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    rc::Rc,
};

use itertools::Itertools;

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
    /// How many steps have occurred since the start?
    steps: u64,
    position: (usize, usize),
    previous: Option<Rc<RefCell<State>>>,
}

impl State {
    fn came_from_direction(&self) -> Direction {
        if let Some(prev) = self.previous.as_ref() {
            // The case where this is not the first node.
            let prev = prev.borrow();
            if prev.position.0 < self.position.0 {
                return Direction::West;
            } else if prev.position.0 > self.position.0 {
                return Direction::East;
            } else if prev.position.1 < self.position.1 {
                return Direction::North;
            } else {
                return Direction::South;
            }
        } else {
            // The case where this is the first node.
            return Direction::West;
        }
    }
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
    let distance =
        (position.0.abs_diff(end_position.0) + position.1.abs_diff(end_position.1)) as u64;
    distance + (distance / 4)
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
                steps: 1,
            },
            State {
                position: (0, 1),
                priority: u64::MAX,
                cost: u64::MAX,
                forward_steps: 0,
                previous: Some(state_ptr),
                steps: 1,
            },
        ];
    }

    let max_x = input[0].len() - 1;
    let max_y = input.len() - 1;

    let state = state_ptr.borrow();
    let prev = state.previous.clone().unwrap();
    let prev = prev.borrow();
    let (diff_x, diff_y) = (
        state.position.0 as isize - prev.position.0 as isize,
        state.position.1 as isize - prev.position.1 as isize,
    );

    let mut neighbors = vec![];
    if state.forward_steps < 2 {
        // Move forward
        let forward = (
            state.position.0 as isize + diff_x,
            state.position.1 as isize + diff_y,
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
                steps: state.steps + 1,
            });
        }
    }

    if diff_x != 0 {
        // Move up and down.
        let up = (state.position.0, state.position.1 as isize - 1);
        if up.1 <= max_y as isize && up.1 >= 0 {
            neighbors.push(State {
                position: (up.0, up.1 as usize),
                priority: u64::MAX,
                cost: u64::MAX,
                forward_steps: 0,
                previous: Some(state_ptr.clone()),
                steps: state.steps + 1,
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
                steps: state.steps + 1,
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
                steps: state.steps + 1,
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
                steps: state.steps + 1,
            });
        }
    }

    neighbors
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Direction {
    North = 0,
    East,
    South,
    West,
}

fn a_star(input: &[Vec<u32>], start: (usize, usize), end: (usize, usize)) -> Option<State> {
    let mut open = BinaryHeap::<Reverse<State>>::new();
    open.push(Reverse(State {
        cost: input[start.1][start.0] as u64,
        position: start,
        forward_steps: 0,
        previous: None,
        priority: 0,
        steps: 0,
    }));
    let mut closed = HashMap::<(Direction, (usize, usize)), State>::new();
    let mut possible_solution: Option<State> = None;

    while let Some(current) = open.pop() {
        let current = current.0;

        let mut neighbors = neighbors(input, &current);
        for neighbor in &mut neighbors {
            neighbor.previous = Some(Rc::new(RefCell::new(current.clone())));
            neighbor.cost = current.cost + input[neighbor.position.1][neighbor.position.0] as u64;
            neighbor.priority = neighbor.cost + distance(neighbor.position, end);

            if possible_solution
                .as_ref()
                .map(|s| s.cost < neighbor.steps || s.cost < neighbor.cost)
                .unwrap_or(false)
            {
                // We've definitely found a solution that is better than this one.
                continue;
            }

            if neighbor.position == end {
                if possible_solution
                    .as_ref()
                    .map(|s| s.cost > neighbor.cost)
                    .unwrap_or(true)
                {
                    possible_solution = Some(neighbor.clone());
                }
                continue;
            }

            if let Some(s) = closed.get(&(neighbor.came_from_direction(), neighbor.position)) {
                // Already gone through this node
                if s.priority < neighbor.priority {
                    continue;
                }
            }

            open.push(Reverse(neighbor.clone()));
        }

        closed.insert((current.came_from_direction(), current.position), current);
    }

    possible_solution
}

fn shortest_path(input: &[Vec<u32>]) -> u64 {
    let ending_position = (input[0].len() - 1, input.len() - 1);
    let state = a_star(input, (0, 0), ending_position).unwrap();

    let mut path = vec![state.position];
    let mut current = state.previous.clone();

    while let Some(prev) = current {
        path.push(prev.borrow().position);
        current = prev.borrow().previous.clone();
    }

    println!("{:#?}", path);

    for y in 0..input.len() {
        for x in 0..input[0].len() {
            if path.iter().contains(&(x, y)) {
                print!(" = ");
            } else {
                print!(" . ");
            }
        }
        print!("\n");
    }

    state.cost
}

pub fn part_one(_args: Args) {
    let input = input();
    println!("Cost: {}", shortest_path(&input));
}
pub fn part_two(_args: Args) {}
