pub struct MovingTotal3<I>
where
    I: Iterator<Item = u32>,
{
    data: I,
    window: [u32; 3],
}

impl<'a, I> MovingTotal3<I>
where
    I: Iterator<Item = u32> + 'a,
{
    pub fn new(data: I) -> Self {
        let mut result = Self {
            data: data,
            window: [0; 3],
        };
        result.next();
        result.next();
        result
    }
}

impl<I> Iterator for MovingTotal3<I>
where
    I: Iterator<Item = u32>,
{
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // Shift all existing values
        self.window[..].rotate_right(1);
        // If a new value is available
        if let Some(new_value) = self.data.next() {
            // Place it in the first position
            self.window[0] = new_value;
            // Return the total
            Some(self.window.iter().sum())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::MovingTotal3;

    #[test]
    fn moving_total_3() {
        let text = include_str!("day1/input-example.txt");
        let numbers = text.lines().map(|line| line.parse::<u32>().unwrap());

        let mut totals = MovingTotal3::new(Box::new(numbers));

        assert_eq!(totals.next(), Some(607));
        assert_eq!(totals.next(), Some(618));
        assert_eq!(totals.next(), Some(618));
        assert_eq!(totals.next(), Some(617));
        assert_eq!(totals.next(), Some(647));
        assert_eq!(totals.next(), Some(716));
        assert_eq!(totals.next(), Some(769));
        assert_eq!(totals.next(), Some(792));
        assert_eq!(totals.next(), None);
    }
}
