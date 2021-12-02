pub use crate::file_helpers;

pub fn part_1() {
    let instructions = get_instructions();
    let initial_position = Position { horizontal: 0, depth: 0 };
    let final_position = apply_instructions_part1(initial_position, instructions, 0);
    let solution = final_position.depth * final_position.horizontal;
    println!("{}", solution);
}

pub fn part_2() {
    let instructions = get_instructions();
    let initial_position = PositionWithAim { horizontal: 0, depth: 0, aim: 0 };
    let final_position = apply_instructions_part2(initial_position, instructions, 0);
    let solution = final_position.depth * final_position.horizontal;
    println!("{}", solution);
}

struct Position {
    horizontal: i64,
    depth: i64,
}

struct PositionWithAim {
    horizontal: i64,
    depth: i64,
    aim: i64,
}

struct Instruction {
    direction: Direction,
    magnitude: i64,
}

enum Direction {
    Forward,
    Down,
    Up,
}

fn apply_instructions_part1(current_position: Position, instructions: Vec<Instruction>, current_index: usize) -> Position {
    let current_instruction = &instructions[current_index];
    let next_position = apply_instruction_part1(current_position, current_instruction);
    let next_index = current_index + 1;

    return if next_index < instructions.len() {
        apply_instructions_part1(next_position, instructions, next_index)
    } else {
        next_position
    }
}

fn apply_instruction_part1(current_position: Position, instruction: &Instruction) -> Position {
    let current_horizontal = current_position.horizontal;
    let current_depth = current_position.depth;

    return match instruction.direction {
        Direction::Forward => Position { horizontal: current_horizontal + instruction.magnitude, depth: current_depth },
        Direction::Down => Position { horizontal: current_horizontal, depth: current_depth + instruction.magnitude },
        Direction::Up => Position { horizontal: current_horizontal, depth: current_depth - instruction.magnitude },
    }
}

fn apply_instructions_part2(current_position: PositionWithAim, instructions: Vec<Instruction>, current_index: usize) -> PositionWithAim {
    let current_instruction = &instructions[current_index];
    let next_position = apply_instruction_part2(current_position, current_instruction);
    let next_index = current_index + 1;

    return if next_index < instructions.len() {
        apply_instructions_part2(next_position, instructions, next_index)
    } else {
        next_position
    }
}

fn apply_instruction_part2(current_position: PositionWithAim, instruction: &Instruction) -> PositionWithAim {
    let current_horizontal = current_position.horizontal;
    let current_depth = current_position.depth;
    let current_aim = current_position.aim;

    let new_position = match instruction.direction {
        Direction::Forward =>
            PositionWithAim
            {
                horizontal: current_horizontal + instruction.magnitude,
                depth: current_depth + (instruction.magnitude * current_aim),
                aim: current_aim,
            },
        Direction::Down =>
            PositionWithAim
            {
                horizontal: current_horizontal,
                depth: current_depth,
                aim: current_aim + instruction.magnitude,
            },
        Direction::Up =>
            PositionWithAim
            {
                horizontal: current_horizontal,
                depth: current_depth,
                aim: current_aim - instruction.magnitude,
            },
    };

    return new_position;
}

fn get_instructions() -> Vec<Instruction> {
    let input_lines = file_helpers::read_file_as_lines("day_02/input.txt");
    let mut instructions: Vec<Instruction> = Vec::new();

    for line in input_lines {
        let instruction = parse_instruction(line);
        instructions.push(instruction);
    }

    return instructions;
}

fn parse_instruction(line: String) -> Instruction {
    let string_values = line.split(" ").collect::<Vec<&str>>();

    let direction: Direction = match string_values[0] {
        "forward" => Direction::Forward,
        "down" => Direction::Down,
        "up" => Direction::Up,
        _ => panic!("Invalid direction")
    };

    let magnitude: i64 = string_values[1].parse().expect("Invalid magnitude");

    return Instruction { direction, magnitude };
}
