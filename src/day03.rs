#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<String> {
    input.lines().map(|line| line.to_string()).collect()
}

#[aoc(day3, part1)]
pub fn solve_part1(lines: &[String]) -> u32 {
    let mut gamma = vec![];
    let mut epsilon = vec![];
    for i in 0..12 {
        let ones = lines
            .iter()
            .filter(|line| line.chars().nth(i).unwrap() == '1')
            .count();
        let zeros = lines.len() - ones;
        if ones >= zeros {
            gamma.push("1");
            epsilon.push("0");
        } else {
            gamma.push("0");
            epsilon.push("1");
        }
    }
    let gamma = isize::from_str_radix(&gamma.join(""), 2).unwrap() as u32;
    let epsilon = isize::from_str_radix(&epsilon.join(""), 2).unwrap() as u32;
    gamma * epsilon
}

#[aoc(day3, part2)]
pub fn solve_part2(lines: &[String]) -> u32 {
    let mut oxygen_lines: Vec<String> = lines.into();
    let mut co2_lines: Vec<String> = lines.into();
    for i in 0..12 {
        let ones_oxygen = oxygen_lines
            .iter()
            .filter(|line| line.chars().nth(i).unwrap() == '1')
            .count();
        let zeroes_oxygen = oxygen_lines.len() - ones_oxygen;
        let ones_co2 = co2_lines
            .iter()
            .filter(|line| line.chars().nth(i).unwrap() == '1')
            .count();
        let zeroes_co2 = co2_lines.len() - ones_co2;
        if oxygen_lines.len() == 1 && co2_lines.len() == 1 {
            break;
        }
        if ones_oxygen >= zeroes_oxygen {
            if oxygen_lines.len() != 1 {
                oxygen_lines.retain(|line| line.chars().nth(i).unwrap() == '0');
            }
        } else if oxygen_lines.len() != 1 {
            oxygen_lines.retain(|line| line.chars().nth(i).unwrap() == '1');
        }
        if ones_co2 >= zeroes_co2 {
            if co2_lines.len() != 1 {
                co2_lines.retain(|line| line.chars().nth(i).unwrap() == '1');
            }
        } else if co2_lines.len() != 1 {
            co2_lines.retain(|line| line.chars().nth(i).unwrap() == '0');
        }
    }
    let oxygen_rate = isize::from_str_radix(&oxygen_lines[0], 2).unwrap() as u32;
    let co2_rate = isize::from_str_radix(&co2_lines[0], 2).unwrap() as u32;
    oxygen_rate * co2_rate
}
