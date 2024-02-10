// ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
// ┃ Copyright: (c) 2023, Mike 'PhiSyX' S. (https://github.com/PhiSyX)         ┃
// ┃ SPDX-License-Identifier: MPL-2.0                                          ┃
// ┃ ╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌ ┃
// ┃                                                                           ┃
// ┃  This Source Code Form is subject to the terms of the Mozilla Public      ┃
// ┃  License, v. 2.0. If a copy of the MPL was not distributed with this      ┃
// ┃  file, You can obtain one at https://mozilla.org/MPL/2.0/.                ┃
// ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛

use super::extension::LoggerStdoutBuilderExtension;
use super::LoggerFilter;
use crate::builder::LoggerFormatFn;
use crate::{LoggerBuilder, LoggerStdout};

// --------- //
// Structure //
// --------- //

#[derive(Default)]
pub struct LoggerStdoutBuilder
{
	colorized: bool,
	timestamp: bool,
	#[cfg(not(feature = "tracing"))]
	level: Option<log::LevelFilter>,
	#[cfg(feature = "tracing")]
	level: Option<tracing::level_filters::LevelFilter>,
	format_fn: Option<LoggerFormatFn>,
	filter: LoggerFilter,
}

// -------------- //
// Implémentation // -> Interface
// -------------- //

impl LoggerBuilder<LoggerStdout> for LoggerStdoutBuilder
{
	fn filter<F>(mut self, predicate: F, dependency: impl ToString) -> Self
	where
		F: 'static,
		F: Send + Sync,
		F: Fn(&log::Metadata) -> bool,
	{
		self.filter.push_callback(predicate);
		self.filter.add_dependency(dependency);
		self
	}

	fn with_color(mut self, colorized: impl Into<bool>) -> Self
	{
		self.colorized = colorized.into();
		self
	}

	fn with_format(mut self, format: LoggerFormatFn) -> Self
	{
		self.format_fn.replace(format);
		self
	}

	#[cfg(not(feature = "tracing"))]
	fn with_level(mut self, level: impl Into<log::LevelFilter>) -> Self
	{
		self.level.replace(level.into());
		self
	}

	#[cfg(feature = "tracing")]
	fn with_level(mut self, level: impl Into<tracing::level_filters::LevelFilter>) -> Self
	{
		self.level.replace(level.into());
		self
	}

	fn with_timestamp(mut self, b: impl Into<bool>) -> Self
	{
		self.timestamp = b.into();
		self
	}

	fn build(self) -> LoggerStdout
	{
		LoggerStdout {
			cache: Default::default(),
			colorized: self.colorized,
			filter: self.filter,
			format_fn: self.format_fn.unwrap_or(LoggerStdout::default_format),
			#[cfg(not(feature = "tracing"))]
			level: self.level.unwrap_or(log::LevelFilter::Off),
			#[cfg(feature = "tracing")]
			level: self.level.unwrap_or(tracing_subscriber::filter::LevelFilter::OFF),
			timestamp: self.timestamp,
		}
	}
}

impl LoggerStdoutBuilderExtension for LoggerStdoutBuilder {}
