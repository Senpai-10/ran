mod args;

use args::ranArgs;
use clap::Parser;

fn main() {
    let cli = ranArgs::parse();

    println!("min: {}\nmax: {}", cli.min, cli.max);
}
