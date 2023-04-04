use std::fs;

pub fn day3_part1() {
    let file = fs::read_to_string("src/day3/day3.in").unwrap();

    let mut numbers = Vec::new();
    let bit_length = file.lines().into_iter().nth(0).unwrap().len();

    for line in file.lines() {
        numbers.push(usize::from_str_radix(line, 2).unwrap());
    }

    let mut gamma: usize = 0;

    for bit_no in 0..bit_length {
        let (mut ones, mut zeros) = (0, 0);

        for num in numbers.iter() {
            let bit = (num >> bit_no) & 1;

            match bit == 1 {
                true => ones += 1,
                false => zeros += 1,
            };
        }

        if ones > zeros {
            gamma |= 1 << bit_no;
        }
    }

    let epsilon = !gamma & ((1 << bit_length) - 1);

    println!("\nDay 3, part 1: {}", gamma * epsilon);
}

fn get_gamma(bit_length: usize, numbers: &Vec<usize>) -> usize {
    let mut gamma: usize = 0;

    for bit_no in 0..bit_length {
        let (mut ones, mut zeros) = (0, 0);

        for num in numbers.iter() {
            let bit = (num >> bit_no) & 1;

            match bit == 1 {
                true => ones += 1,
                false => zeros += 1,
            };
        }

        if ones >= zeros {
            gamma |= 1 << bit_no;
        }
    }

    gamma
}

pub fn day3_part2() {
    let file = fs::read_to_string("src/day3/day3.in").unwrap();

    let mut numbers_g = Vec::new();
    let mut numbers_e = Vec::new();

    let bit_length = file.lines().into_iter().nth(0).unwrap().len();

    for line in file.lines() {
        numbers_g.push(usize::from_str_radix(line, 2).unwrap());
        numbers_e.push(usize::from_str_radix(line, 2).unwrap());
    }

    let mut gamma = get_gamma(bit_length, &numbers_g);
    let mut epsilon = !get_gamma(bit_length, &numbers_e) & ((1 << bit_length) - 1);

    let mut bit_no = 0;

    while numbers_g.len() > 1 || numbers_e.len() > 1 {
        if numbers_g.len() > 1 {
            numbers_g.retain(|number| {
                let test_bit = (1 << (bit_length - bit_no - 1)) & gamma;
                let bit = (1 << (bit_length - bit_no - 1)) & number;

                bit == test_bit
            })
        }

        if numbers_e.len() > 1 {
            numbers_e.retain(|number| {
                let test_bit = (1 << (bit_length - bit_no - 1)) & epsilon;
                let bit = (1 << (bit_length - bit_no - 1)) & number;

                bit == test_bit
            });
        }

        gamma = get_gamma(bit_length, &numbers_g);
        epsilon = !get_gamma(bit_length, &numbers_e) & ((1 << bit_length) - 1);

        bit_no += 1;
    }

    println!("Day 3, part 2: {}", numbers_g[0] * numbers_e[0])
}
