// main.rs

use std::io::stdin;

use rand;
use rand::prelude::SliceRandom;

const HEIGHT : usize = 10;
const WIDTH  : usize = 20;

const START_X : usize = 0;
const START_Y : usize = 0;

const UP_DOWN_CHAR : char    = '│';  // Character 9474 is '│'
const LEFT_RIGHT_CHAR : char = '─';  // Character 9472 is '─'
const DOWN_RIGHT_CHAR : char = '┌';  // Character 9484 is '┌'
const DOWN_LEFT_CHAR : char  = '┐';  // Character 9488 is '┐'
const UP_RIGHT_CHAR : char   = '└';  // Character 9492 is '└'
const UP_LEFT_CHAR : char    = '┘';  // Character 9496 is '┘'


#[derive(Copy, Clone, Debug, PartialEq)]
enum Cell {
    Snail,
    Unvisited,
    Slimed,
    Yarn(char),
}

impl Default for Cell {
    fn default() -> Self { Self::Unvisited }
}

impl Cell {
    fn to_char(self: &Self) -> char {
        match self {
            &Self::Snail => '@',
            &Self::Unvisited => '.',
            &Self::Slimed => '&',
            &Self::Yarn(c) => c,
        }
    }
}


#[derive(Debug, Copy, Clone, PartialEq)]
enum Direction {
    LEFT,
    UP,
    RIGHT,
    DOWN,
}

impl Direction {
    fn opposite(self: &Self) -> Self {
        match self {
            Self::LEFT => Self::RIGHT,
            Self::RIGHT => Self::LEFT,
            Self::UP => Self::DOWN,
            Self::DOWN => Self::UP,
        }
    }
}


#[derive(Debug, Copy, Clone, PartialEq)]
struct Position {
    x: usize,
    y: usize,
}

#[allow(dead_code)]
impl Position {
    fn left(self: &Self) -> Self {
        Self {x: self.x - 1, y: self.y}
    }
    
    fn up(self: &Self) -> Self {
        Self {x: self.x, y: self.y - 1}
    }
    
    fn right(self: &Self) -> Self {
        Self {x: self.x + 1, y: self.y}
    }

    fn down(self: &Self) -> Self {
        Self {x: self.x, y: self.y + 1}
    }

    fn relative_to(self: &Self, other: &Self) -> Direction {
        let delta_y: isize = other.y as isize - self.y as isize;
        let delta_x: isize = other.x as isize - self.x as isize;
        match (delta_x, delta_y) {
            (0, 1) => Direction::DOWN,
            (0, -1) => Direction::UP,
            (-1, 0) => Direction::LEFT,
            (1, 0) => Direction::RIGHT,
            (_, _) => panic!("illegal move? delta_x: {:?} delta_y: {:?}", delta_x, delta_y),
        }
    }

    fn to_the(self: &Self, d: Direction) -> Self {
        match d {
            Direction::RIGHT => Self { x: self.x + 1, y: self.y },
            Direction::LEFT => Self { x: self.x - 1, y: self.y },
            Direction::DOWN => Self { x: self.x, y: self.y + 1 },
            Direction::UP => Self { x: self.x, y: self.y - 1 },
        }
    }
}

impl Default for Position {
    fn default() -> Self { Self { x: 0, y: 0 } }
}

// struct Move {
//     direction: Direction,
//     to: Position
// }


fn find_neighbors(grid: &[[Cell; HEIGHT]; WIDTH], coords: &Position) -> Vec<Position> {
    let mut unvisited_neighbors = vec![];

    let x = coords.x;
    let y = coords.y;

    // check left
    if x > 0 && grid[x - 1][y] == Cell::Unvisited {
        unvisited_neighbors.push(Position {x: x - 1, y: y});
    }

    // check up
    if y > 0 && grid[x][y - 1] == Cell::Unvisited {
        unvisited_neighbors.push(Position {x: x, y: y - 1});
    }
    
    // check right
    if x < WIDTH - 1 && grid[x + 1][y] == Cell::Unvisited {
        unvisited_neighbors.push(Position {x: x + 1, y: y});
    }

    // check down
    if y < HEIGHT - 1 && grid[x][y + 1] == Cell::Unvisited {
        unvisited_neighbors.push(Position {x: x, y: y + 1});
    }

    unvisited_neighbors
}


fn display_grid(grid: &[[Cell; HEIGHT]; WIDTH]) {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            print!("{0}", grid[x][y].to_char())
        }
        print!("\n")
    }
    print!("\n")
}


