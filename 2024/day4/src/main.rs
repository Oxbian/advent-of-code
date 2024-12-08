use std::fs;

fn find_horizontal(line: &str) -> usize {
    let xmas: Vec<&str> = line.matches("XMAS").collect();
    let r_xmas: Vec<&str> = line.matches("SAMX").collect();
    xmas.len() + r_xmas.len()
}

fn find_diagonals(input: &[Vec<char>]) -> usize {
    let mut count: usize = 0;
    let y_size = input.len();
    for y in 0..y_size {
        let x_size = input[y].len();
        for x in 0..x_size {
            // Search for pattern
            if input[y][x] == 'X' {
                // Left to right
                if y + 3 < y_size
                    && x + 3 < x_size
                    && input[y + 1][x + 1] == 'M'
                    && input[y + 2][x + 2] == 'A'
                    && input[y + 3][x + 3] == 'S'
                {
                    count += 1;
                }
                if y >= 3
                    && x >= 3
                    && input[y - 1][x - 1] == 'M'
                    && input[y - 2][x - 2] == 'A'
                    && input[y - 3][x - 3] == 'S'
                {
                    count += 1;
                }
                // Right to left
                if y + 3 < y_size
                    && x >= 3
                    && input[y + 1][x - 1] == 'M'
                    && input[y + 2][x - 2] == 'A'
                    && input[y + 3][x - 3] == 'S'
                {
                    count += 1;
                }
                if y >= 3
                    && x + 3 < x_size
                    && input[y - 1][x + 1] == 'M'
                    && input[y - 2][x + 2] == 'A'
                    && input[y - 3][x + 3] == 'S'
                {
                    count += 1;
                }

                // Straight
                if y >= 3
                    && input[y - 1][x] == 'M'
                    && input[y - 2][x] == 'A'
                    && input[y - 3][x] == 'S'
                {
                    count += 1;
                }
                if y + 3 < y_size
                    && input[y + 1][x] == 'M'
                    && input[y + 2][x] == 'A'
                    && input[y + 3][x] == 'S'
                {
                    count += 1;
                }
            }
        }
    }
    count
}

fn find_x_mas(input: &[Vec<char>]) -> usize {
    let mut count: usize = 0;
    let y_size = input.len();
    for y in 0..y_size {
        let x_size = input[y].len();
        for x in 0..x_size {
            // Search for pattern
            if input[y][x] == 'A' {
                if y + 1 < y_size
                    && y >= 1
                    && x + 1 < x_size
                    && x >= 1
                    && ((input[y - 1][x - 1] == 'M' && input[y + 1][x + 1] == 'S')
                        || (input[y - 1][x - 1] == 'S' && input[y + 1][x + 1] == 'M'))
                    && ((input[y - 1][x + 1] == 'M' && input[y + 1][x - 1] == 'S')
                        || (input[y - 1][x + 1] == 'S' && input[y + 1][x - 1] == 'M'))
                {
                    count += 1;
                }
            }
        }
    }
    count
}

fn main() {
    // Read input
    let input = fs::read_to_string("input").unwrap();

    // Part answers
    let mut p1 = 0;
    let mut p2 = 0;

    // Parse input
    for line in input.lines() {
        // Parsing line to search XMAS and SAMX
        p1 += find_horizontal(line);
    }

    // Check grid for XMAS in diagonals or vertically
    let input = input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    p1 += find_diagonals(&input);
    p2 += find_x_mas(&input);

    println!("Partie 1: {p1}");
    println!("Partie 2: {p2}");
}
