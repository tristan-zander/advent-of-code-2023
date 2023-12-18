use std::fmt::Display;

use itertools::Itertools;

use crate::Args;

const FILE_CONTENTS: &'static str = include_str!("../inputs/day_sixteen.txt");

#[derive(Default, Debug, Clone)]
struct Tile {
    pub typ: TileType,
    pub energized: bool,
}

impl From<u8> for Tile {
    fn from(value: u8) -> Self {
        Self {
            typ: TileType::from(value),
            energized: false,
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.energized {
            true => write!(f, "#"),
            false => write!(f, "."),
        }
    }
}

impl Tile {
    fn visit(&mut self, from: Direction) -> Action {
        self.energized = true;

        match &mut self.typ {
            TileType::Vertical {
                bottom_visited,
                top_visited,
                left_split,
                right_split,
            } => match from {
                Direction::North => {
                    if *top_visited {
                        return Action::Stop;
                    }

                    *top_visited = true;
                    Action::Direction(Direction::South)
                }
                Direction::South => {
                    if *bottom_visited {
                        return Action::Stop;
                    }
                    *bottom_visited = true;
                    Action::Direction(Direction::North)
                }
                Direction::East => {
                    if *right_split {
                        return Action::Stop;
                    }
                    *right_split = true;
                    Action::Split(Direction::North, Direction::South)
                }
                Direction::West => {
                    if *left_split {
                        return Action::Stop;
                    }
                    *left_split = true;
                    Action::Split(Direction::North, Direction::South)
                }
            },
            TileType::Horizontal {
                left_visited,
                right_visited,
                bottom_split,
                top_split,
            } => {
                // do the same thing as vertical, but for hoizontal
                match from {
                    Direction::North => {
                        if *top_split {
                            return Action::Stop;
                        }
                        *top_split = true;
                        Action::Split(Direction::East, Direction::West)
                    }
                    Direction::South => {
                        if *bottom_split {
                            return Action::Stop;
                        }
                        *bottom_split = true;
                        Action::Split(Direction::East, Direction::West)
                    }
                    Direction::East => {
                        if *right_visited {
                            return Action::Stop;
                        }
                        *right_visited = true;
                        Action::Direction(Direction::West)
                    }
                    Direction::West => {
                        if *left_visited {
                            return Action::Stop;
                        }
                        *left_visited = true;
                        Action::Direction(Direction::East)
                    }
                }
            }
            TileType::ReflectForward {
                reflect_top,
                reflect_right,
                reflect_bottom,
                reflect_left,
            } => match from {
                Direction::North => {
                    if *reflect_top {
                        return Action::Stop;
                    }
                    *reflect_top = true;
                    Action::Direction(Direction::West)
                }
                Direction::South => {
                    if *reflect_bottom {
                        return Action::Stop;
                    }
                    *reflect_bottom = true;
                    Action::Direction(Direction::East)
                }
                Direction::East => {
                    if *reflect_right {
                        return Action::Stop;
                    }
                    *reflect_right = true;
                    Action::Direction(Direction::South)
                }
                Direction::West => {
                    if *reflect_left {
                        return Action::Stop;
                    }
                    *reflect_left = true;
                    Action::Direction(Direction::North)
                }
            },
            TileType::ReflectBack {
                reflect_top,
                reflect_right,
                reflect_bottom,
                reflect_left,
            } => match from {
                Direction::North => {
                    if *reflect_top {
                        return Action::Stop;
                    }
                    *reflect_top = true;
                    Action::Direction(Direction::East)
                }
                Direction::South => {
                    if *reflect_bottom {
                        return Action::Stop;
                    }
                    *reflect_bottom = true;
                    Action::Direction(Direction::West)
                }
                Direction::East => {
                    if *reflect_right {
                        return Action::Stop;
                    }
                    *reflect_right = true;
                    Action::Direction(Direction::North)
                }
                Direction::West => {
                    if *reflect_left {
                        return Action::Stop;
                    }
                    *reflect_left = true;
                    Action::Direction(Direction::South)
                }
            },
            TileType::Empty => Action::Direction(from.opposite()),
        }
    }
}

enum Action {
    Stop,
    Split(Direction, Direction),
    Direction(Direction),
}

#[derive(Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn opposite(&self) -> Self {
        use Direction::*;
        match self {
            North => South,
            South => North,
            East => West,
            West => East,
        }
    }
}

