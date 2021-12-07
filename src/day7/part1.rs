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

use crate::day7::utils::Aligment;

pub fn solve(mut lines: std::str::Lines) -> usize {
    let alignment = Aligment::parse(lines.next().unwrap());

    let best_course = (0..1000)
        .min_by_key(|&new_course| alignment.costs(new_course))
        .unwrap();

    alignment.costs(best_course)
}

#[cfg(test)]
mod tests {
    use crate::day7::part1::solve;

    #[test]
    fn example() {
        let text = include_str!("input-example.txt");
        let result = solve(text.lines());
        assert_eq!(result, 37)
    }
}
