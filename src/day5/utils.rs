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

fn better_range(n1: usize, n2: usize) -> RangeInclusive<usize> {
    min(n1, n2)..=max(n1, n2)
}

#[derive(Debug)]
pub struct Point {
    pub x: usize,
    pub y: usize,
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
pub struct Line {
    p1: Point,
    p2: Point,
}

impl Line {
    pub fn parse(text: &str) -> Self {
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

    pub fn is_horizonal_or_vertical(&self) -> bool {
        self.is_horizonal() || self.is_vertical()
    }

    fn is_diagonal_positive(&self) -> bool {
        self.p1.x as isize - self.p2.x as isize == self.p1.y as isize - self.p2.y as isize
    }

    fn is_diagonal_negative(&self) -> bool {
        self.p1.x as isize - self.p2.x as isize == self.p2.y as isize - self.p1.y as isize
    }

    pub fn points(&self) -> Vec<Point> {
        if self.is_horizonal() {
            better_range(self.p1.y, self.p2.y)
                .map(|y| Point { x: self.p1.x, y })
                .collect()
        } else if self.is_vertical() {
            better_range(self.p1.x, self.p2.x)
                .map(|x| Point { x, y: self.p1.y })
                .collect()
        } else if self.is_diagonal_positive() {
            let diff = self.p2.x as isize - self.p1.x as isize;
            if diff < 0 {
                let abs_diff = (0 - diff) as usize;
                (0..=abs_diff)
                    .map(|i| Point {
                        x: self.p1.x - i,
                        y: self.p1.y - i,
                    })
                    .collect()
            } else {
                let diff = diff as usize;
                (0..=diff)
                    .map(|i| Point {
                        x: self.p1.x + i,
                        y: self.p1.y + i,
                    })
                    .collect()
            }
        } else if self.is_diagonal_negative() {
            let diff = self.p1.x as isize - self.p2.x as isize;
            if diff < 0 {
                let abs_diff = (0 - diff) as usize;
                (0..=abs_diff)
                    .map(|i| Point {
                        x: self.p1.x + i,
                        y: self.p1.y - i,
                    })
                    .collect()
            } else {
                let diff = diff as usize;
                (0..=diff)
                    .map(|i| Point {
                        x: self.p1.x - i,
                        y: self.p1.y + i,
                    })
                    .collect()
            }
        } else {
            panic!(
                "Getting point of this type of line is not supported: {:?}",
                self
            )
        }
    }
}
