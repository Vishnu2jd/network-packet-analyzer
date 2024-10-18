use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Network interface to capture packets from
    #[arg(short, long)]
    pub interface: Option<String>,
}

impl Cli {
    pub fn parse() -> Self {
        <Self as Parser>::parse()
    }
}
