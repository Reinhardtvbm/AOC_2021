use std::{collections::HashMap, fs};

pub fn parse_input() -> Vec<usize> {
    let file = fs::read_to_string("src/day6/day6.in").unwrap();
    let mut lines = file.lines();

    lines
        .next()
        .unwrap()
        .split(',')
        .into_iter()
        .map(|_str| _str.parse().unwrap())
        .collect()
}

pub fn day6_part1() {
    let mut state = parse_input();

    println!("{:?}", state);

    for day in 0..80 {
        let mut new_fish = Vec::new();

        for i in 0..state.len() {
            if state[i] == 0 {
                new_fish.push(8);
                state[i] = 7;
            }

            state[i] -= 1;
        }

        state.extend(new_fish);

        // println!("{}: {:?}", day + 1, state);
    }

    println!("\nDay 6, part 1: {}", state.into_iter().count());
}

pub fn day6_part2() {
    let init_state = parse_input();

    let mut state: HashMap<usize, usize> = HashMap::new();

    for i in 0..=9 {
        state.insert(i, 0);
    }

    for s in init_state {
        if let Some(day_value) = state.get_mut(&s) {
            *day_value += 1;
        }
    }

    for day in 0..256 {
        for days_left in 0..=9 {
            if days_left == 0 {
                let amount_new_fish = state.get(&0).unwrap().clone();

                *state.get_mut(&9).unwrap() += amount_new_fish;
                *state.get_mut(&7).unwrap() += amount_new_fish;
                *state.get_mut(&0).unwrap() = 0;
            }

            if days_left > 0 {
                *state.get_mut(&(days_left - 1)).unwrap() = *state.get(&days_left).unwrap();
            }
        }

        *state.get_mut(&9).unwrap() = 0;

        // for (key, val) in state.iter() {
        //     for _ in 0..*val {
        //         print!("{},", key);
        //     }
        // }
        // print!("\n");
    }

    println!("Day 6, part 2: {}", state.values().sum::<usize>());
}
