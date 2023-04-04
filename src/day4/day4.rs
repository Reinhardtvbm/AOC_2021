use std::fs;

pub fn parse_input() -> (Vec<usize>, Vec<[[usize; 5]; 5]>) {
    let file = fs::read_to_string("src/day4/day4.in").unwrap();
    let mut lines = file.lines();

    let draws: Vec<usize> = lines
        .next()
        .unwrap()
        .split(',')
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|str_num| str_num.parse().unwrap())
        .collect();

    lines.next().unwrap();
    let mut boards: Vec<[[usize; 5]; 5]> = Vec::new();

    let mut curr_board: [[usize; 5]; 5] = [[0; 5]; 5];
    let mut row_num = 0;

    for line in lines {
        if line == "" {
            boards.push(curr_board.clone());
            row_num = 0;
        } else {
            let row: Vec<usize> = line
                .split(' ')
                .collect::<Vec<&str>>()
                .into_iter()
                .filter(|string| *string != "")
                .map(|str_num| str_num.parse().unwrap())
                .collect();

            for i in 0..5 {
                curr_board[row_num][i] = row[i]
            }

            row_num += 1;
        }
    }

    boards.push(curr_board.clone());

    (draws, boards)
}

pub fn day4_part1_get_board(
    draws: &Vec<usize>,
    boards: &Vec<[[usize; 5]; 5]>,
) -> (usize, Vec<(usize, usize)>, [[usize; 5]; 5]) {
    let mut highlighted_indices: Vec<Vec<(usize, usize)>> =
        boards.iter().map(|_| Vec::new()).collect();

    for number in draws {
        for (board_index, board) in boards.iter().enumerate() {
            for (row_index, row) in board.into_iter().enumerate() {
                if let Some(col) = row.into_iter().position(|board_num| *board_num == *number) {
                    highlighted_indices[board_index].push((row_index, col));
                }
            }
        }

        for (board_index, highlights) in highlighted_indices.iter().enumerate() {
            let mut row_counts = [0; 5];
            let mut col_counts = [0; 5];

            for (row, col) in highlights {
                row_counts[*row] += 1;
                col_counts[*col] += 1;
            }

            if row_counts.contains(&5) || col_counts.contains(&5) {
                return (*number, (*highlights).clone(), boards[board_index]);
            }
        }
    }

    (0, Vec::new(), [[0; 5]; 5])
}

pub fn day4_part1() {
    let (draws, boards) = parse_input();

    let (last_num, indices, winning_board) = day4_part1_get_board(&draws, &boards);

    let mut sum = 0;

    for (i, row) in winning_board.into_iter().enumerate() {
        for (j, cell) in row.into_iter().enumerate() {
            if !indices.contains(&(i, j)) {
                sum += cell;
            }
        }
    }

    println!("\nDay 4, part 1: {}", sum * last_num);
}

pub fn day4_part2() {
    let (draws, mut boards) = parse_input();

    let mut last_num = 0;
    let mut indices = Vec::new();
    let mut winning_board = [[0; 5]; 5];

    while boards.len() > 1 {
        let (last_num_new, indices_new, winning_board_new) = day4_part1_get_board(&draws, &boards);

        winning_board = winning_board_new;
        indices = indices_new;
        last_num = last_num_new;

        boards.retain(|board| *board != winning_board);
    }

    let (last_num_new, indices_new, winning_board_new) = day4_part1_get_board(&draws, &boards);

    winning_board = winning_board_new;
    indices = indices_new;
    last_num = last_num_new;

    let mut sum = 0;

    for (i, row) in winning_board.into_iter().enumerate() {
        for (j, cell) in row.into_iter().enumerate() {
            if !indices.contains(&(i, j)) {
                sum += cell;
            }
        }
    }

    // println!("{}", last_num);
    println!("Day 4, part 2: {}", sum * last_num);
}
