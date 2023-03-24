use clap::Parser;


/// A build log analysis tool
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Acttion to do
    #[clap(short, long, value_enum)]
    pub action: ActionKind,

    /// Overwrite option
    #[clap(short, long)]
    pub set: Option<String>,

    /// Search for directory
    #[clap(short, long)]
    pub find: bool,

    /// When provided the output will be analysed by logana
    #[clap(short, long)]
    pub parser: Option<String>,
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum ActionKind {
    Build,
    Run,
    Test,
    GoTo
}
