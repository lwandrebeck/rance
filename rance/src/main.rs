//
// rance.rs
//
// Copyright 2020 Laurent Wandrebeck <l.wandrebeck@quelquesmots.fr>
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program; if not, write to the Free Software
// Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston,
// MA 02110-1301, USA.
//

//! # rance
//!
//! `rance` is ansible written in Rust
//! rance aims to be (maybe one day) a (faster) clone of ansible
//! Source code is GPL3. Please note that this is a personal project (read not funded), in order to learn Rust language.
//! That does not mean feedback or patches are not welcome.
//! Right now, rance is not useable for anything.
use clap_v3::{App, load_yaml};

use config::*;

use pyo3::prelude::*;
use pyo3::types::IntoPyDict;

use serde;
#[macro_use]
extern crate serde_derive;

use std::env;
use std::path::Path;

mod settings;
use settings::Settings;

fn main() {
	let settings = Settings::new();
    let yaml = load_yaml!("rance.yml");
    let matches = App::from(yaml).get_matches();
    if let Some(mode) = matches.value_of("pattern") {
        match mode {
            "vi" => println!("You are using vi"),
            "emacs" => println!("You are using emacs..."),
            _ => unreachable!(),
        }
	} else {
		println!("--mode <MODE> wasn't used...");
	}
}
