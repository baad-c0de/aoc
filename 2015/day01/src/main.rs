fn main() {
    part1();
    part2();
}

fn calculate_floor(input: &str) -> i32 {
    input.chars().fold(0, |acc, c| match c {
        '(' => acc + 1,
        ')' => acc - 1,
        _ => acc,
    })
}

fn part1() {
    let input = include_str!("../data.txt");
    let floor = calculate_floor(input);
    println!("Part 1: {}", floor);
}

fn part2() {
    let input = include_str!("../data.txt");
    let mut floor = 0;

    input.chars().enumerate();

    for (i, c) in input.chars().enumerate() {
        floor += match c {
            '(' => 1,
            ')' => -1,
            _ => 0,
        };

        if floor == -1 {
            println!("Part 2: {}", i + 1);
            break;
        }
    }
}
