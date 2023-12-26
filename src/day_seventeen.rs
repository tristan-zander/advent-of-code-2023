use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

use crate::Args;

const FILE_CONTENTS: &'static str = include_str!("../inputs/day_seventeen.txt");

fn input() -> Vec<Vec<u32>> {
    FILE_CONTENTS
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

#[derive(Eq, Clone, Debug)]
struct State {
    /// The cost plus the distance to the end
    // priority: u64,
    /// The actual heat lost
    cost: u64,
    /// How many steps have happened since the last turn?
    forward_steps: u8,
    /// How many steps have occurred since the start?
    // steps: u64,
    position: (usize, usize),
    previous: Option<Box<State>>,
    came_from: Direction,
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position
            && self.came_from == other.came_from
            && self.forward_steps == other.forward_steps
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cost.cmp(&other.cost))
    }
}

impl State {
    pub fn came_from_direction(position: (usize, usize), prev: &State) -> Direction {
        if prev.position.0 < position.0 {
            return Direction::West;
        } else if prev.position.0 > position.0 {
            return Direction::East;
        } else if prev.position.1 < position.1 {
            return Direction::North;
        } else {
            return Direction::South;
        }
    }
}

impl std::hash::Hash for State {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.position.hash(state);
        state.write_u8(0);
        self.came_from.hash(state);
        state.write_u8(0);
        self.forward_steps.hash(state);
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost)
    }
}

fn neighbors(input: &[Vec<u32>], state: &State) -> [Option<State>; 3] {
    let previous = Some(Box::new(state.clone()));
    let max_x = input[0].len() - 1;
    let max_y = input.len() - 1;

    let (diff_x, diff_y) = state.came_from.forward();

    let mut neighbors = [None, None, None];
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
            neighbors[0] = Some(State {
                position: (forward.0 as usize, forward.1 as usize),
                // priority: u64::MAX,
                cost: u64::MAX,
                forward_steps: state.forward_steps + 1,
                // previous: Some(state_ptr.clone()),
                // steps: state.steps + 1,
                came_from: state.came_from,
                previous: previous.clone(),
            });
        }
    }

    if diff_x != 0 {
        // Move up and down.
        let up = (state.position.0, state.position.1 as isize - 1);
        if up.1 <= max_y as isize && up.1 >= 0 {
            neighbors[1] = Some(State {
                position: (up.0, up.1 as usize),
                // priority: u64::MAX,
                cost: u64::MAX,
                forward_steps: 0,
                // previous: Some(state_ptr.clone()),
                // steps: state.steps + 1,
                came_from: State::came_from_direction((up.0, up.1 as usize), state),
                previous: previous.clone(),
            });
        }
        let down = (state.position.0, state.position.1 as isize + 1);
        if down.1 <= max_y as isize && down.1 >= 0 {
            let position = (down.0, down.1 as usize);
            neighbors[2] = Some(State {
                position,
                // priority: u64::MAX,
                cost: u64::MAX,
                forward_steps: 0,
                // previous: Some(state_ptr.clone()),
                // steps: state.steps + 1,
                came_from: State::came_from_direction(position, state),
                previous,
            });
        }
    } else {
        // Move left and right.
        let left = (state.position.0 as isize - 1, state.position.1);
        if left.0 >= 0 && left.0 <= max_x as isize {
            let position = (left.0 as usize, left.1);
            neighbors[1] = Some(State {
                position,
                // priority: u64::MAX,
                cost: u64::MAX,
                forward_steps: 0,
                // previous: Some(state_ptr.clone()),
                // steps: state.steps + 1,
                came_from: State::came_from_direction(position, state),
                previous: previous.clone(),
            });
        }
        let right = (state.position.0 as isize + 1, state.position.1);
        if right.0 >= 0 && right.0 <= max_x as isize {
            let position = (right.0 as usize, right.1);
            neighbors[2] = Some(State {
                position,
                // priority: u64::MAX,
                cost: u64::MAX,
                forward_steps: 0,
                // previous: Some(state_ptr.clone()),
                // steps: state.steps + 1,
                came_from: State::came_from_direction(position, state),
                previous,
            });
        }
    }

    neighbors
}

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Clone, Copy)]
enum Direction {
    North = 0,
    East,
    South,
    West,
}

impl Direction {
    fn forward(&self) -> (isize, isize) {
        match self {
            Direction::North => (0, 1),
            Direction::East => (-1, 0),
            Direction::South => (0, -1),
            Direction::West => (1, 0),
        }
    }
}

fn a_star(input: &[Vec<u32>], start: (usize, usize), end: (usize, usize)) -> State {
    let capacity = 4 * 4 * input.len() * input[0].len();
    let mut open = BinaryHeap::with_capacity(capacity * 5);
    open.push(Reverse(State {
        cost: 0 as u64,
        position: start,
        forward_steps: 0,
        // previous: None,
        // priority: 0,
        // steps: 0,
        came_from: Direction::West,
        previous: None,
    }));
    let mut closed = HashSet::<State>::with_capacity(capacity);

    while let Some(current) = open.pop() {
        let current = current.0;

        if current.position == end {
            println!("Found a solution: {}", current.cost);
            return current;
        }

        if let Some(s) = closed.get(&current) {
            // Already gone through this node
            if s.cost <= current.cost {
                continue;
            }
        }

        let neighbors = neighbors(input, &current).into_iter().filter_map(|s| s);
        for mut neighbor in neighbors {
            // neighbor.previous = Some(Rc::new(RefCell::new(current.clone())));
            neighbor.cost = current.cost + input[neighbor.position.1][neighbor.position.0] as u64;

            // println!("Did not get {:?}", neighbor);

            open.push(Reverse(neighbor.clone()));
        }

        closed.insert(current);
    }

    unreachable!()
}

fn shortest_path(input: &[Vec<u32>]) -> u64 {
    let ending_position = (input[0].len() - 1, input.len() - 1);
    let state = a_star(input, (0, 0), ending_position);

    let mut path = vec![state.position];
    let mut current = state.previous.clone();

    while let Some(prev) = current {
        path.push(prev.position);
        current = prev.previous.clone();
    }

    println!("{:#?}", path);

    for y in 0..input.len() {
        for x in 0..input[0].len() {
            if path.contains(&(x, y)) {
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
