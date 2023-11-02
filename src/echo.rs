// ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
// ┃ Copyright: (c) 2023, Mike 'PhiSyX' S. (https://github.com/PhiSyX)         ┃
// ┃ SPDX-License-Identifier: MPL-2.0                                          ┃
// ┃ ╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌ ┃
// ┃                                                                           ┃
// ┃  This Source Code Form is subject to the terms of the Mozilla Public      ┃
// ┃  License, v. 2.0. If a copy of the MPL was not distributed with this      ┃
// ┃  file, You can obtain one at https://mozilla.org/MPL/2.0/.                ┃
// ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛

use crate::layout;

// --------- //
// Structure //
// --------- //

pub struct Echo<'a>
{
	pub(crate) colorized: bool,
	pub(super) time: Option<chrono::DateTime<chrono::Local>>,
	pub(super) delimiter: String,
	pub(super) level: String,
	pub(super) record_level: log::Level,
	pub(super) table: &'a mut layout::GridLayout<'a>,
}

// -------------- //
// Implémentation //
// -------------- //

impl Echo<'_>
{
	/// `Stdout`: Affichage du log.
	pub(super) fn log(self, text: String)
	{
		if self.record_level == log::LevelFilter::Error {
			eprint!("{text}");
		} else {
			print!("{text}");
		}
	}
}
