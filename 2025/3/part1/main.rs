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
        for line in lines.map_while(Result::ok) {
            let mut largest = 0;
            let mut secondlargest = 0;
            let mut lrt = 0;
            let mut idx: usize = 0;
            let mut idx2: usize = 0;
            for s in 0..line.len() {
                let digit: i32 = line[s..s + 1].parse().unwrap();

                if digit > largest {
                    secondlargest = largest;
                    idx2 = idx;
                    largest = digit;
                    idx = s;
                }
            }

            if idx == line.len() - 1 {
                largest = secondlargest;
                idx = idx2
            }

            for s in idx + 1..line.len() {
                let digit: i32 = line[s..s + 1].parse().unwrap();

                if digit > lrt {
                    lrt = digit;
                }
            }

            total += (largest * 10) + lrt;
        }
    }
    println!("{:?}", total);
}
