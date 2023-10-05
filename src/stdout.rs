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
use std::sync::{Arc, Mutex};

use console::style;

use crate::echo::Echo;
use crate::{builder, layout, FilterFn, FormatFn, NO};

// --------- //
// Structure //
// --------- //

/// Logger Standard
pub struct Logger {
	pub colorized: bool,
	pub timestamp: bool,
	pub level: Option<log::LevelFilter>,
	pub format_fn: FormatFn,
	pub filter: LoggerFilter,
	pub cache: Arc<Mutex<HashMap<String, bool>>>,
}

pub struct LoggerFilter {
	pub callbacks: Vec<Box<FilterFn>>,
	pub dependencies: Vec<String>,
}

// -------------- //
// Implémentation // -> API Publique
// -------------- //

impl Logger {
	pub fn builder() -> builder::Builder {
		Default::default()
	}

	pub fn apply(self) -> Result<(), log::SetLoggerError> {
		let level = self.level.unwrap_or(log::LevelFilter::Off);

		log::set_max_level(level);
		if log::LevelFilter::Off == log::max_level() {
			log::set_logger(&NO)
		} else {
			log::set_boxed_logger(Box::new(self))
		}
	}
}

// -------------- //
// Implémentation // - Interface
// -------------- //

impl log::Log for Logger {
	/// On ne veut pas afficher les logs si le niveau est à
	/// [log::LevelFilter::Off].
	///
	/// Des conditions utilisateurs peuvent être utilisées pour filtrer les
	/// logs.
	fn enabled(&self, metadata: &log::Metadata) -> bool {
		let mut guard = self.cache.lock().expect("guard");
		metadata.level() != log::LevelFilter::Off
			&& (self.filter.callbacks.is_empty()
				|| self.filter.callbacks.iter().enumerate().any(
					|(idx, once_fn)| {
						let cache_key = format!(
							"{}_{}",
							self.filter.dependencies[idx],
							metadata.target()
						);

						if let Some(has) = guard.get(&cache_key) {
							return *has;
						}

						let is_ok = once_fn(metadata);
						guard.insert(cache_key, is_ok);
						is_ok
					},
				))
	}

	/// Affiche le log.
	//
	// FIXME(phisyx): améliorer les performances du logger. Le simple fait de
	// les afficher nous fait perdre un temps considérable.
	fn log(&self, record: &log::Record) {
		if !self.enabled(record.metadata()) {
			return;
		}

		let message = record.args();
		if message.to_string().trim().is_empty() {
			return;
		}

		let level = if self.colorized {
			match record.level() {
				| log::Level::Error => style("ERROR").red(),
				| log::Level::Warn => style(" WARN").yellow(),
				| log::Level::Info => style(" INFO").blue(),
				| log::Level::Debug => style("DEBUG").magenta(),
				| log::Level::Trace => style("TRACE").white(),
			}
			.to_string()
		} else {
			record.level().to_string()
		};

		let mut table = layout::GridLayout::default()
			.define_max_width(120)
			.without_boarder();

		let mut echo = Echo {
			time: if self.timestamp {
				Some(chrono::Local::now())
			} else {
				None
			},
			delimiter: if self.colorized {
				style("|").red()
			} else {
				style("|")
			}
			.to_string(),
			level,
			record_level: record.level(),
			table: &mut table,
		};

		let text = (self.format_fn)(&mut echo, message, record);

		echo.log(text);
	}

	fn flush(&self) {}
}
