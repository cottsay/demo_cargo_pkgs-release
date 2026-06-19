use colored::Colorize;
use demo_cargo_pkg_lib::to_snake;

fn main() {
    println!("{}", to_snake("Demo Cargo Pkg").green());
}
