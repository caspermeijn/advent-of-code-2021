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

use std::{
    cmp::{max, min},
    ops::RangeInclusive,
};

use array2d::Array2D;

fn better_range(n1: usize, n2: usize) -> RangeInclusive<usize> {
    min(n1, n2)..=max(n1, n2)
}

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn parse(text: &str) -> Self {
        let parts = text.split_once(",").unwrap();
        Point {
            x: parts.0.trim().parse().unwrap(),
            y: parts.1.trim().parse().unwrap(),
        }
    }
}

#[derive(Debug)]
struct Line {
    p1: Point,
    p2: Point,
}

impl Line {
    fn parse(text: &str) -> Self {
        let parts = text.split_once("->").unwrap();
        Line {
            p1: Point::parse(parts.0),
            p2: Point::parse(parts.1),
        }
    }

    fn is_horizonal(&self) -> bool {
        self.p1.x == self.p2.x
    }

    fn is_vertical(&self) -> bool {
        self.p1.y == self.p2.y
    }

    fn is_horizonal_or_vertical(&self) -> bool {
        self.is_horizonal() || self.is_vertical()
    }

    fn points(&self) -> Vec<Point> {
        if self.is_horizonal() {
            better_range(self.p1.y, self.p2.y)
                .map(|y| Point { x: self.p1.x, y })
                .collect()
        } else if self.is_vertical() {
            better_range(self.p1.x, self.p2.x)
                .map(|x| Point { x, y: self.p1.y })
                .collect()
        } else {
            panic!()
        }
    }
}

pub fn solve(lines: std::str::Lines) -> usize {
    let lines = lines
        .map(Line::parse)
        .filter(Line::is_horizonal_or_vertical);

    let points = lines.flat_map(|line| line.points());

    let mut field = Array2D::filled_with(0, 1000, 1000);
    for point in points {
        field[(point.y, point.x)] += 1;
    }

    field
        .elements_row_major_iter()
        .filter(|&&cell| cell >= 2)
        .count()
}

#[cfg(test)]
mod tests {
    use crate::day5::part1::solve;

    #[test]
    fn example() {
        let text = include_str!("input-example.txt");
        let result = solve(text.lines());
        assert_eq!(result, 5)
    }
}
