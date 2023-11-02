// ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
// ┃ Copyright: (c) 2023, Mike 'PhiSyX' S. (https://github.com/PhiSyX)         ┃
// ┃ SPDX-License-Identifier: MPL-2.0                                          ┃
// ┃ ╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌ ┃
// ┃                                                                           ┃
// ┃  This Source Code Form is subject to the terms of the Mozilla Public      ┃
// ┃  License, v. 2.0. If a copy of the MPL was not distributed with this      ┃
// ┃  file, You can obtain one at https://mozilla.org/MPL/2.0/.                ┃
// ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛

use std::collections::HashMap;

use super::cell::Cell;
use super::row::Row;
use super::style::{Alignment, Position, Style, STYLE_BLANK};

// --------- //
// Structure //
// --------- //

#[derive(Clone, Debug)]
pub struct GridLayout<'d>
{
	rows: Vec<Row<'d>>,

	style: Style,

	widths: HashMap<usize, usize>,
	max_width: usize,

	separate_rows: bool,
	boarder: GridBoarder,
}

#[derive(Clone, Debug)]
pub struct GridBoarder
{
	top: bool,
	bottom: bool,
}

// -------------- //
// Implémentation //
// -------------- //

impl<'d> GridLayout<'d>
{
	pub fn new() -> Self
	{
		Self {
			rows: Vec::new(),
			style: STYLE_BLANK,

			widths: HashMap::new(),
			max_width: std::usize::MAX,

			separate_rows: true,

			boarder: GridBoarder {
				top: true,
				bottom: true,
			},
		}
	}

	pub fn with_style(mut self, style: Style) -> Self
	{
		self.style = style;
		self
	}

	pub fn without_boarder(mut self) -> Self
	{
		self.boarder.top = false;
		self.boarder.bottom = false;
		self
	}

	pub fn define_max_width(mut self, width: usize) -> Self
	{
		self.max_width = width;
		self
	}

	pub fn add_line(&mut self, cells: impl IntoIterator<Item = impl Into<Cell<'d>>>)
	{
		let row = Row::new(cells);
		self.rows.push(row);
	}

	pub fn render(&self) -> String
	{
		let mut print_buffer = String::new();

		if self.rows.is_empty() {
			return print_buffer;
		}

		let max_widths = self.calculate_max_widths();
		let mut previous_separator = None;
		for index in 0..self.rows.len() {
			let row_pos = if index == 0 { Position::First } else { Position::Middle };

			let separator =
				self.rows[index].generate_separator(&max_widths, &self.style, row_pos, previous_separator.as_ref());

			previous_separator.replace(separator.clone());

			if self.rows[index].separator && ((index == 0 && self.boarder.top) || index != 0 && self.separate_rows) {
				Self::add_newline_to_buffer(&mut print_buffer, separator);
			}

			Self::add_newline_to_buffer(&mut print_buffer, &self.rows[index].format(&max_widths, &self.style));
		}

		if self.boarder.bottom {
			let separator =
				self.rows
					.last()
					.unwrap()
					.generate_separator(&max_widths, &self.style, Position::Last, None);

			Self::add_newline_to_buffer(&mut print_buffer, separator);
		}

		print_buffer
	}
}

impl<'d> GridLayout<'d>
{
	fn calculate_max_widths(&self) -> Vec<usize>
	{
		let total_columns = self
			.rows
			.iter()
			.fold(0, |n, row| core::cmp::max(row.total_columns(), n));

		let (_, mut max_widths) =
			self.rows
				.iter()
				.fold((vec![0; total_columns], vec![0; total_columns]), |acc, row| {
					let column_widths = row.split_column();

					(0..column_widths.len()).fold((acc.0, acc.1), |(mut min, mut max), index| {
						min[index] = core::cmp::max(min[index], column_widths[index].1);

						let mut max_width = *self.widths.get(&index).unwrap_or(&self.max_width);

						max_width = core::cmp::max(min[index], max_width);

						max[index] =
							core::cmp::min(max_width, core::cmp::max(max[index], column_widths[index].0 as usize));

						(min, max)
					})
				});

		self.rows.iter().for_each(|row| {
			row.cells.iter().fold(0, |mut column_index, cell| {
				let total_col_width =
					(column_index..column_index + cell.colspan).fold(0, |total, index| total + max_widths[index]);

				if cell.width() != total_col_width
					&& cell.alignment == Alignment::Center
					&& total_col_width as f32 % 2.0 <= 0.001
				{
					let mut max_col_width = self.max_width;
					if let Some(specific_width) = self.widths.get(&column_index) {
						max_col_width = *specific_width;
					}

					if max_widths[column_index] < max_col_width {
						max_widths[column_index] += 1;
					}
				}

				if cell.colspan > 1 {
					column_index += cell.colspan - 1;
				} else {
					column_index += 1;
				}

				column_index
			});
		});

		max_widths
	}

	fn add_newline_to_buffer(buffer: &mut String, line: impl AsRef<str>)
	{
		buffer.push_str(&format!("{}\n", line.as_ref()));
	}
}

// -------------- //
// Implémentation // -> Interface
// -------------- //

impl<'d> Default for GridLayout<'d>
{
	fn default() -> Self
	{
		GridLayout::new()
	}
}

impl<'d> ToString for GridLayout<'d>
{
	fn to_string(&self) -> String
	{
		self.render()
	}
}
