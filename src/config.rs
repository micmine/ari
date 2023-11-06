use clap::Parser;

/// A build log analysis tool
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Acttion to do
    #[clap(short, long, value_enum, default_value = "", required_unless_present_any = ["print_actions", "import", "find_actions"])]
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
    
    /// Import commands from README.md
    #[clap(short, long)]
    pub import: bool,

    /// Print commands from README.md
    #[clap(short, long)]
    pub find_actions: bool,
}
