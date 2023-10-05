// ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
// ┃ Copyright: (c) 2023, Mike 'PhiSyX' S. (https://github.com/PhiSyX)         ┃
// ┃ SPDX-License-Identifier: MPL-2.0                                          ┃
// ┃ ╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌ ┃
// ┃                                                                           ┃
// ┃  This Source Code Form is subject to the terms of the Mozilla Public      ┃
// ┃  License, v. 2.0. If a copy of the MPL was not distributed with this      ┃
// ┃  file, You can obtain one at https://mozilla.org/MPL/2.0/.                ┃
// ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛

use console::style;

use crate::{FilterFn, FormatFn};

// --------- //
// Structure //
// --------- //

#[derive(Default)]
pub struct Builder {
	colorized: bool,
	timestamp: bool,
	level: Option<log::LevelFilter>,
	format_fn: Option<FormatFn>,
	filter: BuilderFilter,
}

#[derive(Default)]
pub struct BuilderFilter {
	callbacks: Vec<Box<FilterFn>>,
	dependencies: Vec<String>,
}

// -------------- //
// Implémentation //
// -------------- //

impl Builder {
	/// Ajoute un filtre au système de log.
	pub fn filter<F>(mut self, predicate: F, dependency: impl ToString) -> Self
	where
		F: 'static,
		F: Send + Sync,
		F: Fn(&log::Metadata) -> bool,
	{
		self.filter.push_callback(predicate);
		self.filter.add_dependency(dependency);
		self
	}

	/// Autorise les logs à être colorés.
	pub fn with_color(mut self) -> Self {
		self.colorized = true;
		self
	}

	/// Autorise ou non les logs à être colorés.
	pub fn define_color(mut self, b: impl Into<bool>) -> Self {
		self.colorized = b.into();
		self
	}

	/// Le format du log.
	pub fn with_format(mut self, format: FormatFn) -> Self {
		self.format_fn.replace(format);
		self
	}

	/// Le niveau de log.
	pub fn with_level(mut self, level: impl Into<log::LevelFilter>) -> Self {
		self.level.replace(level.into());
		self
	}

	/// Autorise les logs à avoir un timestamp.
	pub fn with_timestamp(mut self) -> Self {
		self.timestamp = true;
		self
	}

	/// Autorise ou non les logs à avoir un timestamp.
	pub fn define_timestamp(mut self, b: impl Into<bool>) -> Self {
		self.timestamp = b.into();
		self
	}

	/// Construction du logger (normal)
	pub fn build_stdout(self) -> Result<(), log::SetLoggerError> {
		use crate::layout::{Alignment, Cell};
		use crate::stdout;

		stdout::Logger {
			colorized: self.colorized,
			timestamp: self.timestamp,
			format_fn: self.format_fn.unwrap_or(|echo, message, record| {
				let local_date_format = echo.time.map(|local_datetime| {
					local_datetime.format("%Y-%m-%d@%H:%M:%S")
				});

				if let Some(time) = local_date_format {
					echo.table.add_line([
						Cell::new(&echo.level).with_alignment(Alignment::Right),
						Cell::new(&echo.delimiter),
						Cell::new(time),
						Cell::new(&echo.delimiter),
						Cell::new(format!(
							"{} {} {}",
							style(record.target()).black().bright(),
							style("->").red(),
							message
						)),
					]);
				} else {
					echo.table.add_line([
						Cell::new(&echo.level).with_alignment(Alignment::Right),
						Cell::new(&echo.delimiter),
						Cell::new(style(record.target()).black().bright()),
						Cell::new(style("->").red()),
						Cell::new(message),
					]);
				}

				echo.table.render()
			}),
			level: self.level,
			filter: stdout::LoggerFilter {
				callbacks: self.filter.callbacks,
				dependencies: self.filter.dependencies,
			},
			cache: Default::default(),
		}
		.apply()
	}
}

impl BuilderFilter {
	fn push_callback<F>(&mut self, predicate: F)
	where
		F: 'static,
		F: Send + Sync,
		F: Fn(&log::Metadata) -> bool,
	{
		self.callbacks.push(Box::new(predicate));
	}

	fn add_dependency(&mut self, dependency: impl ToString) {
		self.dependencies.push(dependency.to_string());
	}
}
