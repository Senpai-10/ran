mod args;

use args::RanArgs;
use clap::Parser;
use rand::Rng;

fn main() {
    let cli = RanArgs::parse();

    if !cli.inline {
        for _ in 0..cli.count {
            let random_number: i32 = rand::thread_rng().gen_range(cli.min..cli.max);
            println!("{}", random_number);
        }
    }

    if cli.inline && cli.count > 1 {
        for c in 1..cli.count + 1 {
            let random_number: i32 = rand::thread_rng().gen_range(cli.min..cli.max);
            if cli.delimiter_before {
                print!("{}", cli.delimiter);
            }

            print!("{}", random_number);

            if cli.delimiter_after || !cli.delimiter_after && !cli.delimiter_before {
                if c != cli.count {
                    print!("{}", cli.delimiter);
                }
            }
        }
    }
}
