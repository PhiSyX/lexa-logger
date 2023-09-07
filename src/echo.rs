// ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
// ┃                           __    _            ____  ______                 ┃
// ┃                    ____  / /_  (_)______  __/ __ \/ ____/                 ┃
// ┃                   / __ \/ __ \/ / ___/ / / / /_/ / /                      ┃
// ┃                  / /_/ / / / / (__  ) /_/ / _, _/ /___                    ┃
// ┃                 / .___/_/ /_/_/____/\__, /_/ |_|\____/                    ┃
// ┃                /_/                 /____/                                 ┃
// ┣━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┫
// ┃ Copyright: (c) 2023, Mike 'PhiSyX' S. (https://github.com/PhiSyX)         ┃
// ┃ SPDX-License-Identifier: MPL-2.0                                          ┃
// ┃ ╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌ ┃
// ┃                                                                           ┃
// ┃  This Source Code Form is subject to the terms of the Mozilla Public      ┃
// ┃  License, v. 2.0. If a copy of the MPL was not distributed with this      ┃
// ┃  file, You can obtain one at https://mozilla.org/MPL/2.0/.                ┃
// ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛

#[cfg(feature = "tui")]
use tokio::sync::mpsc;

use crate::layout;

// --------- //
// Structure //
// --------- //

pub struct Echo<'a> {
	pub(super) time: Option<chrono::DateTime<chrono::Local>>,
	pub(super) delimiter: String,
	pub(super) level: String,
	pub(super) record_level: log::Level,
	pub(super) table: &'a mut layout::GridLayout<'a>,
}

#[cfg(feature = "tui")]
#[derive(Debug)]
pub struct Entry {
	pub level: log::LevelFilter,
	pub target: String,
	pub args: String,
}

// ---- //
// Type //
// ---- //

#[cfg(feature = "tui")]
pub type LoggerWriter = mpsc::UnboundedSender<Entry>;
#[cfg(feature = "tui")]
pub type LoggerReader = mpsc::UnboundedReceiver<Entry>;

// -------------- //
// Implémentation //
// -------------- //

impl Echo<'_> {
	/// `Stdout`: Affichage du log.
	pub(super) fn log(self, text: String) {
		if self.record_level == log::LevelFilter::Error {
			eprint!("{text}");
		} else {
			print!("{text}");
		}
	}
}

#[cfg(feature = "tui")]
impl Entry {
	/// `TUI`: le style d'un log.
	pub(super) fn style(&self) -> terminal::tui::style::Style {
		use terminal::tui::style::{Color, Style};

		match self.level {
			| log::LevelFilter::Off => Style::default(),
			| log::LevelFilter::Error => Style::default().fg(Color::Red),
			| log::LevelFilter::Warn => Style::default().fg(Color::Yellow),
			| log::LevelFilter::Info => Style::default().fg(Color::Blue),
			| log::LevelFilter::Debug => Style::default().fg(Color::Magenta),
			| log::LevelFilter::Trace => Style::default().fg(Color::White),
		}
	}
}

// -------------- //
// Implémentation // -> Interface
// -------------- //

#[cfg(feature = "tui")]
impl From<&log::Record<'_>> for Entry {
	fn from(record: &log::Record) -> Self {
		Self {
			level: record.level().to_level_filter(),
			target: record.target().to_string(),
			args: record.args().to_string(),
		}
	}
}
