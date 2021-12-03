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

#[derive(Debug, Clone, Copy)]
struct Number {
    orig_number: u32,
    processed: u32,
}

impl Number {
    fn new(line: &str) -> Number {
        let number = u32::from_str_radix(&line, 2).unwrap();
        Number {
            orig_number: number,
            processed: number.reverse_bits() >> 32 -line.len(),
        }
    }

    fn get_current_bit(&self) -> u32 {
        self.processed & 0x0001
    }

    fn shift_out_one_bit(mut self) -> Self {
        self.processed = self.processed >> 1;
        self
    }
}

fn reduce_list_oxygen(list: &Vec<Number>) -> Vec<Number> {
    let ones_count = list
        .iter()
        .filter(|number| number.get_current_bit() == 1)
        .count();
    let most_common_bit = if ones_count < (list.len() + 1) / 2 { 0 } else { 1 };

    let new_list = list
        .iter()
        .filter(|number| number.get_current_bit() == most_common_bit)
        .map(|number| number.shift_out_one_bit())
        .collect();

    new_list
}

fn find_oxygen_rating(numbers: &Vec<Number>) -> u32 {
    let mut list = reduce_list_oxygen(numbers);
    while list.len() > 1 {
        list = reduce_list_oxygen(&list);
    }
    list[0].orig_number
}

fn reduce_list_co2(list: &Vec<Number>) -> Vec<Number> {
    let zeros_count = list
        .iter()
        .filter(|number| number.get_current_bit() == 0)
        .count();
    let least_common_bit = if zeros_count > (list.len()) / 2 { 1 } else { 0 };

    let new_list = list
        .iter()
        .filter(|number| number.get_current_bit() == least_common_bit)
        .map(|number| number.shift_out_one_bit())
        .collect();

    new_list
}

fn find_co2_rating(numbers: &Vec<Number>) -> u32 {
    let mut list = reduce_list_co2(numbers);
    while list.len() > 1 {
        list = reduce_list_co2(&list);
    }
    list[0].orig_number
}

pub fn solve(lines: std::str::Lines) -> u32 {
    let numbers: Vec<Number> = lines
        .map(|line| Number::new(line))
        .collect();

        let oxygen = find_oxygen_rating(&numbers);

        let co2 = find_co2_rating(&numbers);

    oxygen * co2
}

#[cfg(test)]
mod tests {
    use crate::day3::part2::solve;

    #[test]
    fn example() {
        let text = include_str!("input-example.txt");
        let result = solve(text.lines());
        assert_eq!(result, 230)
    }
}
