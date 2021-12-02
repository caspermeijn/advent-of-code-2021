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

fn main() {
    println!("Hello, world!");

    let text = include_str!("day1/input-puzzle.txt");
    let result = advent_of_code_2021::day1::part1::solve(text.lines());
    println!("Puzzle 1a: {}", result);
    let result = advent_of_code_2021::day1::part2::solve(text.lines());
    println!("Puzzle 1b: {}", result);

    let text = include_str!("day2/input-puzzle.txt");
    let result = advent_of_code_2021::day2::part1::solve(text.lines());
    println!("Puzzle 2a: {}", result);
    let result = advent_of_code_2021::day2::part2::solve(text.lines());
    println!("Puzzle 2b: {}", result);
}
