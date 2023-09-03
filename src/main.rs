use std::{
    collections::{BTreeMap, HashMap},
    time::SystemTime,
};

use clap::Parser;
use itertools::Itertools;
use serde::{Deserialize, Serialize};

#[cfg(not(target_os = "windows"))]
use find::select;

use crate::config::Args;

mod config;
mod extractor;
mod import;

#[cfg(not(target_os = "windows"))]
mod find;

#[tokio::main]
async fn main() {
    let args = Args::parse();

    if args.import {
        if let Some((key, command)) = import::import() {}
        return;
    }

    let storage_location = &quickcfg::get_location("ari")
        .await
        .expect("Unable to get storage dir");
    let storage: Storage = quickcfg::load(storage_location).await;

    if let Some(new_storage) = set_value(&args, storage.clone()) {
        let backup_file_name = format!(
            "ari_{}.json",
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .expect("unable to get time")
                .as_secs()
        );
        let backup_location = storage_location.replace("ari.json", &backup_file_name);
        match quickcfg::save(storage, &backup_location).await {
            Ok(_) => {
                quickcfg::save(new_storage, storage_location)
                    .await
                    .expect("Unable to save storage");
            }
            Err(_) => todo!(),
        };
        return;
    }
    if let (Some(project), location) = get_project(&args, storage) {
        std::env::set_current_dir(location).unwrap();

        if args.print_actions {
            println!("{}", get_actions_string(&project, '\n'));
            return;
        }

        let Some(command) = project.actions.get(&args.action) else {
            println!(
                "This action is not supported here did you mean: {}",
                get_actions_string(&project, ',')
            );
            return;
        };

        println!("{command}");
    }
}

fn get_project(args: &Args, storage: Storage) -> (Option<Project>, String) {
    #[cfg(not(target_os = "windows"))]
    if args.find {
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

fn set_key_value(args: &Args, storage: Storage) {
    if let Some(new_storage) = set_value(&args, storage.clone()) {
        let backup_file_name = format!(
            "ari_{}.json",
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .expect("unable to get time")
                .as_secs()
        );
        let backup_location = storage_location.replace("ari.json", &backup_file_name);
        match quickcfg::save(storage, &backup_location).await {
            Ok(_) => {
                quickcfg::save(new_storage, storage_location)
                    .await
                    .expect("Unable to save storage");
            }
            Err(_) => todo!(),
        };
    }
}

fn set_value(args: &Args, storage: Storage) -> Option<Storage> {
    if let Some(set) = &args.set {
        println!("Setting {} action to \"{}\"", &args.action, &set);
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

fn get_actions_string(project: &Project, seperator: char) -> String {
    // Keeping if the description makes problems
    //project
    //.actions
    //.keys()
    //.into_iter()
    //.map(|k| k.to_string())
    //.intersperse(seperator.to_string())
    //.collect::<String>()

    project
        .actions
        .clone()
        .into_iter()
        .map(|(a, b)| format!("{a}\t{b}"))
        .intersperse(seperator.to_string())
        .collect::<String>()
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
