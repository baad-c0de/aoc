use iterbox::CombinationIterator;
use meaningful_lines::MeaningfulLines;

fn main() {
    part1();
    part2();
}

fn part1() {
    let lines = MeaningfulLines::new(include_str!("../data.txt"));
    let operators = vec![Operator::Add, Operator::Multiply];

    let result = lines
        .map(parse_line)
        .filter_map(|(result, elems)| valid_calculation(result, &elems, &operators))
        .sum::<u64>();

    println!("Part 1: {}", result);
}

fn part2() {
    let lines = MeaningfulLines::new(include_str!("../data.txt"));
    let operators = vec![Operator2::Add, Operator2::Multiply, Operator2::Concatenate];

    let result = lines
        .map(parse_line)
        .filter_map(|(result, elems)| valid_calculation(result, &elems, &operators))
        .sum::<u64>();

    println!("Part 2: {}", result);
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Operator {
    Add,
    Multiply,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Operator2 {
    Add,
    Multiply,
    Concatenate,
}

trait Operation {
    fn apply(&self, a: u64, b: u64) -> u64;
}

impl Operation for Operator {
    fn apply(&self, a: u64, b: u64) -> u64 {
        match self {
            Operator::Add => a + b,
            Operator::Multiply => a * b,
        }
    }
}

impl Operation for Operator2 {
    fn apply(&self, a: u64, b: u64) -> u64 {
        match self {
            Operator2::Add => a + b,
            Operator2::Multiply => a * b,
            Operator2::Concatenate => {
                let a_string = a.to_string();
                let b_string = b.to_string();

                let concatenated_string = format!("{}{}", a_string, b_string);

                concatenated_string.parse().unwrap()
            }
        }
    }
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

fn valid_calculation<T>(result: u64, elems: &[u64], operators: &[T]) -> Option<u64>
where
    T: Copy + Operation,
{
    // Create a vector of all the permutations of the operators
    let n = elems.len() - 1;

    let combinations = CombinationIterator::new(operators, n);

    for combination in combinations {
        let mut partial_result = elems[0];

        for (i, operator) in combination.iter().enumerate() {
            partial_result = operator.apply(partial_result, elems[i + 1]);
        }

        if partial_result == result {
            return Some(result);
        }
    }

    None
}
