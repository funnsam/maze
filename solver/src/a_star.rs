use crate::*;
use std::collections::HashMap;
use priority_queue::priority_queue::PriorityQueue;

fn reconstruct(came_from: &Vec<Vec<Option<(usize, usize)>>>, mut current: (usize, usize)) -> Vec<(usize, usize)> {
    let mut path = vec![current];

    while let Some(prev) = came_from[current.1][current.0] {
        path.insert(0, prev);
        current = prev;
    }

    path
}

pub fn solve(maze: &Maze) -> Option<Vec<(usize, usize)>> {
    let h = |(x, y)| {
        let dx = maze.goal_x - x;
        let dy = maze.goal_y - y;
        // ((dx*dx + dy*dy) as f32).sqrt() as usize
        (((dx*dx + dy*dy) as f32).sqrt() * 8.0) as usize
    };

    let mut open = PriorityQueue::with_capacity(1);

    let mut closed = HashMap::new();

    open.push((0, 0), usize::MAX - h((0, 0)));

    let mut g_score = vec![vec![usize::MAX; maze.size_x]; maze.size_y];
    let mut f_score = vec![vec![usize::MAX; maze.size_x]; maze.size_y];
    g_score[0][0] = 0;
    f_score[0][0] = h((0, 0));

    let mut came_from = vec![vec![None; maze.size_x]; maze.size_y];

    while let Some((current, f)) = open.pop() {
        closed.insert(current, f);

        if current.0 == maze.goal_x && current.1 == maze.goal_y {
            return Some(reconstruct(&came_from, current));
        }

        for n in Neighbour::at(&current, &maze) {
            let t_g = g_score[current.1][current.0] + 6;

            if t_g < g_score[n.1][n.0] {
                came_from[n.1][n.0] = Some(current);
                g_score[n.1][n.0] = t_g;

                let t_f = t_g + h(n);
                f_score[n.1][n.0] = t_f;

                open.push_decrease(n, usize::MAX - t_f);
            }
        }

        print_grid(&maze.grid, open.clone().into_vec().as_slice(), &closed, &(maze.goal_x, maze.goal_y));
    }

    None
}

struct Neighbour {
    // 0000 WSEN
    unexplored: u8,
    at: (usize, usize)
}

impl Iterator for Neighbour {
    type Item = (usize, usize);
    fn next(&mut self) -> Option<(usize, usize)> {
        for b in 0..4_u8 {
            if (self.unexplored >> b) & 1 == 1 {
                self.unexplored ^= 1 << b;
                let d: Direction = unsafe {
                    core::mem::transmute(b)
                };
                return Some((self.at.0 + d.dx() as usize, self.at.1 + d.dy() as usize));
            }
        }

        None
    }
}

impl Neighbour {
    fn at(node: &(usize, usize), maze: &Maze) -> Self {
        Self {
            unexplored:
                (((0..maze.size_y).contains(&(node.1 - 1)) && !maze.grid[node.1-1][node.0]) as u8) << 0 |
                (((0..maze.size_x).contains(&(node.0 + 1)) && !maze.grid[node.1][node.0+1]) as u8) << 1 |
                (((0..maze.size_y).contains(&(node.1 + 1)) && !maze.grid[node.1+1][node.0]) as u8) << 2 |
                (((0..maze.size_x).contains(&(node.0 - 1)) && !maze.grid[node.1][node.0-1]) as u8) << 3,
            at: node.clone()
        }
    }
}

pub fn print_grid(grid: &Vec<Vec<bool>>, open: &[(usize, usize)], closed: &HashMap<(usize, usize), usize>, end: &(usize, usize)) {
    let min = closed.values().map(|a| usize::MAX - *a).min().unwrap_or(1);
    let max = closed.values().map(|a| usize::MAX - *a).max().unwrap_or(1) - min;

    let snap = |x| {
        (((x - min) as f32 / max as f32) * 95.0) as usize
    };

    for (yi, y) in grid.iter().enumerate() {
        println!("|{}\x1b[0m|", y.iter().enumerate()
            .map(|(x, a)|
                if x + yi == 0 {
                    "\x1b[101m  ".to_string()
                } else if x == end.0 && yi == end.1 {
                    "\x1b[102m  ".to_string()
                } else if open.contains(&(x, yi)) {
                    "\x1b[0;94m▓▓".to_string()
                } else if let Some(s) = closed.get(&(x, yi)) {
                    format!(
                        "\x1b[0;48;2;{};{};0m  ",
                        snap(usize::MAX - *s),
                        95 - snap(usize::MAX - *s)
                    )
                } else if *a {
                    "\x1b[107m  ".to_string()
                } else {
                    "\x1b[0m  ".to_string()
                }
            )
            .collect::<String>()
        );
    }

    println!("\x1b[{}A", grid.len()+1);
    std::thread::sleep_ms(100);
}
