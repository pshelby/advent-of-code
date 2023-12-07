use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    /*
    let mut lines = io::stdin().lock().lines();
    let mut user_input = String::new();

    while let Some(line) = lines.next() {
        let last_input = line.unwrap();

        // stop reading
        if last_input.len() == 0 {
            break;
        }

        // add a new line once user_input starts storing user input
        if user_input.len() > 0 {
            user_input.push_str("\n");
        }

        // store user input
        user_input.push_str(&last_input);
    }

    println!("\nMulti-line user input \n{}", user_input);

    // the lock is released after it goes out of scope
    Ok(())
    */

    // File hosts must exist in current path before this produces output
    let path = "/Users/pshelby/Documents/Programming Practice/Advent of Code 2022/day_1/src/test1.txt";
    println!("Reading file {}", path);
    if let Ok(lines) = read_lines(path) {
        // Consumes the iterator, returns an (Optional) String
        let mut elf_total = 0i32;
        let mut elf_counter = 1i32;
        for line in lines {
            if let Ok(ip) = line {
                if ip.len() == 0 {
                    println!("Elf {:?} total: {:?}", elf_counter, elf_total);
                    elf_counter += 1;
                    elf_total = 0i32;
                } else {
                    elf_total += ip.parse::<i32>().unwrap();
                }
                //println!("{}", ip);
            }
        }
        println!("Elf {:?} total: {:?}", elf_counter, elf_total);
    }
}
