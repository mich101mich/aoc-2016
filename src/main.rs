#![allow(unused_imports)]

#[macro_use]
extern crate scan_fmt;

#[macro_use]
mod utils;
mod days {
    pub mod day_05;
}
use days::day_05;

fn main() {
    day_05::run();
}
