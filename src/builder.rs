// ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
// ┃ Copyright: (c) 2023, Mike 'PhiSyX' S. (https://github.com/PhiSyX)         ┃
// ┃ SPDX-License-Identifier: MPL-2.0                                          ┃
// ┃ ╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌ ┃
// ┃                                                                           ┃
// ┃  This Source Code Form is subject to the terms of the Mozilla Public      ┃
// ┃  License, v. 2.0. If a copy of the MPL was not distributed with this      ┃
// ┃  file, You can obtain one at https://mozilla.org/MPL/2.0/.                ┃
// ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛

use crate::echo::Echo;

// ---- //
// Type //
// ---- //

pub(crate) type LoggerFormatFn = fn(&std::fmt::Arguments, &log::Record, &mut Echo) -> String;
pub(crate) type LoggerFilterCallback = dyn Fn(&log::Metadata) -> bool + Send + Sync;

// --------- //
// Interface //
// --------- //

pub trait LoggerBuilder<T>
{
	/// Ajoute un filtre au système de log.
	fn filter<F>(self, predicate: F, dependency: impl ToString) -> Self
	where
		F: 'static,
		F: Send + Sync,
		F: Fn(&log::Metadata) -> bool;

	/// Autorise ou non les logs à avoir les couleurs sur les informations
	/// contrôlées par notre système.
	fn with_color(self, colorized: impl Into<bool>) -> Self;

	/// Le format du log.
	fn with_format(self, format: LoggerFormatFn) -> Self;

	/// Le niveau de log.
	fn with_level(self, level: impl Into<log::LevelFilter>) -> Self;

	/// Autorise ou non les logs à avoir un timestamp.
	fn with_timestamp(self, b: impl Into<bool>) -> Self;

	/// Construction du logger.
	fn build(self) -> T;
}

// --------- //
// Structure //
// --------- //

pub struct Logger;

// -------------- //
// Implémentation //
// -------------- //

impl Logger
{
	/// Monteur de structure d'un logger de type stdout.
	///
	/// Paramètres activés:
	///    1. [LoggerBuilder::with_color()]
	///    2. [LoggerBuilder::with_timestamp()]
	pub fn stdout() -> crate::stdout::LoggerStdoutBuilder
	{
		crate::stdout::LoggerStdout::builder()
			.with_color(true)
			.with_timestamp(true)
	}

	// NOTE: Ajouter d'autres types de builder avec des paramètres par défaut
	// ici...
}
