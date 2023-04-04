use std::{collections::VecDeque, fs};

pub fn day1_part1() {
    let file = fs::read_to_string("src/day1/day1.in").unwrap();

    let mut prev = 0;
    let mut curr = 0;
    let mut increases = -1;

    for line in file.lines() {
        prev = curr;

        curr = line.parse().unwrap();

        if curr > prev {
            increases += 1;
        }
    }

    println!("\nDay 1, part 1: {}", increases);
}

struct Queue(VecDeque<usize>);

impl Queue {
    fn new() -> Self {
        Self(VecDeque::new())
    }

    fn push(&mut self, val: usize) {
        self.0.push_front(val);

        if self.0.len() == 4 {
            self.pop();
        }
    }

    fn pop(&mut self) {
        self.0.pop_back();
    }

    fn sum(&self) -> Option<usize> {
        match self.0.len() == 3 {
            true => Some(self.0.iter().sum()),
            false => None,
        }
    }
}

pub fn day1_part2() {
    let file = fs::read_to_string("src/day1/day1.in").unwrap();

    let mut measurements = Queue::new();
    let mut prev_sum = 0;
    let mut curr_sum = 0;

    let mut increases = -1;

    for line in file.lines() {
        measurements.push(line.parse().unwrap());

        prev_sum = curr_sum;

        if let Some(sum) = measurements.sum() {
            curr_sum = sum;
        }

        if curr_sum > prev_sum {
            increases += 1;
        }
    }

    println!("Day 1, part 2: {}", increases);
}
