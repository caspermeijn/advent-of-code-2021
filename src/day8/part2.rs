/* Copyright (C) 2021 Casper Meijn <casper@meijn.net>
 * SPDX-License-Identifier: GPL-3.0-or-later
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

fn get_digit_by_unique_len(signal_patterns: &Vec<String>, len: usize) -> String {
    signal_patterns
        .iter()
        .filter(|s| s.len() == len)
        .next()
        .unwrap()
        .clone()
}

fn get_digit_must_contain(
    signal_patterns: &Vec<String>,
    must_contain: String,
    exact_len: usize,
) -> String {
    signal_patterns
        .iter()
        .filter(|s| s.len() == exact_len)
        .filter(|s| must_contain.chars().all(|c1| s.chars().any(|c2| c1 == c2)))
        .next()
        .unwrap()
        .clone()
}

fn get_digit_must_miss(
    signal_patterns: &Vec<String>,
    must_miss: &String,
    exact_len: usize,
) -> String {
    signal_patterns
        .iter()
        .filter(|s| s.len() == exact_len)
        .filter(|s| must_miss.chars().any(|c1| s.chars().all(|c2| c1 != c2)))
        .next()
        .unwrap()
        .clone()
}

fn get_digit_fits_in(signal_patterns: &Vec<String>, fits_in: &String, exact_len: usize) -> String {
    signal_patterns
        .iter()
        .filter(|s| s.len() == exact_len)
        .filter(|s| s.chars().all(|c1| fits_in.chars().any(|c2| c1 == c2)))
        .next()
        .unwrap()
        .clone()
}

fn get_digit_fits_in_exclude(
    signal_patterns: &Vec<String>,
    fits_in: &String,
    exclude: &String,
    exact_len: usize,
) -> String {
    signal_patterns
        .iter()
        .filter(|s| s.len() == exact_len)
        .filter(|&s| s != exclude)
        .filter(|s| s.chars().all(|c1| fits_in.chars().any(|c2| c1 == c2)))
        .next()
        .unwrap()
        .clone()
}

fn get_unique_signal(does_contain: &String, does_not_contain: &String) -> char {
    does_contain
        .chars()
        .filter(|&c1| !does_not_contain.chars().any(|c2| c1 == c2))
        .next()
        .unwrap()
}

fn get_digit_is_not(
    signal_patterns: &Vec<String>,
    is_not: &Vec<&String>,
    exact_len: usize,
) -> String {
    signal_patterns
        .iter()
        .filter(|s| s.len() == exact_len)
        .filter(|s1| is_not.iter().all(|s2| s1 != s2))
        .next()
        .unwrap()
        .clone()
}

fn match_digit(digits: &[String; 10], pattern: &str) -> usize {
    digits
        .iter()
        .enumerate()
        .filter(|(i, s)| s.len() == pattern.len())
        .filter(|(i, s)| s.chars().all(|c1| pattern.chars().any(|c2| c1 == c2)))
        .next()
        .unwrap()
        .0
}

fn concat(s1: &String, s2: &String) -> String {
    s1.clone() + &s2
}

#[derive(Default)]
struct SevenSegment {
    digits: [String; 10],
    number: usize,
}

impl SevenSegment {
    pub fn parse(text: &str) -> Self {
        let mut digits: [String; 10] = Default::default();
        let (signal_patterns, digit_patterns) = text.split_once(" | ").unwrap();
        let signal_patterns: Vec<String> = signal_patterns
            .split_whitespace()
            .map(String::from)
            .collect();

        digits[1] = get_digit_by_unique_len(&signal_patterns, 2);
        digits[7] = get_digit_by_unique_len(&signal_patterns, 3);
        digits[4] = get_digit_by_unique_len(&signal_patterns, 4);
        digits[8] = get_digit_by_unique_len(&signal_patterns, 7);

        digits[9] = get_digit_must_contain(&signal_patterns, concat(&digits[4], &digits[7]), 6);
        digits[6] = get_digit_must_miss(&signal_patterns, &digits[1], 6);
        digits[0] = get_digit_is_not(&signal_patterns, &vec![&digits[9], &digits[6]], 6);

        digits[5] = get_digit_fits_in(&signal_patterns, &digits[6], 5);
        digits[3] = get_digit_fits_in_exclude(&signal_patterns, &digits[9], &digits[5], 5);
        digits[2] = get_digit_is_not(&signal_patterns, &vec![&digits[5], &digits[3]], 5);

        let digit_patterns: Vec<&str> = digit_patterns.split_whitespace().collect();
        let number = match_digit(&digits, digit_patterns[0]) * 1000
            + match_digit(&digits, digit_patterns[1]) * 100
            + match_digit(&digits, digit_patterns[2]) * 10
            + match_digit(&digits, digit_patterns[3]) * 1;

        SevenSegment { digits, number }
    }
}

pub fn solve(lines: std::str::Lines) -> usize {
    lines
        .map(|line| {
            let seven_segment = SevenSegment::parse(line);
            seven_segment.number
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::day8::{part2::solve, part2::SevenSegment};

    #[test]
    fn parse() {
        let text =
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
        let result = SevenSegment::parse(text);

        assert_eq!(result.digits[1], "ab");
        assert_eq!(result.digits[4], "eafb");
        assert_eq!(result.digits[7], "dab");
        assert_eq!(result.digits[8], "acedgfb");
        assert_eq!(result.digits[9], "cefabd");
        assert_eq!(result.digits[6], "cdfgeb");
        assert_eq!(result.digits[0], "cagedb");
        assert_eq!(result.digits[5], "cdfbe");
        assert_eq!(result.digits[2], "gcdfa");
        assert_eq!(result.digits[3], "fbcad");
        assert_eq!(result.number, 5353);
    }

    #[test]
    fn example() {
        let text = include_str!("input-example.txt");
        let result = solve(text.lines());
        assert_eq!(result, 61229)
    }
}
