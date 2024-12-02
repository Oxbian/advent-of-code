use std::fs;

fn check_line(num: &Vec<i32>) -> bool {
    let mut sens: i8 = 0;

    for i in 1..num.len() {
        let diff = num[i - 1] - num[i];

        if diff > 0 && diff.abs() <= 3 && sens <= 0 {
            sens = -1;
        } else if diff < 0 && diff.abs() <= 3 && sens >= 0 {
            sens = 1;
        } else {
            return false;
        }
    }
    true
}

fn main() {
    // Read input
    let input = fs::read_to_string("input").expect("Should have been able to read the file");

    // Part answers
    let mut p1 = 0;
    let mut p2 = 0;

    // Parse input
    for line in input.lines() {
        // Checking on lines
        let num: Vec<i32> = line
            .split_whitespace()
            .map(|numbers| numbers.parse::<i32>().unwrap())
            .collect();

        if check_line(&num) {
            p1 += 1;
            p2 += 1;
        } else {
            for i in 0..num.len() {
                let mut num_copy = num.clone();
                num_copy.remove(i);
                if check_line(&num_copy) {
                    p2 += 1;
                    break;
                }
            }
        }
    }

    println!("Partie 1: {p1}");
    println!("Partie 2: {p2}");
}
