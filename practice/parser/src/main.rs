mod utils;
mod core;
mod models;

use std::env;
use crate::core::cli::{execute, parse_options};

fn main() {
    let env:Vec<String> = env::args().collect();
    execute(parse_options(&env));
}
