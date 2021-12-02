#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<u32> {
    input.lines().map(|l| l.parse::<u32>().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[u32]) -> u32 {
    let mut last = u32::max_value();
    let mut counter = 0;
    input.iter().for_each(|i| {
        if last < *i {
            counter += 1;
        }
        last = *i;
    });
    counter
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[u32]) -> u32 {
    let mut last = u32::max_value();
    let mut index = 0;
    let mut counter = 0;
    while index + 3 <= input.len() {
        let batch = input[index..index + 3].iter().sum::<u32>();
        if batch > last {
            counter += 1;
        }
        last = batch;
        index += 1;
    }
    counter
}
