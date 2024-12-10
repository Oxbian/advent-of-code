use std::fs;

fn create_path(grid: &mut Vec<Vec<char>>, mut pos: (i32, i32)) -> i32 {
    let mut count = 0;
    let mut dir = (0, -1);

    // Checking if guard leave the grid
    while pos.0 < (grid[0].len() - 1) as i32
        && pos.0 > 0
        && pos.1 > 0
        && pos.1 < (grid.len() - 1) as i32
    {
        println!("Pos: {}:{}, dir: {}:{}", pos.0, pos.1, dir.0, dir.1);
        if grid[(pos.1 + dir.1) as usize][(pos.0 + dir.0) as usize] == '#' {
            println!("Change");
            match dir {
                (0, 1) => dir = (-1, 0),
                (1, 0) => dir = (0, 1),
                (0, -1) => dir = (1, 0),
                (-1, 0) => dir = (0, -1),
                _ => panic!("Pattern non reconnu"),
            }
        } else {
            if grid[(pos.1 + dir.1) as usize][(pos.0 + dir.0) as usize] == '#' {
                grid[(pos.1 + dir.1) as usize][(pos.0 + dir.0) as usize] = 'O';
            } else if grid[(pos.1 + dir.1) as usize][(pos.0 + dir.0) as usize] == '.' {
                count += 1;
                println!("Count +1: {count}");
                grid[(pos.1 + dir.1) as usize][(pos.0 + dir.0) as usize] = 'X';
            }
            pos.0 += dir.0;
            pos.1 += dir.1;
        }
    }
    count + 1
}

fn guard_pos(grid: &Vec<Vec<char>>) -> (i32, i32) {
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == '^' {
                return (x as i32, y as i32);
            }
        }
    }
    (-1, -1)
}

fn draw_grid(grid: &Vec<Vec<char>>) {
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            print!("{}", grid[y][x]);
        }
        println!("");
    }
}

fn main() {
    // Read input
    let input = fs::read_to_string("input").unwrap();

    // Part answers
    let mut p1 = 0;
    let mut p2 = 0;

    // Parse input into a grid
    let mut input = input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let pos: (i32, i32) = guard_pos(&input);

    if pos.0 >= 0 && pos.1 >= 0 {
        p1 = create_path(&mut input, pos);
    }

    draw_grid(&input);

    println!("Partie 1: {p1}");
    println!("Partie 2: {p2}");
}
