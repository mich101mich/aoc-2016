#![allow(unused_imports)]

#[macro_use]
extern crate scan_fmt;

#[macro_use]
mod utils;
mod days {
    pub mod day_01;
}
use days::day_01;

fn main() {
    day_01::run();
}
