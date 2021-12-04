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

use array2d::Array2D;

pub struct Board {
    board: Array2D<Option<u32>>,
}

impl Board {
    pub fn from_text(text: &str) -> Self {
        let board_numbers: Vec<Option<u32>> = text
            .split_whitespace()
            .map(|number| number.parse().ok())
            .collect();
        Board {
            board: Array2D::from_row_major(&board_numbers, 5, 5),
        }
    }

    pub fn mark_number(&mut self, number: u32) {
        for x in 0..5 {
            for y in 0..5 {
                if self.board[(x, y)] == Some(number) {
                    self.board[(x, y)] = None
                }
            }
        }
    }

    pub fn is_bingo(&self) -> bool {
        self.board
            .rows_iter()
            .any(|mut row| row.all(|optional_number| optional_number.is_none()))
            || self
                .board
                .columns_iter()
                .any(|mut column| column.all(|optional_number| optional_number.is_none()))
    }

    pub fn remaining_total(&self) -> u32 {
        self.board
            .elements_row_major_iter()
            .filter_map(|n| n.as_ref())
            .sum()
    }
}
