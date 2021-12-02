pub struct Instruction {
    direction: Direction,
    value: u32,
}

pub enum Direction {
    Forward,
    Down,
    Up,
}

pub struct Position {
    x: u32,
    y: u32,
    aim: i32,
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let parts = line.split_whitespace().collect::<Vec<&str>>();
            let direction = match parts[0] {
                "forward" => Direction::Forward,
                "down" => Direction::Down,
                "up" => Direction::Up,
                _ => panic!("Unknown direction: {}", parts[0]),
            };
            let value = parts[1].parse::<u32>().unwrap();
            Instruction { direction, value }
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(instructions: &[Instruction]) -> u32 {
    let mut pos = Position { x: 0, y: 0, aim: 0 };
    for instruction in instructions {
        match instruction.direction {
            Direction::Forward => pos.x += instruction.value,
            Direction::Down => pos.y += instruction.value,
            Direction::Up => pos.y -= instruction.value,
        }
    }
    pos.x * pos.y
}

#[aoc(day2, part2)]
pub fn solve_part2(instructions: &[Instruction]) -> u32 {
    let mut pos = Position { x: 0, y: 0, aim: 0 };
    for instruction in instructions {
        match instruction.direction {
            Direction::Forward => {
                pos.x += instruction.value;
                pos.y += pos.aim as u32 * instruction.value;
            }
            Direction::Down => pos.aim += instruction.value as i32,
            Direction::Up => pos.aim -= instruction.value as i32,
        }
    }
    pos.x * pos.y
}
