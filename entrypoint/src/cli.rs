use clap::Parser;

#[derive(Parser)]
#[command(version)]
pub struct Args {
    /// App name to run. Available app names: 'renderer', '...'
    #[arg(long, default_value = "renderer")]
    pub appname: String,
}

pub fn parse() -> Args {
    Args::parse()
}
