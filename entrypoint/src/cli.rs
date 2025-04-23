use clap::Parser;

#[derive(Parser)]
#[command(version)]
pub struct Args {
    /// App name to run. Available app names: 'mesh_2d', 'renderer'
    #[arg(long, default_value = "renderer")]
    pub appname: String,
}

pub fn parse() -> Args {
    Args::parse()
}
