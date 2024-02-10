// ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
// ┃ Copyright: (c) 2023, Mike 'PhiSyX' S. (https://github.com/PhiSyX)         ┃
// ┃ SPDX-License-Identifier: MPL-2.0                                          ┃
// ┃ ╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌ ┃
// ┃                                                                           ┃
// ┃  This Source Code Form is subject to the terms of the Mozilla Public      ┃
// ┃  License, v. 2.0. If a copy of the MPL was not distributed with this      ┃
// ┃  file, You can obtain one at https://mozilla.org/MPL/2.0/.                ┃
// ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛

mod builder;
mod extension;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use console::style;

pub use self::builder::LoggerStdoutBuilder;
pub use self::extension::LoggerStdoutBuilderExtension;
use crate::builder::{LoggerFilterCallback, LoggerFormatFn};
use crate::echo::Echo;
use crate::layout;

// --------- //
// Structure //
// --------- //

// Contrat
pub struct LoggerStdout
{
	pub(crate) colorized: bool,
	pub(crate) timestamp: bool,
	#[cfg(not(feature = "tracing"))]
	pub(crate) level: log::LevelFilter,
	#[cfg(feature = "tracing")]
	pub(crate) level: tracing::level_filters::LevelFilter,
	pub(crate) format_fn: LoggerFormatFn,
	pub(crate) filter: LoggerFilter,
	pub(crate) cache: Arc<Mutex<HashMap<String, bool>>>,
}

#[derive(Default)]
pub struct LoggerFilter
{
	callbacks: Vec<Box<LoggerFilterCallback>>,
	dependencies: Vec<String>,
}

// -------------- //
// Implémentation //
// -------------- //

impl LoggerStdout
{
	pub fn builder() -> builder::LoggerStdoutBuilder
	{
		builder::LoggerStdoutBuilder::default()
	}
}

impl LoggerStdout
{
	#[cfg(not(feature = "tracing"))]
	pub fn level(&self) -> log::LevelFilter
	{
		self.level
	}

	#[cfg(feature = "tracing")]
	pub fn level(&self) -> tracing::level_filters::LevelFilter
	{
		self.level
	}

	fn default_format(message: &std::fmt::Arguments, record: &log::Record, echo: &mut Echo) -> String
	{
		let local_date_format = echo
			.time
			.map(|local_datetime| local_datetime.format("%Y-%m-%d@%H:%M:%S"));

		if let Some(time) = local_date_format {
			echo.table.add_line([
				layout::Cell::new(&echo.level).with_alignment(layout::Alignment::Right),
				layout::Cell::new(&echo.delimiter),
				layout::Cell::new(time),
				layout::Cell::new(&echo.delimiter),
				layout::Cell::new(format!(
					"{} {} {}",
					if echo.colorized {
						style(record.target()).black().bright()
					} else {
						style(record.target())
					},
					if echo.colorized { style("->").red() } else { style("->") },
					message
				)),
			]);
		} else {
			echo.table.add_line([
				layout::Cell::new(&echo.level).with_alignment(layout::Alignment::Right),
				layout::Cell::new(&echo.delimiter),
				layout::Cell::new(style(record.target()).black().bright()),
				layout::Cell::new(style("->").red()),
				layout::Cell::new(message),
			]);
		}

		echo.table.render()
	}
}

impl LoggerFilter
{
	/// Pousse une callback dans le tableau des callbacks.
	pub(crate) fn push_callback<F>(&mut self, predicate: F)
	where
		F: 'static,
		F: Send + Sync,
		F: Fn(&log::Metadata) -> bool,
	{
		self.callbacks.push(Box::new(predicate));
	}

	/// Ajoute une dépendance dans le tableau des dépendances.
	pub(crate) fn add_dependency(&mut self, dependency: impl ToString)
	{
		self.dependencies.push(dependency.to_string());
	}
}

// -------------- //
// Implémentation // -> Interface
// -------------- //

impl log::Log for LoggerStdout
{
	/// On ne veut pas afficher les logs si le niveau est à
	/// [log::LevelFilter::Off].
	///
	/// Des conditions utilisateurs peuvent être utilisées pour filtrer les
	/// logs.
	fn enabled(&self, metadata: &log::Metadata) -> bool
	{
		let mut guard = self.cache.lock().expect("cache guard");

		metadata.level() != log::LevelFilter::Off
			&& (self.filter.callbacks.is_empty()
				|| self.filter.callbacks.iter().enumerate().any(|(idx, once_fn)| {
					let cache_key = format!("{}_{}", self.filter.dependencies[idx], metadata.target());

					if let Some(has) = guard.get(&cache_key) {
						return *has;
					}

					let is_ok = once_fn(metadata);
					guard.insert(cache_key, is_ok);
					is_ok
				}))
	}

	/// Affiche le log.
	//
	// FIXME(phisyx): améliorer les performances du logger. Le simple fait de
	// les afficher nous fait perdre un temps considérable.
	fn log(&self, record: &log::Record)
	{
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

		let mut table = layout::GridLayout::default().define_max_width(120).without_boarder();

		let mut echo = Echo {
			colorized: self.colorized,
			delimiter: if self.colorized { style("|").red() } else { style("|") }.to_string(),
			level,
			record_level: record.level(),
			table: &mut table,
			time: if self.timestamp {
				Some(chrono::Local::now())
			} else {
				None
			},
		};

		let text = (self.format_fn)(message, record, &mut echo);

		echo.log(text);
	}

	fn flush(&self) {}
}
