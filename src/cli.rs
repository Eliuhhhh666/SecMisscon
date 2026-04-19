use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Target domain to scan 
    #[arg(short, long)]
    pub target: String,

    /// Number of concurrent threads for scanning (default:10)
    #[arg(short, long, default_value_t = 10)]
    pub threads: usize,

    ///Timeout for HTTP requests in seconds (default:5)
    #[arg(short, long, default_value_t = 5)]
    pub timeout: u64,

    ///Optional: Custom User-Agent string to use for requests
    #[arg(short, long)]
    pub user_agent: Option<String>,

    /// Optional: Output findings to a JSON file 
}

impl Cli {
    pub fn parse_args() -> Self {
        Self::parse()
    }
}