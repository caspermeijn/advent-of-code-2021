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

pub struct Population {
    population: [usize; 10],
}

impl Population {
    pub fn parse(lines: std::str::Lines) -> Self {
        let mut population = Population {
            population: [0; 10],
        };

        for line in lines {
            for age in line.split(',') {
                let age: usize = age.parse().unwrap();
                population.population[age] += 1;
            }
        }

        population
    }

    pub fn age(&mut self, days: usize) {
        for _ in 0..days {
            let mut new_population = [0; 10];
            for (age, old_population) in self.population.iter().enumerate() {
                if age == 0 {
                    new_population[6] += old_population;
                    new_population[8] += old_population;
                } else {
                    new_population[age - 1] += old_population;
                }
            }
            self.population = new_population
        }
    }

    pub fn total(&self) -> usize {
        self.population.iter().sum()
    }
}
