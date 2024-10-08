// Furtherance - Track your time without being tracked
// Copyright (C) 2024  Ricky Kresslein <rk@unobserved.io>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct FurShortcut {
    pub id: u32,
    pub name: String,
    pub tags: String,
    pub project: String,
    pub rate: f32,
    pub currency: String,
    pub color_hex: String,
}

impl fmt::Display for FurShortcut {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)?;

        if !self.project.is_empty() {
            write!(f, " @{}", self.project)?;
        }

        if !self.tags.is_empty() {
            write!(f, " {}", self.tags)?;
        }

        if self.rate != 0.0 {
            write!(f, " ${:.2}", self.rate)?;
        }

        Ok(())
    }
}
