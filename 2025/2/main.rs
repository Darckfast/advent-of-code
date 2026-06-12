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
    let mut total: u64 = 0;
    if let Ok(lines) = read_lines("./input") {
        for line in lines.map_while(Result::ok) {
            let parts = line.split(",");

            for part in parts {
                let mut startend = part.split("-");

                if let (Some(first), Some(last)) = (startend.next(), startend.next()) {
                    let fi: u64 = first.parse().unwrap();
                    let li: u64 = last.parse().unwrap();

                    for i in fi..(li + 1) {
                        let s = i.to_string();

                        if s.len() % 2 == 0 {
                            let fh = &s[..s.len() / 2];
                            let lh = &s[s.len() / 2..];

                            if fh == lh {
                                total += i;
                            }
                        }
                    }
                }
            }
        }
    }
    println!("{:?}", total);
}
