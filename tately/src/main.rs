/*
TATELY by Alexander Abraham a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

/// Declaring the "modules"
/// directory as a module itself.
mod modules;

/// Importing the CLI function
/// from "modules/cli.rs".
use modules::cli::cli;

/// Main point of entry
/// for the Rust compiler.
pub fn main() -> () {
    cli();
}