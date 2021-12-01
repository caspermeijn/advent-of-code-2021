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

pub fn solve(lines: std::str::Lines) -> String {
    let numbers = lines.map(|line| line.parse::<u32>().unwrap());
    let mut count = 0;
    let mut previous_number = u32::MAX;
    for number in numbers {
        if number > previous_number {
            count += 1;
        }
        previous_number = number;
    }
    return format!("{}", count);
}

#[cfg(test)]
mod tests {
    use crate::day1::part1::solve;

    #[test]
    fn example() {
        let text = include_str!("input-example.txt");
        let result = solve(text.lines());
        assert_eq!(result, "7")
    }
}
