use clap::Parser;

#[derive(Parser)]
#[command(version)]
pub struct Args {
    /// App name to run. Available app names: 'bloom_2d', 'mesh_2d'
    #[arg(long, default_value = "mesh_2d")]
    pub appname: String,
}

pub fn parse() -> Args {
    Args::parse()
}
