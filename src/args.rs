use clap::Parser;

/// random-number-cli
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
pub struct ranArgs {
    #[clap(short = 'm', long, default_value_t = 1)]
    pub min: u32,

    #[clap(short = 'M', long, default_value_t = 100)]
    pub max: u32,

    // Number of generated numbers
    #[clap(short = 'c', long, default_value_t = 1)]
    pub count: u32,
}
