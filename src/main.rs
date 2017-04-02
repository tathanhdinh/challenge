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
  static ref RANDOM_SEED: Mutex<usize> = Mutex::new(unistd::getpid() as usize);
}

const WIDTH: usize = 20;
const HEIGHT: usize = 7;

// trait GenerateNextRandomNumber<T> {
//   fn next_random_number() -> T;
// }

// impl GenerateNextRandomNumber<u32> for u32 {
//   fn next_random_number() -> u32 {
//     let mut current_seed = *RANDOM_SEED.lock().unwrap() as u32;
//     current_seed ^= current_seed << 13;
//     current_seed ^= current_seed >> 17;
//     current_seed ^= current_seed << 5;
//     *RANDOM_SEED.lock().unwrap() = current_seed as usize;
//     current_seed
//   }
// }

// impl GenerateNextRandomNumber<u64> for u64 {
//   fn next_random_number() -> u64 {
//     let mut current_seed = *RANDOM_SEED.lock().unwrap() as u64;
//     current_seed ^= current_seed << 13;
//     current_seed ^= current_seed >> 47;
//     current_seed ^= current_seed << 23;
//     *RANDOM_SEED.lock().unwrap() = current_seed as usize;
//     current_seed
//   }
// }

fn next_random_number() -> usize {
  let mut current_seed = *RANDOM_SEED.lock().unwrap();
  let usize_length = std::mem::size_of::<usize>();
  if std::mem::size_of::<u32>() == usize_length {
    current_seed ^= current_seed << 13;
    current_seed ^= current_seed >> 17;
    current_seed ^= current_seed << 5;
    *RANDOM_SEED.lock().unwrap() = current_seed as usize;
  } else if std::mem::size_of::<u64>() == usize_length {
    current_seed ^= current_seed << 13;
    current_seed ^= current_seed >> 47;
    current_seed ^= current_seed << 23;
  } else {
    unreachable!();
  }

  *RANDOM_SEED.lock().unwrap() = current_seed;
  current_seed
}

// xorshift algorithm
// fn next_random_number() -> u32 {
//   let mut current_seed = *RANDOM_SEED.lock().unwrap();
//   current_seed ^= current_seed << 13;
//   current_seed ^= current_seed >> 17;
//   current_seed ^= current_seed << 5;
//   *RANDOM_SEED.lock().unwrap() = current_seed;
//   current_seed as u32
// }

struct Cell {
  x: usize,
  y: usize,
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
        full_grid.add_edge(cell_indexes[_y][_x],
                           cell_indexes[_y][_x - 1],
                           Direction::West);  // to left cell
      }

      if _x < x_bound {
        full_grid.add_edge(cell_indexes[_y][_x],
                           cell_indexes[_y][_x + 1],
                           Direction::Est);   // to right cell
      }

      if _y > 0 {
        full_grid.add_edge(cell_indexes[_y][_x],
                           cell_indexes[_y - 1][_x],
                           Direction::North); // to up cell
      }

      if _y < y_bound {
        full_grid.add_edge(cell_indexes[_y][_x],
                           cell_indexes[_y + 1][_x],
                           Direction::South); // to down cell
      }
    }
  }

  full_grid
}

// on the left
fn western_cell(x: usize, y: usize,
             cell_matrix: &[[NodeIndex; WIDTH]; HEIGHT]) -> Option<NodeIndex> {
  if x > 0 {
    Some(cell_matrix[y][x - 1])
  } else {
    None
  }
}

// on the right
fn eastern_cell(x: usize, y: usize,
                cell_matrix: &[[NodeIndex; WIDTH]; HEIGHT]) -> Option<NodeIndex> {
  if x < WIDTH - 1 {
    Some(cell_matrix[y][x + 1])
  } else {
    None
  }
}

// upper
fn northern_cell(x: usize, y: usize,
                 cell_matrix: &[[NodeIndex; WIDTH]; HEIGHT]) -> Option<NodeIndex> {
  if y > 0 {
    Some(cell_matrix[x][y - 1])
  } else {
    None
  }
}

