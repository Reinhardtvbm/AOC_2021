use std::fs;

enum Direction {
    Forward(usize),
    Up(usize),
    Down(usize),
}

pub fn day2_part1() {
    let file = fs::read_to_string("src/day2/day2.in").unwrap();

    let mut directions = Vec::new();

    for line in file.lines() {
        let split: Vec<&str> = line.split(" ").collect();

        directions.push(match split[0] {
            "forward" => Direction::Forward(split[1].parse().unwrap()),
            "down" => Direction::Down(split[1].parse().unwrap()),
            "up" => Direction::Up(split[1].parse().unwrap()),
            _ => panic!("Something went wrong"),
        });
    }

    let (mut depth, mut horiz) = (0, 0);

    directions
        .into_iter()
        .for_each(|direction| match direction {
            Direction::Forward(amount) => horiz += amount,
            Direction::Up(amount) => depth -= amount,
            Direction::Down(amount) => depth += amount,
        });

    println!("\nDay 2, part 1: {}", depth * horiz);
}

pub fn day2_part2() {
    let file = fs::read_to_string("src/day2/day2.in").unwrap();

    let mut directions = Vec::new();

    for line in file.lines() {
        let split: Vec<&str> = line.split(" ").collect();

        directions.push(match split[0] {
            "forward" => Direction::Forward(split[1].parse().unwrap()),
            "down" => Direction::Down(split[1].parse().unwrap()),
            "up" => Direction::Up(split[1].parse().unwrap()),
            _ => panic!("Something went wrong"),
        });
    }

    let (mut aim, mut depth, mut horiz) = (0, 0, 0);

    directions
        .into_iter()
        .for_each(|direction| match direction {
            Direction::Forward(amount) => {
                horiz += amount;
                depth += amount * aim;
            }
            Direction::Up(amount) => aim -= amount,
            Direction::Down(amount) => aim += amount,
        });

    println!("Day 2, part 2: {}", depth * horiz);
}
