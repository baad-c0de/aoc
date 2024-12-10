use iterbox::CombinationIterator;
use meaningful_lines::MeaningfulLines;

fn main() {
    part1();
    part2();
}

fn part1() {
    let lines = MeaningfulLines::new(include_str!("../data.txt"));

    let result = lines
        .map(parse_line)
        .filter_map(|(result, elems)| valid_calculation(result, &elems))
        .sum::<u64>();

    println!("Part 1: {}", result);
}

fn part2() {}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Operator {
    Add,
    Multiply,
}

fn parse_line(line: &str) -> (u64, Vec<u64>) {
    let mut parts = line.split(':');

    let result = parts.next().unwrap().parse().unwrap();

    let elems = parts
        .next()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    (result, elems)
}

fn valid_calculation(result: u64, elems: &[u64]) -> Option<u64> {
    // Create a vector of all the permutations of the operators
    let n = elems.len() - 1;
    let operators = vec![Operator::Add, Operator::Multiply];

    let combinations = CombinationIterator::new(&operators, n);

    for combination in combinations {
        let mut partial_result = elems[0];

        for (i, operator) in combination.iter().enumerate() {
            match operator {
                Operator::Add => partial_result += elems[i + 1],
                Operator::Multiply => partial_result *= elems[i + 1],
            }
        }

        if partial_result == result {
            return Some(result);
        }
    }

    None
}
