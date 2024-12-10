use std::fs;

fn add_tuple(mut order: Vec<i32>, tuple: (i32, i32)) -> Vec<i32> {
    // Get id of value in list
    let a = order.iter().position(|&x| x == tuple.0);
    let b = order.iter().position(|&x| x == tuple.1);

    if a.is_none() && b.is_none() {
        order.push(tuple.0);
        order.push(tuple.1);
    } else if a.is_some() && b.is_none() {
        order.push(tuple.1);
    } else if a.is_none() && b.is_some() {
        order.insert(b.unwrap(), tuple.0);
    } else if a.is_some() && b.is_some() {
        if a.unwrap() > b.unwrap() {
            order.swap(a.unwrap(), b.unwrap());
        }
    }
    order
}

fn main() {
    // Read input
    let input = fs::read_to_string("input").unwrap();

    // Part answers
    let mut p1 = 0;
    let mut p2 = 0;

    let mut rules: Vec<(i32, i32)> = vec![];

    let mut is_rules = true;
    // Parse input
    for line in input.lines() {
        dbg!(line);
        if line.is_empty() {
            is_rules = false;
            continue;
        }

        if is_rules {
            let mut items = line.split('|');
            let a: i32 = items.next().unwrap().parse::<i32>().unwrap();
            let b: i32 = items.next().unwrap().parse::<i32>().unwrap();
            rules.push((a, b));
        } else {
            let mut valid_order: Vec<i32> = vec![];

            let numbers: Vec<i32> = line
                .split(',')
                .map(|num| num.parse::<i32>().unwrap())
                .collect();
            for tuple in rules.iter() {
                let a = numbers.iter().position(|&x| x == tuple.0);
                let b = numbers.iter().position(|&x| x == tuple.1);

                if a.is_some() && b.is_some() {
                    valid_order = add_tuple(valid_order, *tuple);
                    println!("Rules {} {}, order: ", tuple.0, tuple.1);
                    //dbg!(&valid_order);
                }
            }

            if valid_order == numbers {
                println!("Valid");
                dbg!(&valid_order);
                p1 += valid_order[valid_order.len() / 2];
            } else {
                p2 += valid_order[valid_order.len() / 2];
            }
        }
    }

    println!("Partie 1: {p1}");
    println!("Partie 2: {p2}");
}
