use clap::Parser;


/// A build log analysis tool
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(short, long, value_enum)]
    pub action: ActionKind,

    #[clap(short, long)]
    pub set: Option<String>,

    #[clap(short, long)]
    pub find: bool,

    #[clap(short, long, default_value = "cargo")]
    pub parser: String,
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum ActionKind {
    Build,
    Run,
    Test,
    GoTo
}
