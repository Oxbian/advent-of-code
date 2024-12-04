use regex::Regex;
use std::fs;

fn main() {
    // Read input
    let input = fs::read_to_string("input").expect("Should have been able to read the file");

    // Part answers
    let mut p1 = 0;
    let mut p2 = 0;

    // Bool to enable or not the multiplication
    let mut enabled = true;

    // Parse input
    for line in input.lines() {
        // Parsing line with Regex (thx ChatGPT)
        let re = Regex::new(r"(mul\((\d+),(\d+)\)|do\(\)|don't\(\))").unwrap();

        for pattern in re.captures_iter(line) {
            if &pattern[1] == "do()" {
                enabled = true;
            } else if &pattern[1] == "don't()" {
                enabled = false;
            } else if enabled {
                p1 += pattern[2].to_string().parse::<i32>().unwrap()
                    * pattern[3].to_string().parse::<i32>().unwrap();
                p2 += pattern[2].to_string().parse::<i32>().unwrap()
                    * pattern[3].to_string().parse::<i32>().unwrap();
            } else {
                p1 += pattern[2].to_string().parse::<i32>().unwrap()
                    * pattern[3].to_string().parse::<i32>().unwrap();
            }
        }
    }

    println!("Partie 1: {p1}");
    println!("Partie 2: {p2}");
}
