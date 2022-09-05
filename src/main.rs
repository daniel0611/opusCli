//! # opusCli
//! 
//! cli todo tool - add, edit, tag and delete tasks
//! 
//! ## Showcase
//! 
//! ## Installation
//! 
//! ## Syntax
//! 
//! ### Add a task
//! 
//! ```
//! opus add "update excelsheet #work @tomorrow ,,,"
//!           ^^^^^^^^^^^^^^^^^ ^^^^^ ^^^^^^^^^ ^^^
//!           |                 |     |         |
//!           |                 |     |         |_ priority
//!           |                 |     |
//!           |                 |     |_ due date
//!           |                 |
//!           |                 |_ tag
//!           |_ title
//! ```
//! 
//! -   tags are prefixed with a `#` and should contain `_` instead of spaces.
//! -   due date is prefixed with a `@` and can either be `@today`, `@tomorrow` or a date (`yyyy-mm-dd`)
//! -   priority is specified using `,` - highest priority is one `,`
//! 
//! ### List tasks
//! 
//! ```bash
//! opus list
//! opus list "#work"   # List all tasks with tag work (#work):
//! opus list ","       # List all tasks with priority !! (2):
//! opus list .2        # List task with id 2
//! ```
//! 
//! ## Contributing
use std::env;

use crate::cli::parse_cli;
use crate::types::Cli;

mod cli;
mod types;

fn main() {
    let args: Vec<String> = env::args().collect();
    let result: Cli = parse_cli(args);
    println!("{:?}", result);
}