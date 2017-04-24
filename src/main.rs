// #![feature(alloc_system)]
// extern crate alloc_system;
#[macro_use]
extern crate lazy_static;
extern crate nix;
extern crate petgraph;
// extern crate nadeko;

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
//     current_seed ^= current_sRamblr: Making Reassembly Great Againeed >> 17;
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

struct Cell {
  pub x: usize,
  pub y: usize,
}

#[derive(Copy, Clone)]
enum Direction {
  North = 0x0, // up
  South = 0x1, // down
  Est = 0x2,   // right
  West = 0x3,  // left
}

fn draw(maze: &DiGraph<Cell, Direction>) {
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
          Some(_) => row_y_lbound.push_str("  "),
          None => row_y_lbound.push_str("--"),
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

// fn initialize_grid() -> DiGraph<Cell, Direction> {
//   let mut full_grid = petgraph::graph::DiGraph::<Cell, Direction>::new();
//   let mut cell_indexes = [[NodeIndex::from(0u32); WIDTH]; HEIGHT];
//   for _y in 0..HEIGHT {
//     for _x in 0..WIDTH {
//       cell_indexes[_y][_x] = full_grid.add_node(Cell { x: _x, y: _y });
//     }
//   }

//   for _y in 0..HEIGHT {
//     for _x in 0..WIDTH {
//       let (x_bound, y_bound) = (WIDTH - 1, HEIGHT - 1);

//       if _x > 0 {
//         full_grid.add_edge(cell_indexes[_y][_x],
//                            cell_indexes[_y][_x - 1],
//                            Direction::West);  // to left cell
//       }

//       if _x < x_bound {
//         full_grid.add_edge(cell_indexes[_y][_x],
//                            cell_indexes[_y][_x + 1],
//                            Direction::Est);   // to right cell
//       }

//       if _y > 0 {
//         full_grid.add_edge(cell_indexes[_y][_x],
//                            cell_indexes[_y - 1][_x],
//                            Direction::North); // to up cell
//       }

//       if _y < y_bound {
//         full_grid.add_edge(cell_indexes[_y][_x],
//                            cell_indexes[_y + 1][_x],
//                            Direction::South); // to down cell
//       }
//     }
//   }

//   full_grid
// }
fn select_generic<T: Copy>(select: bool, value_for_true: T, value_for_false: T) -> T {
  let values = [value_for_false, value_for_true];
  let flag = unsafe { std::mem::transmute::<bool, u8>(select) } as usize;
  values[flag].clone()
}

// on the left
fn western_cell(x: usize, y: usize,
                cell_matrix: &[[NodeIndex; WIDTH]; HEIGHT]) -> Option<NodeIndex> {
  select_generic::<Option<NodeIndex>>(x > 0, Some(cell_matrix[y][x - 1]), None)
  // let values = [None, Some(cell_matrix[y][x - 1])];
  // let flag = unsafe { std::mem::transmute::<bool, u8>(x > 0) } as usize;
  // values[flag]
  // if x > 0 {
  //   Some(cell_matrix[y][x - 1])
  // } else {
  //   None
  // }
}

// on the right
fn eastern_cell(x: usize, y: usize,
                cell_matrix: &[[NodeIndex; WIDTH]; HEIGHT]) -> Option<NodeIndex> {
  // if x < WIDTH - 1 {
  //   Some(cell_matrix[y][x + 1])
  // } else {
  //   None
  // }
  select_generic::<Option<NodeIndex>>(x < WIDTH - 1, Some(cell_matrix[y][x + 1]), None)
}

// upper
fn northern_cell(x: usize, y: usize,
                 cell_matrix: &[[NodeIndex; WIDTH]; HEIGHT]) -> Option<NodeIndex> {
  if y > 0 {
    Some(cell_matrix[y - 1][x])
  } else {
    None
  }
}

// lower
fn southern_cell(x: usize, y: usize,
                 cell_matrix: &[[NodeIndex; WIDTH]; HEIGHT]) -> Option<NodeIndex> {
  if y < HEIGHT - 1 {
    Some(cell_matrix[y + 1][x])
  } else {
    None
  }
}

fn generate_maze() -> DiGraph<Cell, Direction> {
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

  struct Wall {
    pub source: petgraph::graph::NodeIndex,
    pub target: petgraph::graph::NodeIndex,
    pub direction: [Direction; 2],
  }

  let mut examining_walls = Vec::new();
  let init_cell_index = maze_cell_indexes[y_init][x_init];


  fn bounded_walls(x: usize, y: usize, maze: &[[NodeIndex; WIDTH]; HEIGHT]) -> Vec<Wall> {
    let mut walls = Vec::new();
    let init_cell_index = maze[y][x];

    match western_cell(x, y, maze) {
      Some(cell_index) => {
        walls.push(Wall { source: init_cell_index, target: cell_index,
                          direction: [Direction::West, Direction::Est] });
      },
      None => (),
    }

    match eastern_cell(x, y, maze) {
      Some(cell_index) => {
        walls.push(Wall { source: init_cell_index, target: cell_index,
                          direction: [Direction::Est, Direction::West] });
      },
      None => (),
    }

    match northern_cell(x, y, maze) {
      Some(cell_index) => {
        walls.push(Wall { source: init_cell_index, target: cell_index,
                          direction: [Direction::North, Direction::South] });
      },
      None => (),
    }

    match southern_cell(x, y, &maze) {
      Some(cell_index) => {
        walls.push(Wall { source: init_cell_index, target: cell_index,
                          direction: [Direction::South, Direction::North] });
      },
      None => (),
    }

    walls
  }

  examining_walls.append(&mut bounded_walls(x_init, y_init, &maze_cell_indexes));

  let mut examined_cell_indexes = Vec::new();
  examined_cell_indexes.push(init_cell_index);

  let mut examined_walls = Vec::new();

  while !examining_walls.is_empty() {
    // remove a random wall from the examining set
    let random_wall_index = next_random_number() % examining_walls.len();
    let examining_wall = examining_walls.remove(random_wall_index);

    let attached_cell_index = examining_wall.target;

    // if the attached cell is not examined yet
    if !examined_cell_indexes.contains(&attached_cell_index) {
      maze.add_edge(examining_wall.source, examining_wall.target, examining_wall.direction[0]);
      maze.add_edge(examining_wall.target, examining_wall.source, examining_wall.direction[1]);

      let attached_cell = maze.node_weight(attached_cell_index).unwrap();
      let attached_cell_walls = bounded_walls(attached_cell.x, attached_cell.y, &maze_cell_indexes);
      for wall in attached_cell_walls {
        if !examined_cell_indexes.contains(&wall.target) {
          // examined_walls.push(wall);
          examining_walls.push(wall);
        }
      }

      examined_cell_indexes.push(attached_cell_index);
    }

    examined_walls.push(examining_wall); // add the wall into the examined set
  }

  maze
}

fn main() {
  // println!("Hello, world!");
  // use pid as random seed
  // let pid = unistd::getpid();
  // println!("pid = {}", pid);
  // let mut maze = petgraph::graph::DiGraph::<Cell, Direction>::new();
  // initialize(&mut maze);
  // let full_grid = initialize_grid();
  let maze = generate_maze();
  draw(&maze);
  // draw_maze(3, 4);
  let ran = next_random_number();
  // println!("seed = {}", next_random_number<u32>());
  // println!("seed = {}", next_random_number());
  println!("seed = {}", ran);
}
