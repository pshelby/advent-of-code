use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut words_to_digits: HashMap<&str, char> = HashMap::new();
    words_to_digits.insert("zero", '0');
    words_to_digits.insert("one", '1');
    words_to_digits.insert("two", '2');
    words_to_digits.insert("three", '3');
    words_to_digits.insert("four", '4');
    words_to_digits.insert("five", '5');
    words_to_digits.insert("six", '6');
    words_to_digits.insert("seven", '7');
    words_to_digits.insert("eight", '8');
    words_to_digits.insert("nine", '9');

    // File input.txt must exist in current path
    if let Ok(lines) = read_lines("./input.txt") {
        let mut sum: i32 = 0;

        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            //println!("line: {:?}", line);

            let mut nums = HashMap::new();

            // Find digits in line
            for (index, c) in line.as_ref().unwrap().chars().enumerate() {
                if c.is_ascii_digit() {
                    //println!("{}: {}", index, c);
                    nums.insert(index, c);
                }
            }
            //println!("digits: {:?}", digits);

            // Find number words in line
            for (word, char) in words_to_digits.iter() {
                let matches: Vec<_> = line.as_ref().unwrap().match_indices(word).collect();

                if !matches.is_empty() {
                    //println!("{:?} matches: {:?}", word, matches);
                    for (mindex, _) in matches.iter() {
                        //println!("mindex: {}", mindex);
                        nums.insert(*mindex, *char);
                    }
                }
            }

            // Order nums by index
            let mut num_keys: Vec<usize> = nums.clone().into_keys().collect();
            num_keys.sort();
            //println!("num_keys: {:?}", num_keys);

            // Concatenate first and last char in nums vector
            let mut first = nums.get(&num_keys[0]).unwrap().to_string();
            let last = nums.get(&num_keys[num_keys.len() - 1]).unwrap().to_string();
            first.push_str(&last);

            // Convert string to i32
            let num: i32 = first.parse().unwrap();
            //println!("num: {}", num);

            // Add num to sum
            sum += num;
        }

        println!("callibration sum: {}", sum);
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
