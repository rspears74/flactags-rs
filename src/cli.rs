use clap::Parser;

#[derive(Parser, Debug)]
pub struct Cli {
    #[arg(action, long)]
    pub print: bool,
    #[arg(long, short)]
    pub dir: Option<std::path::PathBuf>,
    #[arg(long)]
    pub album: Option<String>,
    #[arg(long)]
    pub artist: Option<String>,
    #[arg(long)]
    pub album_artist: Option<String>,
    #[arg(long)]
    pub genre: Option<String>,
    #[arg(long)]
    pub year: Option<String>,
    #[arg(action, long)]
    pub titles: bool,
    #[arg(action, long)]
    pub track_numbers: bool,
    #[arg(action, long)]
    pub disc_numbers: bool,
}
