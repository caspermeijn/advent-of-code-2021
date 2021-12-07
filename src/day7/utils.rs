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

pub const fn abs_diff(one: usize, other: usize) -> usize {
    if one < other {
        other.wrapping_sub(one)
    } else {
        one.wrapping_sub(other)
    }
}

pub struct Aligment {
    alignments: Vec<usize>,
}

impl Aligment {
    pub fn parse(text: &str) -> Self {
        Self {
            alignments: text.split(',').map(|n| n.parse().unwrap()).collect(),
        }
    }

    pub fn costs(&self, new_course: usize) -> usize {
        self.alignments
            .iter()
            .map(|&old_course| abs_diff(old_course, new_course))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::day7::utils::Aligment;

    #[test]
    fn example() {
        let text = include_str!("input-example.txt");
        let alignment = Aligment::parse(text);
        assert_eq!(alignment.costs(2), 37);
        assert_eq!(alignment.costs(1), 41);
        assert_eq!(alignment.costs(3), 39);
        assert_eq!(alignment.costs(10), 71);
    }
}
