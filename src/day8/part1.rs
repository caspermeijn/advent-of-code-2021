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

pub fn solve(lines: std::str::Lines) -> usize {
    let mut count = 0;
    for line in lines {
        let (_signal_patterns, digit_patterns) = line.split_once(" | ").unwrap();
        for digit_pattern in digit_patterns.split_whitespace() {
            match digit_pattern.len() {
                2 => count += 1, // Digit 1
                4 => count += 1, // Digit 4
                3 => count += 1, // Digit 7
                7 => count += 1, // Digit 8
                _ => {}
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use crate::day8::part1::solve;

    #[test]
    fn example() {
        let text = include_str!("input-example.txt");
        let result = solve(text.lines());
        assert_eq!(result, 26)
    }
}
