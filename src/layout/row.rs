// ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
// ┃ Copyright: (c) 2023, Mike 'PhiSyX' S. (https://github.com/PhiSyX)         ┃
// ┃ SPDX-License-Identifier: MPL-2.0                                          ┃
// ┃ ╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌ ┃
// ┃                                                                           ┃
// ┃  This Source Code Form is subject to the terms of the Mozilla Public      ┃
// ┃  License, v. 2.0. If a copy of the MPL was not distributed with this      ┃
// ┃  file, You can obtain one at https://mozilla.org/MPL/2.0/.                ┃
// ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛

use super::cell::{str_len, Cell};
use super::style::{Alignment, Position, Style};

// --------- //
// Structure //
// --------- //

#[derive(Debug)]
#[derive(Clone)]
pub struct Row<'d> {
	pub cells: Vec<Cell<'d>>,
	pub separator: bool,
}

// -------------- //
// Implémentation //
// -------------- //

impl<'d> Row<'d> {
	pub fn new(
		cells: impl IntoIterator<Item = impl Into<Cell<'d>>>,
	) -> Row<'d> {
		let mut row = Row {
			cells: vec![],
			separator: true,
		};

		cells
			.into_iter()
			.for_each(|entry| row.cells.push(entry.into()));

		row
	}
}

impl<'d> Row<'d> {
	pub(crate) fn format(&self, widths: &[usize], style: &Style) -> String {
		let mut temporary_buffer = String::new();
		let mut wrapped_cells = Vec::default();

		let mut spanned_columns = 0;
		let mut row_height = 0;

		for cell in &self.cells {
			let width = (0..cell.colspan)
				.fold(0, |width, j| width + widths[j + spanned_columns]);
			let wrapped_cell = cell.wrapped_content(width + cell.colspan - 1);
			row_height = core::cmp::max(row_height, wrapped_cell.len());
			wrapped_cells.push(wrapped_cell);
			spanned_columns += cell.colspan;
		}

		spanned_columns = 0;

		let mut lines = vec![String::new(); row_height];

		for (column_index, _) in
			wrapped_cells.iter().enumerate().take(widths.len())
		{
			if self.cells.len() > column_index {
				let cell = &self.cells[column_index];

				let cell_span = (0..cell.colspan)
					.fold(0, |n, c| n + widths[spanned_columns + c]);

				let callback_fn = |(row_index, row): (usize, &mut String)| {
					let padding = if wrapped_cells[column_index].len()
						> row_index
					{
						let str_width =
							str_len(&wrapped_cells[column_index][row_index]);

						let mut padding = 0;
						if cell_span >= str_width {
							padding += cell_span - str_width;

							if cell.colspan > 1 {
								padding += style.vertical.len_utf8()
									* (cell.colspan - 1);
							}
						}

						self.padding_string(
							padding,
							cell.alignment,
							&wrapped_cells[column_index][row_index],
						)
					} else {
						str::repeat(
							" ",
							widths[spanned_columns] * cell.colspan
								+ cell.colspan - 1,
						)
					};

					row.push_str(&format!("{}{padding}", style.vertical));
				};

				lines
					.iter_mut()
					.enumerate()
					.take(row_height)
					.for_each(callback_fn);

				spanned_columns += cell.colspan;
			} else {
				lines.iter_mut().take(row_height).for_each(|line| {
					line.push_str(&format!(
						"{}{}",
						style.vertical,
						str::repeat(" ", widths[spanned_columns])
					));
				});

				spanned_columns += 1;
			}

			if spanned_columns == widths.len() {
				break;
			}
		}

		lines.iter().for_each(|line| {
			temporary_buffer.push_str(line);
			temporary_buffer.push(style.vertical);
			temporary_buffer.push('\n');
		});

		temporary_buffer.pop();

		temporary_buffer
	}

	pub(crate) fn generate_separator(
		&self,
		widths: &[usize],
		style: &Style,
		row: Position,
		previous_separator: Option<&String>,
	) -> String {
		let mut temporary_buffer = String::new();

		let mut next_intersection = match self.cells.first() {
			| Some(cell) => cell.colspan,
			| None => 1,
		};

		temporary_buffer.push(style.start_position(row));

		let mut current_column = 0;

		for (index, width) in widths.iter().enumerate() {
			if index == next_intersection {
				temporary_buffer.push(style.intersect_position(row));

				current_column += 1;

				if self.cells.len() > current_column {
					next_intersection += self.cells[current_column].colspan;
				} else {
					next_intersection += 1;
				}
			} else if index > 0 {
				temporary_buffer.push(style.horizontal);
			}

			temporary_buffer
				.push_str(&str::repeat(&style.horizontal.to_string(), *width));
		}

		temporary_buffer.push(style.end_position(row));

		if let Some(prev) = previous_separator {
			let mut output = String::new();

			for pair in temporary_buffer.chars().zip(prev.chars()) {
				if pair.0 == style.left || pair.0 == style.right {
					output.push(pair.0);
				} else if pair.0 != style.horizontal
					|| pair.1 != style.horizontal
				{
					output.push(
						style.merge_intersection_position(pair.1, pair.0, row),
					);
				} else {
					output.push(style.horizontal);
				}
			}

			return output;
		}

		temporary_buffer
	}

	pub(crate) fn split_column(&self) -> Vec<(f32, usize)> {
		let callback_fn = |mut output: Vec<(f32, usize)>, cell: &Cell| {
			let value = cell.split_width();

			let min_w =
				(cell.min_width() as f32 / cell.colspan as f32) as usize;
			let add_1 = cell.min_width() as f32 % cell.colspan as f32 > 0.001;

			for i in 0..cell.colspan {
				if add_1 && i == cell.colspan - 1 {
					output.push((value + 1.0, min_w + 1));
				} else {
					output.push((value, min_w));
				}
			}

			output
		};

		let output = self.cells.iter().fold(Vec::default(), callback_fn);

		output
	}

	pub(crate) fn total_columns(&self) -> usize {
		self.cells.iter().map(|cell| cell.colspan).sum()
	}

	fn padding_string(
		&self,
		padding: usize,
		alignment: Alignment,
		text: &str,
	) -> String {
		match alignment {
			| Alignment::Left => {
				let r = str::repeat(" ", padding);
				format!("{text}{r}")
			}
			| Alignment::Right => {
				let l = str::repeat(" ", padding);
				format!("{l}{text}")
			}
			| Alignment::Center => {
				let half_padding = padding as f32 / 2.0;
				let l = str::repeat(" ", half_padding.ceil() as usize);
				let r = str::repeat(" ", half_padding.floor() as usize);
				format!("{l}{text}{r}",)
			}
		}
	}
}
