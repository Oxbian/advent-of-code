use std::fs;

fn main() {
    // Read input
    let input = fs::read_to_string("input").expect("Should have been able to read the file");

    // Parse input
    let mut left = vec![];
    let mut right = vec![];

    for line in input.lines() {
        let mut items = line.split_whitespace();
        left.push(items.next().unwrap().parse::<i32>().unwrap());
        right.push(items.next().unwrap().parse::<i32>().unwrap());
    }

    // Sort lists
    left.sort();
    right.sort();

    let mut p1 = 0;
    let mut p2 = 0;
    for i in 0..left.len() {
        p1 += (left[i] - right[i]).abs();

        let mut occur = 0;
        for j in 0..right.len() {
            if left[i] == right[j] {
                occur += 1;
            }
        }
        p2 += left[i] * occur;
    }

    println!("Partie 1: {p1}");
    println!("Partie 2: {p2}");
}
