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
}