#[derive(Default, Debug, Clone)]
enum TileType {
    Vertical {
        bottom_visited: bool,
        top_visited: bool,
        left_split: bool,
        right_split: bool,
    },
    Horizontal {
        left_visited: bool,
        right_visited: bool,
        bottom_split: bool,
        top_split: bool,
    },
    ReflectForward {
        reflect_top: bool,
        reflect_right: bool,
        reflect_bottom: bool,
        reflect_left: bool,
    },
    ReflectBack {
        reflect_top: bool,
        reflect_right: bool,
        reflect_bottom: bool,
        reflect_left: bool,
    },
    #[default]
    Empty,
}

impl From<u8> for TileType {
    fn from(value: u8) -> Self {
        use TileType::*;
        match value {
            b'.' => Empty,
            b'/' => ReflectForward {
                reflect_top: false,
                reflect_right: false,
                reflect_bottom: false,
                reflect_left: false,
            },
            b'\\' => ReflectBack {
                reflect_top: false,
                reflect_right: false,
                reflect_bottom: false,
                reflect_left: false,
            },
            b'-' => Horizontal {
                left_visited: false,
                right_visited: false,
                bottom_split: false,
                top_split: false,
            },
            b'|' => Vertical {
                bottom_visited: false,
                top_visited: false,
                left_split: false,
                right_split: false,
            },
            _ => unreachable!(),
        }
    }
}

fn input() -> Vec<Vec<Tile>> {
    FILE_CONTENTS
        .lines()
        .map(|line| {
            line.bytes()
                .map(|byte| Tile::from(byte))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn walk(input: &mut [Vec<Tile>], mut current: (isize, isize), mut heading_toward: Direction) {
    let x_len = input.len();
    let y_len = input[0].len();
    loop {
        let (x, y) = current;
        if x < 0 || x >= x_len as isize || y < 0 || y >= y_len as isize {
            println!("Stopped at edge: {:?}", current);
            return;
        }
        let tile = &mut input[y as usize][x as usize];
        let action = tile.visit(heading_toward.opposite());

        match action {
            Action::Stop => {
                println!("Stopped at {:?}", current);
                break;
            }
            Action::Split(left, right) => {
                println!("Splitting at {:?}", current);
                walk(input, (x, y), left);
                walk(input, (x, y), right);
                return;
            }
            Action::Direction(direction) => {
                println!("Moving {:?} from {:?}", direction, current);
                match direction {
                    Direction::North => current.1 -= 1,
                    Direction::South => current.1 += 1,
                    Direction::East => current.0 += 1,
                    Direction::West => current.0 -= 1,
                }
                heading_toward = direction;
            }
        }
    }
}

fn debug(input: &[Vec<Tile>]) -> String {
    input
        .iter()
        .map(|row| row.iter().map(|t| format!("{}", t)).join(""))
        .join("\n")
}

fn count_tiles(input: &[Vec<Tile>]) -> usize {
    input
        .iter()
        .map(|r| r.iter().filter(|t| t.energized).count())
        .sum::<usize>()
}

pub fn part_one(_args: Args) {
    let mut input = input();

    let current = (0, 0);
    let heading_toward = Direction::East;
    walk(&mut input, current, heading_toward);
    println!("{}", debug(&input));

    let count = count_tiles(&input);
    println!("Count: {}", count);
}

pub fn part_two(_args: Args) {
    let input = input();
    let x_len = input.len();
    let y_len = input[0].len();

    let mut largest = 0;

    for x in 0..x_len {
        {
            let mut input = input.clone();
            walk(&mut input, (x as isize, 0), Direction::South);
            largest = std::cmp::max(largest, count_tiles(&input));
        }
        {
            let mut input = input.clone();
            walk(
                &mut input,
                (x as isize, (y_len - 1) as isize),
                Direction::North,
            );
            largest = std::cmp::max(largest, count_tiles(&input));
        }
    }

    for y in 0..y_len {
        {
            let mut input = input.clone();
            walk(&mut input, (0 as isize, y as isize), Direction::East);
            largest = std::cmp::max(largest, count_tiles(&input));
        }
        {
            let mut input = input.clone();
            walk(
                &mut input,
                ((x_len - 1) as isize, y as isize),
                Direction::West,
            );
            largest = std::cmp::max(largest, count_tiles(&input));
        }
    }

    println!(
        "Most energized layout causes {} tiles to be energized",
        largest
    );
}
