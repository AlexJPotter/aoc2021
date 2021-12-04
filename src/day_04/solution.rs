use crate::file_helpers;

pub fn part_1() {
    let initial_game_state = get_initial_game_state();
    let final_game_state = play(initial_game_state);
    let winning_board = final_game_state.boards.iter().find(|b| has_won(b)).unwrap();
    let score = get_score(winning_board);
    let solution = final_game_state.numbers.first().unwrap() * score;
    println!("Solution: {}", solution);
}

pub fn part_2() {
    let initial_game_state = get_initial_game_state();
    let final_game_state = play_2(initial_game_state, Vec::new());
    let solution = final_game_state.1.last().unwrap();
    println!("Solution: {}", solution);
}

fn get_score(board: &Board) -> u32 {
    let mut score = 0;
    board.numbers.iter().for_each(|&row| row.iter().for_each(|&x| if x.is_some() { score = score + x.unwrap() }));
    return score;
}

fn play(state: GameState) -> GameState {
    if state.numbers.is_empty() {
        return state;
    }

    let &number = state.numbers.first().unwrap();

    let mut new_boards: Vec<Board> = Vec::new();

    for board in state.boards {
        new_boards.push(check_number(number, &board));
    }

    if new_boards.iter().any(|board| has_won(board)) {
        return GameState { boards: new_boards, numbers: state.numbers };
    }

    let mut new_numbers = Vec::new();
    state.numbers[1..].iter().for_each(|&n| new_numbers.push(n));
    let new_state = GameState { boards: new_boards, numbers: new_numbers };
    return play(new_state);
}

fn play_2(state: GameState, winner_scores: Vec<u32>) -> (GameState, Vec<u32>) {
    if state.numbers.is_empty() {
        return (state, winner_scores);
    }

    let &number = state.numbers.first().unwrap();

    let mut new_boards: Vec<Board> = Vec::new();

    for board in state.boards {
        new_boards.push(check_number(number, &board));
    }

    let new_winners = new_boards.iter().filter(|&b| has_won(b));
    let mut winner_scores_next: Vec<u32> = winner_scores.clone();
    new_winners.for_each(|b| winner_scores_next.push(number * get_score(b)));

    let mut remaining_boards: Vec<Board> = Vec::new();
    new_boards.iter().filter(|&b| !has_won(b)).for_each(|b| remaining_boards.push(b.clone()));

    let mut new_numbers = Vec::new();
    state.numbers[1..].iter().for_each(|&n| new_numbers.push(n));
    let new_state = GameState { boards: remaining_boards, numbers: new_numbers };
    return play_2(new_state, winner_scores_next);
}

fn check_number(number: u32, board: &Board) -> Board {
    let mut new_board = board.clone();

    let indices: [usize; 5] = [0, 1, 2, 3, 4];

    for i in indices {
        for j in indices {
            let number_at_position = board.numbers[i][j];

            new_board.numbers[i][j] =
                if number_at_position.is_none() { Option::None }
                else if number_at_position.unwrap() == number { Option::None }
                else { number_at_position };
        }
    }

    return new_board;
}

fn get_initial_game_state() -> GameState {
    let input_lines = file_helpers::read_file_as_raw_lines("day_04/input.txt");

    let mut numbers: Vec<u32> = Vec::new();
    let mut boards: Vec<Board> = Vec::new();

    let mut row_index: usize = 0;
    let indices: [usize; 5] = [0, 1, 2, 3, 4];

    let mut current_board: Board = Board {
        numbers: [[Option::None; 5]; 5],
    };

    for line in input_lines {
        if numbers.len() == 0 {
            line.split(",").for_each(|value| numbers.push(value.parse().expect("Invalid number")));
            continue;
        }

        if line.trim() == "" && row_index > 0 {
            row_index = 0;
            boards.push(current_board.clone());
            continue;
        }

        if line.trim() != "" {
            let mut row: [Option<u32>; 5] = [Option::None; 5];

            for i in indices {
                let start = i * 3;
                let end = start + 2;
                let value: u32 = line[start..end].trim().parse().expect("Invalid number");
                row[i] = Option::from(value);
            }

            current_board.numbers[row_index] = row;

            row_index = row_index + 1;
        }
    }

    return GameState { numbers, boards };
}

struct GameState {
    numbers: Vec<u32>,
    boards: Vec<Board>,
}

#[derive(Clone)]
struct Board {
    numbers: [[Option<u32>; 5]; 5],
}

fn has_won(board: &Board) -> bool {
    let indices: [usize; 5] = [0, 1, 2, 3, 4];
    return indices.iter().any(|&index| row_is_complete(board, index) || column_is_complete(board, index));
}

fn row_is_complete(board: &Board, row_index: usize) -> bool {
    return board.numbers[row_index].iter().all(|element| element.is_none());
}

fn column_is_complete(board: &Board, column_index: usize) -> bool {
    return board.numbers.iter().all(|row| row[column_index].is_none());
}
