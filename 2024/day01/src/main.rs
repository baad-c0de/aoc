use std::str::Lines;

fn main() {
    part1();
    part2();
}

fn get_sorted_vectors(lines: &'static str) -> (Vec<u32>, Vec<u32>) {
    let lines = DoubleDigits::new(lines);

    let (mut first, mut second): (Vec<u32>, Vec<u32>) = lines.unzip();
    first.sort();
    second.sort();

    (first, second)
}

fn part1() {
    let (first, second) = get_sorted_vectors(include_str!("../data1.txt"));
    let first_iterator = first.iter();
    let second_iterator = second.iter();

    let sum_diff: u32 = first_iterator
        .zip(second_iterator)
        .map(|(a, b)| a.abs_diff(*b))
        .sum();

    println!("Sum of differences: {}", sum_diff);
}

fn part2() {
    let (first, second) = get_sorted_vectors(include_str!("../data2.txt"));

    let answer: u32 = first
        .iter()
        .map(|a| second.iter().filter(|b| *a == **b).count() as u32 * a)
        .sum();

    println!("Part 2 answer: {answer}");
}

struct MeaningfulLines<'a> {
    lines: Lines<'a>,
}

impl<'a> MeaningfulLines<'a> {
    fn new(lines: &'a str) -> Self {
        Self {
            lines: lines.lines(),
        }
    }
}

impl<'a> Iterator for MeaningfulLines<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        self.lines.find(|line| !line.trim().is_empty())
    }
}

struct DoubleDigits<'a> {
    lines: MeaningfulLines<'a>,
}

impl<'a> DoubleDigits<'a> {
    fn new(lines: &'a str) -> Self {
        Self {
            lines: MeaningfulLines::new(lines),
        }
    }
}

impl<'a> Iterator for DoubleDigits<'a> {
    type Item = (u32, u32);

    fn next(&mut self) -> Option<Self::Item> {
        let line = self.lines.next()?;
        let mut digits = line.split_whitespace().filter_map(|s| s.parse().ok());
        let first = digits.next()?;
        let second = digits.next()?;
        Some((first, second))
    }
}
