use clap::*;
use common::*;

#[derive(Debug, Clone, Parser)]
struct Args {
    size_x: usize,
    size_y: usize,
    #[clap(short, long, default_value_t = 1)]
    seed: usize,
}

// http://weblog.jamisbuck.org/2010/12/27/maze-generation-recursive-backtracking
fn generate_maze(args: &Args) -> (Vec<Vec<bool>>, (usize, usize)) {
    let mut max_depth = 0;
    let mut max_cell = (0, 0);

    fn carve(x: usize, y: usize, grid: &mut Vec<Vec<bool>>, max_depth: &mut usize, max_cell: &mut (usize, usize), depth: usize) {
        let directions = RandomDirection::default();
        for dir in directions {
            let x1 = (x as isize + dir.dx()) as usize;
            let y1 = (y as isize + dir.dy()) as usize;
            let x2 = (x as isize + dir.dx() * 2) as usize;
            let y2 = (y as isize + dir.dy() * 2) as usize;

            grid[y][x] = false;

            if (0..grid.len()).contains(&y2) && (0..grid[y2].len()).contains(&x2) && grid[y2][x2] {
                if depth > *max_depth {
                    *max_depth = depth;
                    *max_cell = (x2, y2);
                }

                grid[y1][x1] = false;
                carve(x2, y2, grid, max_depth, max_cell, depth+1);
            }
        }
    }

    let mut grid = vec![vec![true; args.size_x]; args.size_y];

    carve(0, 0, &mut grid, &mut max_depth, &mut max_cell, 0);

    (grid, max_cell)
}

fn main() {
    let args = Args::parse();
    srand(args.seed as u64);

    let (grid, end) = generate_maze(&args);
    print_grid(&grid, &end);
    println!("\x1b[{}B", grid.len()+1);

    let maze = Maze {
        size_x: args.size_x,
        size_y: args.size_y,
        goal_x: end.0,
        goal_y: end.1,
        grid
    };
    std::fs::write("maze.mz", to_allocvec(&maze).unwrap()).unwrap();
}

fn print_grid(grid: &Vec<Vec<bool>>, end: &(usize, usize)) {
    for (yi, y) in grid.iter().enumerate() {
        println!("|{}\x1b[0m|", y.iter().enumerate()
            .map(|(x, a)| if x == end.0 && yi == end.1 {
                "\x1b[102m  "
            } else if *a {
                "\x1b[107m  "
            } else {
                "\x1b[0m  "
            })
            .collect::<String>()
        );
    }

    println!("\x1b[{}A", grid.len()+1);
}
