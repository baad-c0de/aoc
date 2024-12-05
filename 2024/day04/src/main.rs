#![allow(unused)]

use std::fmt::Display;

use meaningful_lines::MeaningfulLines;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

fn main() {
    part1();
    part2();
}

fn part1() {
    let grid = Wordsearch::new(include_str!("../data.txt"));
    let found_words = grid.find_words("XMAS");
    println!("Number of times found: {}", found_words.len());
}

fn part2() {
    let grid = Wordsearch::new(include_str!("../data.txt"));
    let found_words = grid.find_x_words("MAS");
    println!("Number of times found: {}", found_words.len());
}

struct Wordsearch {
    grid: Vec<char>,
    width: usize,
    height: usize,
}

#[derive(Debug, Clone, Copy, EnumIter)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

#[derive(Debug)]
struct FoundWord {
    word: String,
    start: (usize, usize),
    end: (usize, usize),
    direction: Direction,
}

#[derive(Debug)]
struct FoundXWord {
    word: String,
    start: (usize, usize),
}

impl Wordsearch {
    fn new(data: &'static str) -> Self {
        let height = MeaningfulLines::new(data).count();
        let width = MeaningfulLines::new(data)
            .next()
            .expect("Need at least one line in the data, and all lines should be the same length")
            .len();

        let mut grid = Vec::with_capacity(width * height);

        // Let's fill the grid with the data
        for line in MeaningfulLines::new(data) {
            for c in line.chars() {
                grid.push(c);
            }
        }
        if grid.len() != width * height {
            panic!("Something went wrong while filling the grid");
        }

        Wordsearch {
            grid,
            width,
            height,
        }
    }

    fn find_word_in_direction(
        &self,
        word: &str,
        start: (usize, usize),
        direction: Direction,
    ) -> Option<(usize, usize)> {
        let (mut x, mut y) = start;

        // Check to see if the possible word can actually fit in the grid based
        // on the direction.
        match direction {
            Direction::Up => {
                if y < word.len() - 1 {
                    return None;
                }
            }
            Direction::Down => {
                if y + word.len() > self.height {
                    return None;
                }
            }
            Direction::Left => {
                if x < word.len() - 1 {
                    return None;
                }
            }
            Direction::Right => {
                if x + word.len() > self.width {
                    return None;
                }
            }
            Direction::UpLeft => {
                if x < word.len() - 1 || y < word.len() - 1 {
                    return None;
                }
            }
            Direction::UpRight => {
                if x + word.len() > self.width || y < word.len() - 1 {
                    return None;
                }
            }
            Direction::DownLeft => {
                if x < word.len() - 1 || y + word.len() > self.height {
                    return None;
                }
            }
            Direction::DownRight => {
                if x + word.len() > self.width || y + word.len() > self.height {
                    return None;
                }
            }
        }

        let mut word_chars = word.chars();
        let mut current_char = word_chars.next().unwrap();
        loop {
            if self.grid[y * self.width + x] == current_char {
                // Check to see if we've reached the end of the word.
                // If so, return it's final position.
                let next_char = word_chars.next();
                if next_char.is_none() {
                    return Some((x, y));
                }

                // Otherwise, continue checking the rest of the word.
                match direction {
                    Direction::Up => {
                        if y == 0 {
                            return None;
                        }
                        y -= 1;
                    }
                    Direction::Down => {
                        if y == self.height - 1 {
                            return None;
                        }
                        y += 1;
                    }
                    Direction::Left => {
                        if x == 0 {
                            return None;
                        }
                        x -= 1;
                    }
                    Direction::Right => {
                        if x == self.width - 1 {
                            return None;
                        }
                        x += 1;
                    }
                    Direction::UpLeft => {
                        if x == 0 || y == 0 {
                            return None;
                        }
                        x -= 1;
                        y -= 1;
                    }
                    Direction::UpRight => {
                        if x == self.width - 1 || y == 0 {
                            return None;
                        }
                        x += 1;
                        y -= 1;
                    }
                    Direction::DownLeft => {
                        if x == 0 || y == self.height - 1 {
                            return None;
                        }
                        x -= 1;
                        y += 1;
                    }
                    Direction::DownRight => {
                        if x == self.width - 1 || y == self.height - 1 {
                            return None;
                        }
                        x += 1;
                        y += 1;
                    }
                }

                current_char = next_char.unwrap();
            } else {
                return None;
            }
        }
    }

    fn find_words(&self, word: &str) -> Vec<FoundWord> {
        let mut words = Vec::new();
        for y in 0..self.height {
            for x in 0..self.width {
                for direction in Direction::iter() {
                    if let Some(possible_word_pos) =
                        self.find_word_in_direction(word, (x, y), direction)
                    {
                        let (end_x, end_y) = possible_word_pos;
                        let found_word = FoundWord {
                            word: word.to_string(),
                            start: (x, y),
                            end: (end_x, end_y),
                            direction,
                        };
                        words.push(found_word);
                    }
                }
            }
        }
        words
    }

    fn find_x_word_at(&self, x: usize, y: usize, word: &str) -> Option<FoundXWord> {
        assert_eq!(word.len() % 2, 1, "Word length must be odd");

        let reach = (word.len() - 1) / 2;
        if reach > x || reach > y || reach > self.width - x || reach > self.height - y {
            return None;
        }

        // Check the top-left to bottom-right diagonal
        {
            let x = x - reach;
            let y = y - reach;
            if self
                .find_word_in_direction(word, (x, y), Direction::DownRight)
                .is_none()
            {
                // Check the bottom-right to top-left diagonal
                let x = x + 2 * reach;
                let y = y + 2 * reach;
                if self
                    .find_word_in_direction(word, (x, y), Direction::UpLeft)
                    .is_none()
                {
                    return None;
                }
            }
        }
        // Check the bottom-left to top-right diagonal
        {
            let x = x - reach;
            let y = y + reach;
            if self
                .find_word_in_direction(word, (x, y), Direction::UpRight)
                .is_none()
            {
                // Check the top-right to bottom-left diagonal
                let x = x + 2 * reach;
                let y = y - 2 * reach;
                if self
                    .find_word_in_direction(word, (x, y), Direction::DownLeft)
                    .is_none()
                {
                    return None;
                }
            }
        }

        Some(FoundXWord {
            word: word.to_string(),
            start: (x, y),
        })
    }

    fn find_x_words(&self, word: &str) -> Vec<FoundXWord> {
        let mut words = Vec::new();
        let reach = (word.len() - 1) / 2;

        for y in reach..self.height - reach {
            for x in reach..self.width - reach {
                if let Some(found_word) = self.find_x_word_at(x, y, word) {
                    words.push(found_word);
                }
            }
        }
        words
    }
}

impl Display for Wordsearch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{}", self.grid[y * self.width + x])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
