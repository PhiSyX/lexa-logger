// ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
// ┃ Copyright: (c) 2023, Mike 'PhiSyX' S. (https://github.com/PhiSyX)         ┃
// ┃ SPDX-License-Identifier: MPL-2.0                                          ┃
// ┃ ╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌ ┃
// ┃                                                                           ┃
// ┃  This Source Code Form is subject to the terms of the Mozilla Public      ┃
// ┃  License, v. 2.0. If a copy of the MPL was not distributed with this      ┃
// ┃  file, You can obtain one at https://mozilla.org/MPL/2.0/.                ┃
// ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛

use std::borrow::Cow;
use std::cmp;
use std::collections::HashSet;

use unicode_width::UnicodeWidthStr;

use super::style::Alignment;

// --------- //
// Structure //
// --------- //

#[derive(Debug)]
#[derive(Clone)]
pub struct Cell<'d>
{
	pub(crate) alignment: Alignment,
	pub(crate) colspan: usize,
	data: Cow<'d, str>,
	padding: bool,
}

// -------------- //
// Implémentation //
// -------------- //

impl<'d> Cell<'d>
{
	pub fn new(data: impl ToString) -> Self
	{
		Self {
			alignment: Alignment::Left,
			colspan: 1,
			data: Cow::from(data.to_string()),
			padding: true,
		}
	}

	pub fn with_alignment(mut self, alignment: Alignment) -> Self
	{
		self.alignment = alignment;
		self
	}
}

impl<'d> Cell<'d>
{
	pub(crate) fn width(&self) -> usize
	{
		self.wrapped_content(std::usize::MAX)
			.iter()
			.fold(0, |max, text| cmp::max(max, str_len(text)))
	}

	pub(crate) fn split_width(&self) -> f32
	{
		self.width() as f32 / self.colspan as f32
	}

	pub(crate) fn min_width(&self) -> usize
	{
		let max_char_width = self.data.chars().fold(0, |max, ch| cmp::max(max, ch.len_utf8()));

		if self.padding {
			max_char_width + ' '.len_utf8() * 2
		} else {
			max_char_width
		}
	}

	pub(crate) fn wrapped_content(&self, width: usize) -> Vec<String>
	{
		let pad_char = if self.padding { ' ' } else { '\0' };

		let hidden: HashSet<usize> = STRIP_ANSI_RE
			.find_iter(&self.data)
			.flat_map(|m| m.start()..m.end())
			.collect();

		let mut output = Vec::default();

		let mut temporary_buffer = String::default();

		temporary_buffer.push(pad_char);

		let mut byte_index = 0;

		for ch in self.data.chars() {
			if !hidden.contains(&byte_index)
				&& (str_len(&temporary_buffer) >= width - pad_char.len_utf8() || ch == '\n')
			{
				temporary_buffer.push(pad_char);
				output.push(temporary_buffer);
				temporary_buffer = String::new();
				temporary_buffer.push(pad_char);

				if ch == '\n' {
					byte_index += 1;
					continue;
				}
			}

			byte_index += ch.len_utf8();

			temporary_buffer.push(ch);
		}

		temporary_buffer.push(pad_char);

		output.push(temporary_buffer);

		output
	}
}

pub(crate) fn str_len(string: &str) -> usize
{
	let stripped = STRIP_ANSI_RE.replace_all(string, "");
	stripped.width()
}

lazy_static::lazy_static! {
	// STRIP_ANSI_RE is lifted from the "console" crate.
	// Copyright 2017 Armin Ronacher <armin.ronacher@active-4.com>. MIT License.
	static ref STRIP_ANSI_RE: regex::Regex = regex::Regex::new(
		r"[\x1b\x9b][\[()#;?]*(?:[0-9]{1,4}(?:;[0-9]{0,4})*)?[0-9A-PRZcf-nqry=><]"
	)
	.unwrap();
}
