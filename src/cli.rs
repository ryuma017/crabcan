use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
pub struct Args {
    /// Activate debug mode
    #[clap(short, long)]
    debug: bool,

    /// Command to execute inside the container
    #[clap(short, long)]
    pub command: String,

    /// User ID to create inside the container
    #[clap(short, long)]
    pub uid: u32,

    /// Directory to mount as root of the container
    #[clap(parse(from_os_str), short = 'm', long = "mount")]
    pub mount_dir: PathBuf,
}

#[allow(clippy::let_and_return)]
pub fn parse_args() -> Args {
    let args = Args::parse();

    if args.debug {
        setup_log(log::LevelFilter::Debug);
    } else {
        setup_log(log::LevelFilter::Info);
    }

    // TODO
    // Validate arguments

    args
}

pub fn setup_log(level: log::LevelFilter) {
    env_logger::Builder::from_default_env()
        .format_timestamp_secs()
        .filter(None, level)
        .init();
}
