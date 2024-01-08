use common::*;

const I: bool = false;
const W: bool = true;

fn main() {
    let maze = Maze {
        size_x: 10,
        size_y: 10,
        goal_x: 9,
        goal_y: 9,
        grid: vec![
            vec![I,I,I,I,I,I,I,I,I,I],
            vec![I,I,I,I,I,I,I,I,I,I],
            vec![I,I,I,I,I,I,I,I,I,I],
            vec![I,I,I,I,I,I,W,I,I,I],
            vec![I,I,I,I,I,I,W,I,I,I],
            vec![I,I,I,I,I,I,W,I,I,I],
            vec![I,I,I,W,W,W,W,I,I,I],
            vec![I,I,I,I,I,I,I,I,I,I],
            vec![I,I,I,I,I,I,I,I,I,I],
            vec![I,I,I,I,I,I,I,I,I,I],
        ],
    };

    std::fs::write("maze.mz", to_allocvec(&maze).unwrap()).unwrap();
}

