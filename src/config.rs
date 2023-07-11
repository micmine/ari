use clap::Parser;

/// A build log analysis tool
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Acttion to do
    #[clap(short, long, value_enum, default_value = "", required_unless_present = "print_actions")]
    pub action: String,

    /// Overwrite option
    #[clap(short, long)]
    pub set: Option<String>,

    /// Prints all available actions for the current project
    #[clap(long)]
    pub print_actions: bool,

    /// Search for directory
    #[cfg(not(target_os = "windows"))]
    #[clap(short, long)]
    pub find: bool,

    /// When provided the output will be analysed by logana
    #[clap(short, long)]
    pub parser: Option<String>,
}
