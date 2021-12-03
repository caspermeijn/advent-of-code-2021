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

pub fn solve(lines: std::str::Lines) -> u32 {
    let mut zeros_count = [0; 12];
    let mut ones_count = [0; 12];

    for line in lines {
        let line_len = line.len();
        for (index, char) in line.chars().enumerate() {
            let index = 12 - line_len + index;
            if char == '0' {
                zeros_count[index] += 1;
            } else {
                ones_count[index] += 1;
            }
        }
    }

    let gamma = zeros_count
        .iter()
        .zip(ones_count.iter())
        .map(|(zeros, ones)| {
            if zeros >= ones {
                String::from("0")
            } else {
                String::from("1")
            }
        })
        .collect::<Vec<String>>()
        .join("");

    let gamma = u32::from_str_radix(&gamma, 2).unwrap();

    let epsilon = zeros_count
        .iter()
        .zip(ones_count.iter())
        .map(|(zeros, ones)| {
            if zeros <= ones {
                String::from("0")
            } else {
                String::from("1")
            }
        })
        .collect::<Vec<String>>()
        .join("");

    let epsilon = u32::from_str_radix(&epsilon, 2).unwrap();

    gamma * epsilon
}

#[cfg(test)]
mod tests {
    use crate::day3::part1::solve;

    #[test]
    fn example() {
        let text = include_str!("input-example.txt");
        let result = solve(text.lines());
        assert_eq!(result, 198)
    }
}
