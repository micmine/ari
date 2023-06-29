use std::collections::{BTreeMap, HashMap};

use clap::Parser;
use itertools::Itertools;
use serde::{Deserialize, Serialize};

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

    if let Some(storage) = set_value(&args, storage.clone()) {
        quickcfg::save(storage, storage_location)
            .await
            .expect("Unable to save storage");
        return;
    }
    if let (Some(project), location) = get_project(&args, storage) {
        std::env::set_current_dir(location).unwrap();
        let Some(command) = project.actions.get(&args.action) else {
            println!("This action is not supported here did you mean: {}", project.actions.keys()
                     .into_iter()
                     .map(|k| k.to_string())
                     .intersperse(", ".to_string())
                     .collect::<String>());
            return;
        };

        if let Some(logana_parser) = &args.parser {
            run_logana(&command, logana_parser).await;
        } else {
            println!("{command}");
        }
    }
}

fn get_project(args: &Args, storage: Storage) -> (Option<Project>, String) {
    if cfg!(not(target_os = "windows")) && args.find {
        return select(storage);
    }

    let Ok(current) = std::env::current_dir() else {
        println!("Unable to get current directory");
        return (None, String::new());
    };
    let Some(current) = current.to_str() else {
        return (None, String::new());
    };

    (storage.projects.get(current).cloned(), current.to_string())
}

fn set_value(args: &Args, storage: Storage) -> Option<Storage> {
    if let Some(set) = &args.set {
        dbg!(set);
        let mut updated_storage = storage;
        let location = std::env::current_dir().unwrap().display().to_string();

        if let Some(project) = updated_storage.projects.get_mut(&location) {
            project
                .actions
                .insert(args.action.to_string(), set.to_string());
        } else {
            let mut newproject = Project {
                actions: BTreeMap::new(),
            };
            newproject
                .actions
                .insert(args.action.to_string(), set.to_string());
            updated_storage.projects.insert(location, newproject);
        }

        return Some(updated_storage);
    }

    None
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Storage {
    pub projects: HashMap<String, Project>,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Project {
    #[serde(flatten)]
    pub actions: BTreeMap<String, String>,
}

async fn run_logana(command: &str, parser: &str) {
    let args = vec!["logana", "-p", parser, "-c", command];
    let args = logana::core::config::Args::parse_from(args);
    if let Ok(dir) = std::env::current_dir() {
        if let Some(dir) = dir.to_str() {
            logana::run(args, dir).await;
        }
    }
}
