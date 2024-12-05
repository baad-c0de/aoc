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

pub struct FileSections<'a> {
    lines: &'a str,
    cursor: usize,
}

impl<'a> FileSections<'a> {
    pub fn new(lines: &'a str) -> Self {
        FileSections { lines, cursor: 0 }
    }
}

impl<'a> Iterator for FileSections<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor >= self.lines.len() {
            return None;
        }

        let section = &self.lines[self.cursor..];
        let end_section = section.find("\n\n");

        match end_section {
            Some(end_section) => {
                let section = &section[..end_section];
                self.cursor += end_section + 2;
                Some(section)
            }
            None => {
                self.cursor = self.lines.len();

                if section.ends_with('\n') {
                    let section = &section[..section.len() - 1];
                    Some(section)
                } else {
                    Some(section)
                }
            }
        }
    }
}
