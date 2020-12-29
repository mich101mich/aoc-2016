#![allow(unused_imports)]

#[macro_use]
extern crate scan_fmt;

#[macro_use]
mod utils;
mod days {
    pub mod day_04;
}
use days::day_04;

fn main() {
    day_04::run();
}
