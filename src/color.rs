// Copyright 2014 The Peevee Developers
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// static COLOR_BLACK: &'static str   = "\x1b[30m";
static COLOR_RED: &'static str     = "\x1b[31m";
static COLOR_GREEN: &'static str   = "\x1b[32m";
// static COLOR_YELLOW: &'static str  = "\x1b[33m";
static COLOR_BLUE: &'static str    = "\x1b[34m";
// static COLOR_MAGENTA: &'static str = "\x1b[35m";
// static COLOR_CYAN: &'static str    = "\x1b[36m";
// static COLOR_WHITE: &'static str   = "\x1b[37m";

pub enum Color {
    // Black,
    Red,
    Green,
    // Yellow,
    Blue,
    // Magenta,
    // Cyan,
    // White,
}

impl Color {
    pub fn to_color_code(&self) -> &'static str {
        match *self {
            // Color::Black   => COLOR_BLACK,
            Color::Red     => COLOR_RED,
            Color::Green   => COLOR_GREEN,
            // Color::Yellow  => COLOR_YELLOW,
            Color::Blue    => COLOR_BLUE,
            // Color::Magenta => COLOR_MAGENTA,
            // Color::Cyan    => COLOR_CYAN,
            // Color::White   => COLOR_WHITE,
        }
    }
}
