use std::fmt::Debug;
use std::vec;

pub struct ExpandingWindow<'data> {
    data: &'data str,
    window_size: usize,
}

impl ExpandingWindow<'_> {
    pub fn new(data: &str) -> ExpandingWindow {
        ExpandingWindow {
            data,
            window_size: 0,
        }
    }
}

impl<'data> Iterator for ExpandingWindow<'data> {
    type Item = &'data str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.window_size == self.data.len() {
            None
        } else {
            self.window_size += 1;
            Some(&self.data[..self.window_size])
        }
    }
}

pub struct CombinationIterator<'data, T> {
    data: &'data [T],
    indices: Vec<usize>,
    complete: bool,
}

impl<'data, T> CombinationIterator<'data, T> {
    pub fn new(data: &'data [T], size: usize) -> CombinationIterator<'data, T> {
        let indices = vec![0; size];

        CombinationIterator {
            data,
            indices,
            complete: false,
        }
    }
}

impl<'data, T> Iterator for CombinationIterator<'data, T>
where
    T: Copy + Debug,
{
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.complete {
            return None;
        }

        let result = Some(
            self.indices
                .iter()
                .map(|&i| self.data[i])
                .collect::<Vec<T>>(),
        );

        let mut k = self.indices.len() - 1;

        // Increment the last index and propagate the carry if necessary
        self.indices[k] += 1;
        loop {
            if self.indices[k] < self.data.len() {
                break;
            }

            if k == 0 {
                self.complete = true;
                break;
            }

            self.indices[k] = 0;
            k -= 1;
            self.indices[k] += 1;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expanding_window() {
        let data = "hello";
        let mut window = ExpandingWindow::new(data);

        assert_eq!(window.next(), Some("h"));
        assert_eq!(window.next(), Some("he"));
        assert_eq!(window.next(), Some("hel"));
        assert_eq!(window.next(), Some("hell"));
        assert_eq!(window.next(), Some("hello"));
        assert_eq!(window.next(), None);
    }

    #[test]
    fn test_gen_combinations() {
        let data = vec![1, 2];

        let mut combinations = CombinationIterator::new(&data, 3);
        assert_eq!(combinations.next(), Some(vec![1, 1, 1]));
        assert_eq!(combinations.next(), Some(vec![1, 1, 2]));
        assert_eq!(combinations.next(), Some(vec![1, 2, 1]));
        assert_eq!(combinations.next(), Some(vec![1, 2, 2]));
        assert_eq!(combinations.next(), Some(vec![2, 1, 1]));
        assert_eq!(combinations.next(), Some(vec![2, 1, 2]));
        assert_eq!(combinations.next(), Some(vec![2, 2, 1]));
        assert_eq!(combinations.next(), Some(vec![2, 2, 2]));
        assert_eq!(combinations.next(), None);
    }
}