// lower
fn southern_cell(x: usize, y: usize,
                 cell_matrix: &[[NodeIndex; WIDTH]; HEIGHT]) -> Option<NodeIndex> {
  if y < HEIGHT - 1 {
    Some(cell_matrix[x][y + 1])
  } else {
    None
  }
}

fn generate_maze(background_grid: &DiGraph<Cell, Direction>) -> DiGraph<Cell, Direction> {
  // full_grid
  let mut maze = petgraph::graph::DiGraph::<Cell, Direction>::new();
  let mut maze_cell_indexes = [[NodeIndex::from(0u32); WIDTH]; HEIGHT];
  for _y in 0..HEIGHT {
    for _x in 0..WIDTH {
      maze_cell_indexes[_y][_x] = maze.add_node(Cell { x: _x, y: _y });
    }
  }

  let x_init = next_random_number() % WIDTH;
  let y_init = next_random_number() % HEIGHT;
  // let (x_bound, y_bound) = (WIDTH - 1, HEIGHT - 1);
  // let mut examining_edges: Vec<EdgeIndex> = Vec::new();
  let mut examining_edges = Vec::new();

  let current_cell_index = maze_cell_indexes[y_init][x_init];
  // if x_init > 0 {
  //   examining_edges.push(maze.add_edge(current_cell_index,
  //                                      maze_cell_indexes[y_init][x_init - 1],
  //                                      Direction::West)); // to the left cell
  //   maze.add_edge(maze_cell_indexes[y_init][x_init - 1],
  //                 current_cell_index,
  //                 Direction::Est); // backward edge (don't need to add in the vector)
  // }
  match western_cell(x_init, y_init, &maze_cell_indexes) {
    Some(cell_index) => {
      examining_edges.push(maze.add_edge(current_cell_index, cell_index, Direction::West));
      maze.add_edge(cell_index, current_cell_index, Direction::Est);
    },
    None => (),
  }

  match eastern_cell(x_init, y_init, &maze_cell_indexes) {
    Some(cell_index) => {
      examining_edges.push(maze.add_edge(current_cell_index, cell_index, Direction::Est));
      maze.add_edge(cell_index, current_cell_index, Direction::West);
    },
    None => (),
  }

  match northern_cell(x_init, y_init, &maze_cell_indexes) {
    Some(cell_index) => {
      examining_edges.push(maze.add_edge(current_cell_index, cell_index, Direction::North));
      maze.add_edge(cell_index, current_cell_index, Direction::South);
    },
    None => (),
  }

  match southern_cell(x_init, y_init, &maze_cell_indexes) {
    Some(cell_index) => {
      examining_edges.push(maze.add_edge(current_cell_index, cell_index, Direction::South));
      maze.add_edge(cell_index, current_cell_index, Direction::North);
    },
    None => (),
  }

  while !examining_edges.is_empty() {
    let examining_edge_num = examining_edges.len();
    let random_edge_index = next_random_number() % examining_edge_num;
  }

  // if x_init < x_bound {
  //   examining_edges.push(maze.add_edge(current_cell_index,
  //                                      maze_cell_indexes[y_init][x_init + 1],
  //                                      Direction::Est)); // to the right cell
  //   maze.add_edge(maze_cell_indexes[y_init][x_init + 1],
  //                 current_cell_index,
  //                 Direction::West); // backward edge
  // }

  // if y_init > 0 {
  //   examining_edges.push(maze.add_edge(current_cell_index,
  //                                      maze_cell_indexes[y_init - 1][x_init],
  //                                      Direction::North)); // to the upper cell
  //   maze.add_edge(maze_cell_indexes[y_init - 1][x_init],
  //                 current_cell_index, Direction::South); // backward edge
  // }

  // if y_init < y_bound {
  //   examining_edges.push(maze.add_edge(current_cell_index,
  //                                      maze_cell_indexes[y_init + 1][x_init],
  //                                      Direction::South));
  // }

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
  let ran = next_random_number();
  // println!("seed = {}", next_random_number<u32>());
  // println!("seed = {}", next_random_number());
  println!("seed = {}", ran);
}
