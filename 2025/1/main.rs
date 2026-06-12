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
    let mut sp = 50;
    let mut secret = 0;
    match read_lines("./input") {
        Ok(lines) => {
            for line in lines {
                match line {
                    Ok(l) => {
                        let mut chars = l.chars();
                        let f = chars.next().unwrap();
                        let r: i32 = chars.as_str().parse().unwrap();
                        match f {
                            'L' => sp -= r % 100,
                            'R' => sp += r % 100,
                            _ => {}
                        }

                        if sp < 0 {
                            sp += 100;
                        }

                        if sp > 99 {
                            sp -= 100;
                        }

                        if sp == 0 {
                            secret += 1;
                        }
                    }
                    Err(e) => eprintln!("error: {e}"),
                }
            }
        }
        Err(e) => eprintln!("file error: {e}"),
    }
    println!("{:?}", secret);
}
