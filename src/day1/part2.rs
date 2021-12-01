use crate::utils::MovingTotal3;

pub fn solve(lines: std::str::Lines) -> String {
    let numbers = lines.map(|line| line.parse::<u32>().unwrap());
    let totals = MovingTotal3::new(Box::new(numbers));
    let mut count = 0;
    let mut previous_number = u32::MAX;
    for number in totals {
        if number > previous_number {
            count += 1;
        }
        previous_number = number;
    }
    return format!("{}", count);
}

#[cfg(test)]
mod tests {
    use crate::day1::part2::solve;

    #[test]
    fn example() {
        let text = include_str!("input-example.txt");
        let result = solve(text.lines());
        assert_eq!(result, "5")
    }
}
