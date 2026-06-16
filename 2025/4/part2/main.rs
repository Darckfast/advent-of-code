use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let mut total: i32 = 0;
    if let Ok(lines) = read_lines("./input") {
        let mut grid: Vec<Vec<char>> = lines
            .map_while(Result::ok)
            .map(|line| line.chars().collect())
            .collect();

        let directions = [
            (0, 1),
            (1, 1),
            (1, 0),
            (1, -1),
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
        ];

        let mut done = false;

        while !done {
            done = true;
            let mut g = grid.clone();
            for x in 0..grid.len() {
                let row = &grid[x];
                for y in 0..row.len() {
                    let mut streak = 0;
                    if row[y] == '@' {
                        for (dx, dy) in directions {
                            let x1 = (x as i32) + dx;
                            let y1 = (y as i32) + dy;

                            if x1 >= 0
                                && y1 >= 0
                                && (x1 as usize) < grid.len()
                                && (y1 as usize) < row.len()
                                && g[x1 as usize][y1 as usize] == '@'
                            {
                                streak += 1;
                            }
                        }

                        if streak < 4 {
                            done = false;
                            total += 1;
                            g[x][y] = '.';
                        }
                    }
                }
            }
            grid = g;
        }
    }

    println!("{:?}", total);
}
