use std::str::Lines;

pub struct MeaningfulLines<'a> {
    lines: Lines<'a>,
}

impl<'a> MeaningfulLines<'a> {
    pub fn new(lines: &'a str) -> Self {
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
