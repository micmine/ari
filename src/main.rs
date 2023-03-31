use clap::Parser;
use config::ActionKind;
use serde::{Serialize, Deserialize};

#[cfg(not(target_os = "windows"))]
use find::select;

use crate::config::Args;

mod config;

#[cfg(not(target_os = "windows"))]
mod find;

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let storage_location = &quickcfg::get_location("ari")
        .await
        .expect("Unable to get storage dir");
    let storage: Storage = quickcfg::load(storage_location).await;

    if let Some(project) = get_project(&args, storage.clone()) {
        if let Some(storage) = set_value(&args, &project, storage) {
            quickcfg::save(storage, storage_location)
                .await
                .expect("Unable to save storage");
            return;
        }
        std::env::set_current_dir(&project.location).unwrap();
        let Some(command) = get_command_from_action(args.action, &project) else {
            println!("There is no command defined for this action");
            return;
        };

        if let Some(logana_parser) = &args.parser {
            run_logana(&command, logana_parser).await;
        } else {
            println!("{command}");
        }
    }
}

fn get_project(args: &Args, storage: Storage) -> Option<Project> {
    if cfg!(not(target_os = "windows")) && args.find {
        return select(storage);
    }

    let current = std::env::current_dir().unwrap();
    let current: &str = current.to_str().unwrap();

    storage
        .projects
        .into_iter()
        .filter(|p| p.location == current)
        .last()
}

fn set_value(args: &Args, project: &Project, storage: Storage) -> Option<Storage> {
    if let Some(set) = &args.set {
        let mut updated_storage = storage;
        for p in &mut updated_storage.projects {
            if p.location == project.location {
                match &args.action {
                    ActionKind::Build => p.build = Some(set.to_owned()),
                    ActionKind::Run => p.run = Some(set.to_owned()),
                    ActionKind::Test => p.test = Some(set.to_owned()),
                    ActionKind::GoTo => (),
                }
            }
        }

        return Some(updated_storage);
    }

    None
}

fn get_command_from_action(action: ActionKind, project: &Project) -> Option<String> {
    match action {
        ActionKind::Build => project.build.to_owned(),
        ActionKind::Run => project.run.to_owned(),
        ActionKind::Test => project.test.to_owned(),
        ActionKind::GoTo => Some("cd ".to_owned() + &project.location),
    }
}


#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Storage {
    pub projects: Vec<Project>,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Project {
    pub location: String,
    pub build: Option<String>,
    pub run: Option<String>,
    pub test: Option<String>,
}

async fn run_logana(command: &str, parser: &str) {
    let args = vec!["logana", "-p", parser, "-c", command];
    let args = logana::core::config::Args::parse_from(args);
    logana::run(args).await;
}