fn find_yarn_char(prev_pos: &Position, curr_pos: &Position, next_pos: &Position) -> Result<char, String> {
    let went: Direction = prev_pos.relative_to(curr_pos);
    let going: Direction = curr_pos.relative_to(next_pos);
    
    println!("went: {:?} going {:?}", went, going);
    
    match went {
        Direction::DOWN => {
            match going {
                Direction::DOWN => Ok(UP_DOWN_CHAR),
                Direction::RIGHT => Ok(UP_RIGHT_CHAR),
                Direction::LEFT => Ok(UP_LEFT_CHAR),
                Direction::UP => Err("going backward".to_string()),
            }
        },
        Direction::UP => {
            match going {
                Direction::UP => Ok(UP_DOWN_CHAR),
                Direction::RIGHT => Ok(DOWN_RIGHT_CHAR),
                Direction::LEFT => Ok(DOWN_LEFT_CHAR),
                _ => Err("going backward".to_string()),
            }
        },
        Direction::LEFT => {
            match going {
                Direction::LEFT => Ok(LEFT_RIGHT_CHAR),
                Direction::UP => Ok(UP_RIGHT_CHAR),
                Direction::DOWN => Ok(DOWN_RIGHT_CHAR),
                _=> Err("going backward".to_string()),
            }
        },
        Direction::RIGHT => {
            match going {
                Direction::RIGHT => Ok(LEFT_RIGHT_CHAR),
                Direction::UP => Ok(UP_LEFT_CHAR),
                Direction::DOWN => Ok(DOWN_LEFT_CHAR),
                _ => Err("going backward".to_string()),
            }
        },
    }
}

fn move_forward(grid: &mut [[Cell; HEIGHT]; WIDTH], snail_position: &mut Position, yarn_trail: &mut Vec<(Position, char)>, next_step: Position) {

    // println!("snail_position: {:?} next_step: {:?}", snail_position, next_step);

    if let Some(last_yarn) = yarn_trail.last() {
        let back_step = last_yarn.0;
        println!("back: {:?} snail: {:?} next: {:?}", &back_step, &snail_position, &next_step);
        if let Ok(c) = find_yarn_char(&back_step, &snail_position, &next_step) {
            // yarn on current position
            yarn_trail.push((snail_position.clone(), c));
            grid[snail_position.x][snail_position.y] = Cell::Yarn(c);
            // move snail to next position
            *snail_position = next_step;
            grid[snail_position.x][snail_position.y] = Cell::Snail;
        } else {
            panic!("going and next_step.1 are not equallllllllllllllllll");
        }
    } else {
        let d = snail_position.relative_to(&next_step);
        // println!("first move probably, going: {:?}", d);
        match d {
            Direction::LEFT | Direction::RIGHT => {
                // yarn on current position
                yarn_trail.push((snail_position.clone(), LEFT_RIGHT_CHAR));
                grid[snail_position.x][snail_position.y] = Cell::Yarn(LEFT_RIGHT_CHAR);
                // move snail to next position
                *snail_position = next_step;
                grid[snail_position.x][snail_position.y] = Cell::Snail;
            },
            Direction::UP | Direction::DOWN => {
                // yarn on current position
                yarn_trail.push((snail_position.clone(), UP_DOWN_CHAR));
                grid[snail_position.x][snail_position.y] = Cell::Yarn(UP_DOWN_CHAR);
                // move snail to next position
                *snail_position = next_step;
                grid[snail_position.x][snail_position.y] = Cell::Snail;
            }
        }
    }
}

fn move_back_along_yarn(grid: &mut [[Cell; HEIGHT]; WIDTH], snail_position: &mut Position, yarn_trail: &mut Vec<(Position, char)>) {
    let (next_pos, _) = match yarn_trail.pop() {
        Some(it) => it,
        _ => return,
    };
    grid[snail_position.x][snail_position.y] = Cell::Slimed;
    *snail_position = next_pos;
    grid[snail_position.x][snail_position.y] = Cell::Snail;
}


fn wait_for_input() {
    let mut s = String::new();
    println!("press enter to continue");
    let _ = stdin().read_line(&mut s);
}


fn main() {
    println!("Hello, world!");
    // println!("Hello, {}", UP_LEFT_CHAR);
    
    // initialize grid
    let mut grid = [[Cell::Unvisited; HEIGHT]; WIDTH];
    // starting point
    grid[START_X][START_Y] = Cell::Snail;
    
    // let mut snail_coords = (START_X, START_Y);
    let mut snail_pos = Position {x: START_X, y: START_Y};

    let mut yarn_trail: Vec<(Position, char)> = vec![];

    // // clear screen
    // print!("\x1B[2J\x1B[1;1H");

    loop {
        // set cursor to first row/column. overwriting on next display
        print!("\x1B[2J\x1B[1;1H");
        // draw grid
        display_grid(&grid);
        
        //wait for input
        wait_for_input();
        
        // find unslimed neighbors
        let neighbors = find_neighbors(&grid, &snail_pos);
        // println!("free neighbors: {:?}", neighbors);
        
        // mark old spot slimed
        grid[snail_pos.x][snail_pos.y] = Cell::Slimed;

        // needed for random choose
        let mut rng = rand::thread_rng();
        if let Some(next_step) = neighbors.choose(&mut rng) {
            // move snail forward
            // println!("going {:?}", next_step);
            move_forward(&mut grid, &mut snail_pos, &mut yarn_trail, *next_step);
        } else {
            // move snail back
            if !yarn_trail.is_empty() {
                move_back_along_yarn(&mut grid, &mut snail_pos, &mut yarn_trail);
                // println!("backtracking");
            } else {
                // if no yarn left we are done
                break;
            }
        }
    }
}
