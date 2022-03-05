use std::io::{stdin,stdout};

use rand;
use rand::prelude::SliceRandom;
use rand::seq::IteratorRandom;

const HEIGHT : usize = 5;
const WIDTH  : usize = 10;

const START_X : usize = 0;
const START_Y : usize = 0;


#[derive(Copy, Clone, Debug, PartialEq)]
enum Cell {
    Snail,
    Unvisited,
    Slimed,
    Yarn,
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
            &Self::Yarn => '/',
        }
    }
}


#[derive(Debug, Copy, Clone)]
enum Direction {
    LEFT,
    UP,
    RIGHT,
    DOWN,
}


#[derive(Debug, Copy, Clone, PartialEq)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn left(self: &Self) -> Self {
        Self {x: self.x - 1, y: self.y}
    }
}

impl Default for Position {
    fn default() -> Self { Self { x: 0, y: 0 } }
}


fn find_neighbors(grid: &[[Cell; HEIGHT]; WIDTH], coords: &Position) -> Vec<(Position, Direction)> {
    let mut unvisited_neighbors = vec![];

    let x = coords.x;
    let y = coords.y;

    // check left
    if x > 0 && grid[x - 1][y] == Cell::Unvisited {
        unvisited_neighbors.push((Position {x: x - 1, y: y}, Direction::LEFT));
    }

    // check up
    if y > 0 && grid[x][y - 1] == Cell::Unvisited {
        unvisited_neighbors.push((Position {x: x, y: y - 1}, Direction::UP));
    }
    
    // check right
    if x < WIDTH - 1 && grid[x + 1][y] == Cell::Unvisited {
        unvisited_neighbors.push((Position {x: x + 1, y: y}, Direction::RIGHT));
    }

    // check down
    if y < HEIGHT - 1 && grid[x][y + 1] == Cell::Unvisited {
        unvisited_neighbors.push((Position {x: x, y: y + 1}, Direction::DOWN));
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

fn set_snail(grid: &mut [[Cell; HEIGHT]; WIDTH], snail_coords: &Position) {
    grid[snail_coords.x][snail_coords.y] = Cell::Snail;
}

fn set_yarn(grid: &mut [[Cell; HEIGHT]; WIDTH], yarn_pos: &Position) {
    grid[yarn_pos.x][yarn_pos.y] = Cell::Yarn;
}

fn unset_yarn(grid: &mut [[Cell; HEIGHT]; WIDTH], old_yarn_pos: &Position) {
    grid[old_yarn_pos.x][old_yarn_pos.y] = Cell::Slimed;
}

fn move_forward(grid: &mut [[Cell; HEIGHT]; WIDTH], snail_position: &mut Position, yarn_trail: &mut Vec<Position>, next_step: Position) {
    yarn_trail.push(snail_position.clone());
    set_yarn(grid, snail_position);
    *snail_position = next_step;
    set_snail(grid, snail_position);
}

fn move_back_along_yarn(grid: &mut [[Cell; HEIGHT]; WIDTH], snail_position: &mut Position, yarn_trail: &mut Vec<Position>) {
    let next_step = match yarn_trail.pop() {
        Some(it) => it,
        _ => return,
    };
    grid[snail_position.x][snail_position.y] = Cell::Slimed;
    *snail_position = next_step;
    grid[snail_position.x][snail_position.y] = Cell::Snail;
}


fn main() {
    println!("Hello, world!");
    // initialize grid
    let mut grid = [[Cell::Unvisited; HEIGHT]; WIDTH];
    // starting point
    grid[START_X][START_Y] = Cell::Snail;
    
    // let mut snail_coords = (START_X, START_Y);
    let mut snail_coords = Position {x: START_X, y: START_Y};

    let mut yarn_trail: Vec<Position> = vec![];

    // clear screen
    print!("\x1B[2J\x1B[1;1H");

    loop {
        // set cursor to first row/column
        print!("\x1B[2J\x1B[1;1H");
        // draw grid
        display_grid(&grid);
        
        //wait for input
        let mut s = String::new();
        println!("press enter to continue");
        let _ = stdin().read_line(&mut s);
        
        // find unslimed neighbors
        let neighbors = find_neighbors(&grid, &snail_coords);

        // mark old spot slimed
        grid[snail_coords.x][snail_coords.y] = Cell::Slimed;

        // needed for random choose
        let mut rng = rand::thread_rng();
        if let Some(next_step) = neighbors.choose(&mut rng) {
            // move snail forward
            println!("going {:?}", next_step.1);
            move_forward(&mut grid, &mut snail_coords, &mut yarn_trail, next_step.0);
        } else {
            // move snail back
            if !yarn_trail.is_empty() {
                move_back_along_yarn(&mut grid, &mut snail_coords, &mut yarn_trail);
                println!("backtracking");
            } else {
                // if no yarn left we are done
                break;
            }
        }
    }
}
