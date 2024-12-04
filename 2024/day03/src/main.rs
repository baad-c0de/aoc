fn main() {
    part1();
    part2();
}

fn part1() {
    let program = Program::new(include_str!("../data1.txt"));
    let sum: u32 = program
        .map(|inst| match inst {
            Instruction::Mul(a, b) => a * b,
            Instruction::Do => 0,
            Instruction::Dont => 0,
        })
        .sum();

    println!("Part 1: {}", sum);
}
fn part2() {
    let program = Program::new(include_str!("../data2.txt"));
    let mut enabled = true;
    let sum: u32 = program
        .map(|inst| match inst {
            Instruction::Mul(a, b) => {
                if enabled {
                    a * b
                } else {
                    0
                }
            }
            Instruction::Do => {
                enabled = true;
                0
            }
            Instruction::Dont => {
                enabled = false;
                0
            }
        })
        .sum();

    println!("Part 2: {}", sum);
}

#[derive(Debug)]
enum Instruction {
    Mul(u32, u32),
    Do,
    Dont,
}

struct Program {
    code: &'static str,
    cursor: usize,
}

impl Program {
    fn new(code: &'static str) -> Self {
        Self { code, cursor: 0 }
    }

    // Find the first instance of a string from a set of strings in the code.
    // It returns (text-index, cursor)
    fn start_parse_strings(&self, cursor: usize, texts: &[&'static str]) -> Option<(usize, usize)> {
        let cursors = texts
            .iter()
            .enumerate()
            // Convert to (text, optional cursor) tuple
            .map(|(i, text)| (i, self.start_parse_string(cursor, *text)));

        // Reduce to the minimum cursor
        let cursor = cursors.reduce(|(a_text, a_cursor), (b_text, b_cursor)| {
            if a_cursor.is_none() {
                return (b_text, b_cursor);
            }
            if b_cursor.is_none() {
                return (a_text, a_cursor);
            }
            if a_cursor.unwrap() < b_cursor.unwrap() {
                (a_text, a_cursor)
            } else if a_cursor.unwrap() == b_cursor.unwrap() {
                if texts[a_text].len() < texts[b_text].len() {
                    (b_text, b_cursor)
                } else {
                    (a_text, a_cursor)
                }
            } else {
                (b_text, b_cursor)
            }
        });

        if let Some((text, cursor)) = cursor {
            if let Some(cursor) = cursor {
                Some((text, cursor))
            } else {
                None
            }
        } else {
            None
        }
    }

    // Looks for an initial string and updates the cursor to the end of the string.
    fn start_parse_string(&self, cursor: usize, text: &'static str) -> Option<usize> {
        let cursor = self.code[cursor..]
            .find(text)
            .map(|i| i + text.len() + cursor);
        cursor
    }

    // Read a string at the actual cursor position.
    fn parse_string(&self, cursor: usize, text: &'static str) -> Option<usize> {
        let end = cursor + text.len();
        let end = usize::min(end, self.code.len());

        // Check if the string is at the cursor position
        if self.code[cursor..end].starts_with(text) {
            Some(end)
        } else {
            None
        }
    }

    fn parse_number(&self, cursor: usize, max_num: u32) -> Option<(usize, u32)> {
        // Create an iterator that will cover the next characters until a non-digit character is found.
        let iter = self.code[cursor..].chars().take_while(|c| c.is_digit(10));
        let num_digits = iter.count();

        if num_digits == 0 {
            return None;
        }

        // Convert the digits to a number.
        let num = self.code[cursor..cursor + num_digits].parse::<u32>().ok()?;

        if num > max_num {
            return None;
        }

        Some((cursor + num_digits, num))
    }
}

impl Iterator for Program {
    type Item = Instruction;

    fn next(&mut self) -> Option<Self::Item> {
        let keywords = ["mul(", "don't()", "do()"];

        // Find the next sequence of `mul(` followed by a number (maximum 3 digits) and a comma
        // followed by another number (maximum 3 digits) and a closing parenthesis.
        // Return the slice of the code that matches this pattern.
        loop {
            if let Some((text, cursor)) = self.start_parse_strings(self.cursor, &keywords[..]) {
                self.cursor = cursor;
                match text {
                    0 => {
                        let num_a = self.parse_number(self.cursor, 999);
                        if num_a.is_none() {
                            continue;
                        }
                        let (cursor, a) = num_a.unwrap();
                        self.cursor = cursor;

                        if let Some(cursor) = self.parse_string(self.cursor, ",") {
                            self.cursor = cursor;
                        } else {
                            continue;
                        }

                        let num_b = self.parse_number(self.cursor, 999);
                        if num_b.is_none() {
                            continue;
                        }
                        let (cursor, b) = num_b.unwrap();
                        self.cursor = cursor;

                        if let Some(cursor) = self.parse_string(self.cursor, ")") {
                            self.cursor = cursor;
                        } else {
                            continue;
                        }

                        return Some(Instruction::Mul(a, b));
                    }
                    1 => {
                        return Some(Instruction::Dont);
                    }

                    2 => {
                        return Some(Instruction::Do);
                    }

                    _ => {
                        return None;
                    }
                }
            } else {
                return None;
            }
        }
    }
}
