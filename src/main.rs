#![allow(unused_imports)]

#[macro_use]
extern crate scan_fmt;

#[macro_use]
mod utils;
mod days {
    pub mod day_02;
}
use days::day_02;

fn main() {
    day_02::run();
}
