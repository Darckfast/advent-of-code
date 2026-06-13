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

                    for i in fi..=li {
                        let s = i.to_string();
                        let mut dst: &str;

                        for j in 1..s.len() {
                            dst = &s[0..j];

                            if j > s.len() / 2 {
                                break;
                            }

                            let lh = &s[j..j + dst.len()];
                            if *dst == *lh {
                                if s.len() % j != 0 {
                                    break;
                                }
                                let mut has = false;
                                for k in (0..s.len()).step_by(j) {
                                    if s[k..k + j] == *dst {
                                        has = true;
                                    } else {
                                        has = false;
                                        break;
                                    }
                                }
                                if has {
                                    total += i;
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    println!("{:?}", total);
}
