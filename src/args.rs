use clap::Parser;

/// random-number-cli
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
pub struct RanArgs {
    #[clap(short = 'm', long, default_value_t = 1)]
    pub min: i32,

    #[clap(short = 'M', long, default_value_t = 100)]
    pub max: i32,

    /// Number of generated numbers
    #[clap(short = 'c', long, default_value_t = 1)]
    pub count: u32,

    /// Inline print all generated numbers
    #[clap(short = 'i', long)]
    pub inline: bool,

    /// Change inline print delimiter
    #[clap(short = 'd', long, default_value = " ")]
    pub delimiter: String,

    /// Print delimiter after number
    #[clap(short = 'a', long = "delimiter-after")]
    pub delimiter_after: bool,

    /// Print delimiter before number
    #[clap(short = 'b', long = "delimiter-before")]
    pub delimiter_before: bool,
}
