#![allow(unused)]

use std::mem::swap;

use meaningful_lines::FileSections;

fn main() {
    part1();
    part2();
}

fn part1() {
    let data = Data::new(include_str!("../data.txt"));
    let updates: u32 = data
        .apply_rules()
        .iter()
        .filter_map(|update| match update {
            Update::InvalidUpdate(_) => None,
            Update::ValidUpdate(page) => Some(page),
        })
        .sum();

    println!("Part 1: {}", updates);
}

fn part2() {
    let data = Data::new(include_str!("../data.txt"));
    let updates = data
        .apply_rules()
        .iter()
        .filter(|update| match update {
            Update::InvalidUpdate(_) => true,
            Update::ValidUpdate(_) => false,
        })
        .map(|update| match update {
            Update::InvalidUpdate(pages) => data.correct_pages(pages),
            Update::ValidUpdate(_) => panic!("Invalid update"),
        })
        .map(|pages| pages[pages.len() / 2])
        .sum::<u32>();

    println!("Part 2: {}", updates);
}

#[derive(Debug)]
struct Data {
    ordering_rules: Vec<(u32, u32)>,
    updates: Vec<Vec<u32>>,
}

#[derive(Debug)]
enum Update {
    // The pages are invalid.  Data is the invalid pages
    InvalidUpdate(Vec<u32>),

    // The update pages are valid and the data is the middle page
    ValidUpdate(u32),
}

impl Data {
    fn new(input: &str) -> Self {
        let mut sections = FileSections::new(input);

        let ordering_rules_input = sections.next().expect("No ordering rules");
        let update_input = sections.next().expect("No updates");

        let ordering_rules = ordering_rules_input
            .lines()
            .map(|line| {
                let mut parts = line.split('|');
                let a = parts
                    .next()
                    .expect("No first part")
                    .parse()
                    .expect("Not a number");
                let b = parts
                    .next()
                    .expect("No second part")
                    .parse()
                    .expect("Not a number");
                (a, b)
            })
            .collect();

        let updates = update_input
            .lines()
            .map(|line| {
                line.split(',')
                    .map(|part| part.parse().expect("Not a number"))
                    .collect()
            })
            .collect();

        Self {
            ordering_rules,
            updates,
        }
    }

    fn apply_rules(&self) -> Vec<Update> {
        self.updates
            .iter()
            .map(|update| {
                let mut valid = true;
                for (a, b) in &self.ordering_rules {
                    let a_index = update.iter().position(|&x| x == *a);
                    let b_index = update.iter().position(|&x| x == *b);

                    if let (Some(a_index), Some(b_index)) = (a_index, b_index) {
                        if a_index > b_index {
                            valid = false;
                            break;
                        }
                    }
                }
                if valid {
                    Update::ValidUpdate(update[update.len() / 2])
                } else {
                    Update::InvalidUpdate(update.clone())
                }
            })
            .collect()
    }

    fn correct_pages(&self, updates: &Vec<u32>) -> Vec<u32> {
        let mut pages = updates.clone();
        let mut change_count = 1;
        let mut iteration_count = 0;

        while change_count > 0 {
            change_count = 0;

            for (a, b) in &self.ordering_rules {
                let a_index = pages.iter().position(|&x| x == *a);
                let b_index = pages.iter().position(|&x| x == *b);

                if let (Some(a_index), Some(b_index)) = (a_index, b_index) {
                    if a_index > b_index {
                        let elem = pages.remove(a_index);
                        pages.insert(b_index, elem);
                        change_count += 1;
                    }
                }
            }

            iteration_count += 1;
            if iteration_count > self.ordering_rules.len() {
                panic!("Unable to solve constraints");
            }
        }

        pages
    }
}
