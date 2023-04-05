use std::{collections::HashMap, fs, num};

pub fn parse_input() -> Vec<(Vec<usize>, Vec<usize>)> {
    let file = fs::read_to_string("src/day5/day5.in").unwrap();
    let lines = file.lines();

    lines
        .into_iter()
        .map(|line| {
            let left_right = line.split(" -> ").collect::<Vec<&str>>();

            let left: Vec<usize> = left_right[0]
                .split(',')
                .into_iter()
                .map(|_str| _str.parse().unwrap())
                .collect();

            let right: Vec<usize> = left_right[1]
                .split(',')
                .into_iter()
                .map(|_str| _str.parse().unwrap())
                .collect();

            (left, right)
        })
        .collect()
}

pub fn day5_part1() {
    let input = parse_input();

    let mut position_counts: HashMap<(usize, usize), usize> = HashMap::new();

    for (start, end) in input {
        let (start_x, start_y) = (start[0], start[1]);
        let (end_x, end_y) = (end[0], end[1]);

        // println!("{},{} -> {},{}", start_x, start_y, end_x, end_y);

        if start_x == end_x {
            let range = match start_y < end_y {
                true => start_y..=end_y,
                false => end_y..=start_y,
            };

            for y in range {
                if let Some(previous_val) = position_counts.get_mut(&(start_x, y)) {
                    *previous_val += 1;
                } else {
                    position_counts.insert((start_x, y), 1);
                }
            }
        } else if start_y == end_y {
            let range = match start_x < end_x {
                true => start_x..=end_x,
                false => end_x..=start_x,
            };

            for x in range {
                if let Some(previous_val) = position_counts.get_mut(&(x, start_y)) {
                    *previous_val += 1;
                } else {
                    position_counts.insert((x, start_y), 1);
                }
            }
        }
        // println!("{:?}", position_counts);
    }

    let number_overlaps = position_counts
        .iter()
        .filter(|((_, _), count)| **count > 1)
        .count();

    println!("\nDay 5, part 1: {}", number_overlaps);
}

pub fn day5_part2() {
    let input = parse_input();

    let mut position_counts: HashMap<(usize, usize), usize> = HashMap::new();

    for (start, end) in input {
        let (start_x, start_y) = (start[0], start[1]);
        let (end_x, end_y) = (end[0], end[1]);

        // println!("{},{} -> {},{}", start_x, start_y, end_x, end_y);

        if start_x == end_x {
            let range = match start_y < end_y {
                true => start_y..=end_y,
                false => end_y..=start_y,
            };

            for y in range {
                if let Some(previous_val) = position_counts.get_mut(&(start_x, y)) {
                    *previous_val += 1;
                } else {
                    position_counts.insert((start_x, y), 1);
                }
            }
        } else if start_y == end_y {
            let range = match start_x < end_x {
                true => start_x..=end_x,
                false => end_x..=start_x,
            };

            for x in range {
                if let Some(previous_val) = position_counts.get_mut(&(x, start_y)) {
                    *previous_val += 1;
                } else {
                    position_counts.insert((x, start_y), 1);
                }
            }
        } else {
            let x_range: Box<dyn Iterator<Item = usize>> = match start_x < end_x {
                true => Box::new(start_x..=end_x),
                false => Box::new((end_x..=start_x).rev()),
            };

            let y_range: Box<dyn Iterator<Item = usize>> = match start_y < end_y {
                true => Box::new(start_y..=end_y),
                false => Box::new((end_y..=start_y).rev()),
            };

            for (x, y) in x_range.into_iter().zip(y_range) {
                // print!("{},{}; ", x, y);

                if let Some(previous_val) = position_counts.get_mut(&(x, y)) {
                    *previous_val += 1;
                } else {
                    position_counts.insert((x, y), 1);
                }
            }
            // print!("\n");
        }
        // println!("{:?}", position_counts);
    }

    let number_overlaps = position_counts
        .iter()
        .filter(|((_, _), count)| **count > 1)
        .count();

    println!("Day 5, part 2: {}", number_overlaps);
}
