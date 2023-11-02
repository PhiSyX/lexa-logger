// ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
// ┃ Copyright: (c) 2023, Mike 'PhiSyX' S. (https://github.com/PhiSyX)         ┃
// ┃ SPDX-License-Identifier: MPL-2.0                                          ┃
// ┃ ╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌ ┃
// ┃                                                                           ┃
// ┃  This Source Code Form is subject to the terms of the Mozilla Public      ┃
// ┃  License, v. 2.0. If a copy of the MPL was not distributed with this      ┃
// ┃  file, You can obtain one at https://mozilla.org/MPL/2.0/.                ┃
// ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛

// --------- //
// Structure //
// --------- //

#[derive(Debug)]
#[derive(Clone)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Settings
{
	pub colorized: bool,
	pub max_level: SettingsLevel,
	pub target_filters: Vec<String>,
	pub timestamp: bool,
}

// ----------- //
// Énumération //
// ----------- //

#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(serde::Deserialize, serde::Serialize)]
pub enum SettingsLevel
{
	DEBUG,
	ERROR,
	INFO,
	TRACE,
	WARNING,
}

// -------------- //
// Implémentation // -> Interface
// -------------- //

impl Default for Settings
{
	fn default() -> Self
	{
		Self {
			colorized: true,
			timestamp: true,
			max_level: SettingsLevel::TRACE,
			target_filters: Default::default(),
		}
	}
}

impl From<SettingsLevel> for log::LevelFilter
{
	fn from(level: SettingsLevel) -> Self
	{
		match level {
			| SettingsLevel::DEBUG => Self::Debug,
			| SettingsLevel::ERROR => Self::Error,
			| SettingsLevel::INFO => Self::Info,
			| SettingsLevel::TRACE => Self::Trace,
			| SettingsLevel::WARNING => Self::Warn,
		}
	}
}
