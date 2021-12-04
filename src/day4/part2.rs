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

use crate::day4::utils::Board;

pub fn solve(text: &str) -> u32 {
    let mut sections = text.split("\n\n");
    let numbers_drawn = sections
        .next()
        .unwrap()
        .split(',')
        .map(|number| number.parse::<u32>().unwrap());

    let mut boards: Vec<Board> = sections.map(|section| Board::from_text(section)).collect();

    for number_drawn in numbers_drawn {
        for board in &mut boards {
            board.mark_number(number_drawn);
        }

        if boards.len() == 1 {
            let board = &boards[0];
            if board.is_bingo() {
                return number_drawn * board.remaining_total();
            }
        } else {
            boards = boards
                .into_iter()
                .filter(|board| !board.is_bingo())
                .collect();
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use crate::day4::part2::solve;

    #[test]
    fn example() {
        let text = include_str!("input-example.txt");
        let result = solve(text);
        assert_eq!(result, 1924)
    }
}
