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
            let nums: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
            let mut l: [usize; 12] = [0; 12];
            let mut lidx: usize = 0;

            'out: loop {
                let r = if lidx == 0 { l[0] } else { l[lidx - 1] + 1 };

                for s in r..=nums.len() - 12 + lidx {
                    if s == r {
                        l[lidx] = s;
                    } else if nums[s] > nums[l[lidx]] {
                        l[lidx] = s;
                    }
                }

                lidx += 1;

                if lidx == 12 {
                    break 'out;
                }
            }

            for n in 0..l.len() {
                total += nums[l[n]] as u64 * 10_u64.pow(11 - (n as u32));
            }
        }
    }

    println!("{:?}", total);
}
