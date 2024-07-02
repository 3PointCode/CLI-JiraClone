mod models;

mod data_base;
use data_base::*;

mod user_interface;

mod io_utils;
use io_utils::*;

mod navigator;
use navigator::*;

use std::rc::Rc;

fn main() {
    println!("Welcome to my Jira clone!");
}
