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

use std::panic;

struct Position {
    horizontal: u32,
    depth: u32,
    aim: u32,
}

pub fn solve(lines: std::str::Lines) -> u32 {
    let mut postition = Position {
        horizontal: 0,
        depth: 0,
        aim: 0,
    };
    for line in lines {
        if let Some((command, amount)) = line.split_once(" ") {
            let amount: u32 = amount.parse().unwrap();
            match command {
                "forward" => {
                    postition.horizontal += amount;
                    postition.depth += amount * postition.aim;
                }
                "down" => postition.aim += amount,
                "up" => postition.aim -= amount,
                _ => panic!(),
            }
        }
    }
    postition.horizontal * postition.depth
}

#[cfg(test)]
mod tests {
    use crate::day2::part2::solve;

    #[test]
    fn example() {
        let text = include_str!("input-example.txt");
        let result = solve(text.lines());
        assert_eq!(result, 900)
    }
}
