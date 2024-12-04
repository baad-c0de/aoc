use std::str::Lines;

use itertools::Itertools;

fn main() {
    part1();
    part2();
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

struct Reports {
    lines: MeaningfulLines<'static>,
}

impl Reports {
    fn new(lines: &'static str) -> Self {
        Self {
            lines: MeaningfulLines::new(lines),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Direction {
    Ascending,
    Descending,
    Mixed,
}

impl Iterator for Reports {
    type Item = (Direction, u32, u32);

    fn next(&mut self) -> Option<Self::Item> {
        let nums_line = self.lines.next()?;
        let nums = nums_line
            .split_whitespace()
            .map(str::parse::<u32>)
            .collect::<Result<Vec<_>, _>>()
            .ok()?;

        // Convert the values into an iterator of tuples of (Direction, u32)
        let stats = nums.iter().tuple_windows().map(|(a, b)| {
            let direction = match a < b {
                true => Direction::Ascending,
                false => Direction::Descending,
            };

            (direction, a.abs_diff(*b))
        });

        // Reduce the iterator of tuples into a single tuple.
        let answer = stats.map(|(dir, dist)| (dir, dist, dist)).reduce(
            |(a_dir, a_diff1, a_diff2), (b_dir, b_diff1, b_diff2)| {
                let min = a_diff1.min(b_diff1);
                let max = a_diff2.max(b_diff2);
                if a_dir == b_dir {
                    (a_dir, min, max)
                } else {
                    (Direction::Mixed, min, max)
                }
            },
        );

        answer
    }
}

fn part1() {
    let reports = Reports::new(include_str!("../data1.txt"));

    let count = reports
        .filter(|&(dir, min_diff, max_diff)| {
            dir != Direction::Mixed && max_diff <= 3 && min_diff >= 1
        })
        .count();
    println!("Valid reports: {count}");
}

struct Reports2 {
    lines: MeaningfulLines<'static>,
}

impl Reports2 {
    fn new(lines: &'static str) -> Self {
        Self {
            lines: MeaningfulLines::new(lines),
        }
    }
}

struct Report {
    pub nums: Vec<u32>,
}

impl Iterator for Reports2 {
    type Item = Report;

    fn next(&mut self) -> Option<Self::Item> {
        let nums_line = self.lines.next()?;
        let nums = nums_line
            .split_whitespace()
            .map(str::parse::<u32>)
            .collect::<Result<Vec<_>, _>>()
            .ok()?;

        Some(Report { nums })
    }
}

fn part2() {
    let reports = Reports2::new(include_str!("../data2.txt"));

    let count = reports
        .map(|report| {
            let is_safe = (0..report.nums.len())
                .map(|i| {
                    let nums_before = 0..i;
                    let nums_after = i + 1..report.nums.len();
                    let new_report = nums_before
                        .chain(nums_after)
                        .map(|i| report.nums[i])
                        .collect::<Vec<_>>();

                    let stats = new_report.iter().tuple_windows().map(|(a, b)| {
                        let direction = match a < b {
                            true => Direction::Ascending,
                            false => Direction::Descending,
                        };

                        (direction, a.abs_diff(*b))
                    });

                    let attrs = stats
                        .map(|(dir, dist)| (dir, dist, dist))
                        .reduce(|(a_dir, a_diff1, a_diff2), (b_dir, b_diff1, b_diff2)| {
                            let min = a_diff1.min(b_diff1);
                            let max = a_diff2.max(b_diff2);
                            if a_dir == b_dir {
                                (a_dir, min, max)
                            } else {
                                (Direction::Mixed, min, max)
                            }
                        })
                        .unwrap();

                    attrs.0 != Direction::Mixed && attrs.2 <= 3 && attrs.1 >= 1
                })
                .any(|is_safe| is_safe);

            is_safe
        })
        .filter(|is_safe| *is_safe)
        .count();

    println!("Valid reports: {count}");
}
