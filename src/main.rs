#![allow(unused_imports)]

#[macro_use]
extern crate scan_fmt;

#[macro_use]
mod utils;
mod days {
    pub mod day_03;
}
use days::day_03;

fn main() {
    day_03::run();
}
