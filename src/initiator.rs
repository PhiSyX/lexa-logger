// ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
// ┃ Copyright: (c) 2023, Mike 'PhiSyX' S. (https://github.com/PhiSyX)         ┃
// ┃ SPDX-License-Identifier: MPL-2.0                                          ┃
// ┃ ╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌ ┃
// ┃                                                                           ┃
// ┃  This Source Code Form is subject to the terms of the Mozilla Public      ┃
// ┃  License, v. 2.0. If a copy of the MPL was not distributed with this      ┃
// ┃  file, You can obtain one at https://mozilla.org/MPL/2.0/.                ┃
// ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛

use crate::LoggerBuilder;

// --------- //
// Structure //
// --------- //

pub struct LoggerInitiator;

// -------------- //
// Implémentation //
// -------------- //

impl LoggerInitiator
{
	/// Initialise le logger STDOUT à partir du builder.
	#[cfg(not(feature = "tracing"))]
	pub fn stdout(builder: impl LoggerBuilder<crate::LoggerStdout>) -> Result<(), log::SetLoggerError>
	{
		let stdout = builder.build();
		let level = stdout.level();

		log::set_max_level(level);

		if log::LevelFilter::Off == log::max_level() {
			const NO: crate::noop::NopeLogger = crate::noop::NopeLogger;
			log::set_logger(&NO)
		} else {
			log::set_boxed_logger(Box::new(stdout))
		}
	}

	#[cfg(feature = "tracing")]
	pub fn stdout(
		builder: impl LoggerBuilder<crate::LoggerStdout>,
	) -> Result<(), tracing_subscriber::fmt::SubscriberBuilder>
	{
		let stdout = builder.build();
		let level = stdout.level();

		let trsb = tracing_subscriber::fmt()
			.with_max_level(level)
			.with_ansi(stdout.colorized)
			.with_line_number(true);

		if stdout.timestamp {
			trsb.init();
		} else {
			trsb.without_time().init();
		}

		Ok(())
	}

	// NOTE: Initialiser d'autres types de logger ici...
}
