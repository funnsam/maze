use serde::*;
use postcard::*;

mod a_star;
mod utils;

#[derive(Debug, Clone, Deserialize)]
pub struct Maze {
    pub size_x: usize,
    pub size_y: usize,
    pub goal_x: usize,
    pub goal_y: usize,
    pub grid: Vec<Vec<bool>>,
}

fn main() {
    let maze = std::fs::read("maze.mz").unwrap();
    let maze: Maze = from_bytes(&maze).unwrap();

    let solution = a_star::solve(&maze).unwrap();
    print_grid(&maze.grid, &solution, &(maze.goal_x, maze.goal_y));
}

pub fn print_grid(grid: &Vec<Vec<bool>>, solution: &[(usize, usize)], end: &(usize, usize)) {
    for (yi, y) in grid.iter().enumerate() {
        println!("|{}\x1b[0m|", y.iter().enumerate()
            .map(|(x, a)|
                if x + yi == 0 {
                    "\x1b[101m  "
                } else if x == end.0 && yi == end.1 {
                    "\x1b[102m  "
                } else if solution.contains(&(x, yi)) {
                    "\x1b[0;33m▓▓"
                } else if *a {
                    "\x1b[107m  "
                } else {
                    "\x1b[0m  "
                }
            )
            .collect::<String>()
        );
    }

    // println!("\x1b[{}A", grid.len()+1);
}
