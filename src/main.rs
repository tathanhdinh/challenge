// #![feature(alloc_system)]
// extern crate alloc_system;
#[macro_use]
extern crate lazy_static;
extern crate nix;
extern crate petgraph;

use nix::*;
use std::sync::*;
use petgraph::graph::*;

lazy_static! {
  static ref RANDOM_SEED: Mutex<u32> = Mutex::new(unistd::getpid() as u32);
}

const WIDTH: u32 = 20;
const HEIGHT: u32 = 7;

// xorshift algorithm
fn next_random_number() -> u32 {
  let mut current_seed = *RANDOM_SEED.lock().unwrap();
  current_seed ^= current_seed << 13;
  current_seed ^= current_seed >> 17;
  current_seed ^= current_seed << 5;
  *RANDOM_SEED.lock().unwrap() = current_seed;
  current_seed
}

struct Cell {
  x: u32,
  y: u32,
}

enum Direction {
  North, // up
  South, // down
  Est,   // right
  West,  // left
}

fn draw(maze: &DiGraph<Cell, Direction>) {
  // for _ in 0..WIDTH {
  //   print!("+--");
  // }
  // println!("+");

  // for _ in 0..HEIGHT {
  //   for _ in 0..WIDTH {
  //     print!("|  ");
  //   }
  //   println!("|");

  //   for _ in 0..WIDTH {
  //     print!("+--");
  //   }
  //   println!("+");
  // }

  let mut cell_indexes = [[NodeIndex::from(0u32); WIDTH]; HEIGHT];
  let mut node_indexes = maze.node_indices();
  for _y in 0..HEIGHT {
    for _x in 0..WIDTH {
      let predicate_xy = |ix: &NodeIndex| {
        let cell = maze.node_weight(*ix).unwrap();
        _x == cell.x && _y == cell.y
      };
      let node_xy_index = node_indexes.find(predicate_xy).unwrap();
      cell_indexes[_y][_x] = node_xy_index;
    }
  }

  let mut maze_visual_form = String::from("+");
  for _ in 0..WIDTH {
    maze_visual_form.push_str("--+");
  }

  let (x_bound, y_bound) = (WIDTH - 1, HEIGHT - 1);

  for _y in 0..HEIGHT {
    let mut row_y_body = String::from("|");
    let mut row_y_lbound = String::from("+");
    
    for _x in 0..WIDTH {
      row_y_body.push_str("  ");

      if _x < x_bound {
        match maze.find_edge(cell_indexes[_y][_x], cell_indexes[_y][_x + 1]) {
          Some(_) => row_y_body.push(' '),
          None => row_y_body.push('|'),
        }
      } else {
        row_y_body.push('|');
      }

      if _y < y_bound {
        match maze.find_edge(cell_indexes[_y][_x], cell_indexes[_y + 1][_x]) {
          Some(_) => row_y_lbound.push_str("--"),
          None => row_y_lbound.push_str("  "),
        }
      } else {
        row_y_lbound.push_str("--");
      }    
      
      row_y_lbound.push('+');
    }

    maze_visual_form.push('\n');
    maze_visual_form.push_str(row_y_body.as_str());
    maze_visual_form.push('\n');
    maze_visual_form.push_str(row_y_lbound.as_str());
  }

  println!("{}", maze_visual_form);
}

// fn initialize(full_grid: &mut DiGraph<Cell, Direction>) {
fn initialize_grid() -> DiGraph<Cell, Direction> {
  let mut full_grid = petgraph::graph::DiGraph::<Cell, Direction>::new();
  let mut cell_indexes = [[NodeIndex::from(0u32); WIDTH]; HEIGHT];
  for _y in 0..HEIGHT {
    for _x in 0..WIDTH {
      cell_indexes[_y][_x] = full_grid.add_node(Cell { x: _x, y: _y });
    }
  }

  for _y in 0..HEIGHT {
    for _x in 0..WIDTH {
      let (x_bound, y_bound) = (WIDTH - 1, HEIGHT - 1);

      if _x > 0 {
        full_grid.add_edge(cell_indexes[_y][_x], cell_indexes[_y][_x - 1], Direction::West);  // to left cell
      }

      if _x < x_bound {
        full_grid.add_edge(cell_indexes[_y][_x], cell_indexes[_y][_x + 1], Direction::Est);   // to right cell
      }

      if _y > 0 {
        full_grid.add_edge(cell_indexes[_y][_x], cell_indexes[_y - 1][_x], Direction::North); // to up cell
      }

      if _y < y_bound {
        full_grid.add_edge(cell_indexes[_y][_x], cell_indexes[_y + 1][_x], Direction::South); // to down cell
      }
    }
  }

  full_grid
}

fn generate_maze(background_grid: &DiGraph<Cell, Direction>) -> DiGraph<Cell, Direction> {
  // full_grid
  let mut maze = petgraph::graph::DiGraph::<Cell, Direction>::new();
  maze
}

fn main() {
  // println!("Hello, world!");
  // use pid as random seed
  // let pid = unistd::getpid();
  // println!("pid = {}", pid);
  // let mut maze = petgraph::graph::DiGraph::<Cell, Direction>::new();
  // initialize(&mut maze);
  let full_grid = initialize_grid();
  let maze = generate_maze(&full_grid);
  draw(&maze);
  // draw_maze(3, 4);
  // println!("seed = {}", next_random_number());
  // println!("seed = {}", next_random_number());
}
