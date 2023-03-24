use std::io::Cursor;

use clap::Parser;
use config::ActionKind;
use skim::{
    prelude::{SkimItemReader, SkimOptionsBuilder},
    Skim,
};
use storage::{Project, Storage};

use crate::config::Args;

mod config;
mod storage;

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let storage_location = &storage::location()
        .await
        .expect("Unable to get storage dir");
    let storage = storage::load_storage(&storage_location).await;

    if let Some(project) = get_project(&args, storage.clone()) {
        if let Some(storage) = set_value(&args, &project, storage) {
            storage::save_storage(storage, &storage_location)
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
            run_logana(&command, &logana_parser).await;
        } else {
            println!("{command}");
        }
    }
}

fn get_project<'a>(args: &Args, storage: Storage) -> Option<Project> {
    if args.find {
        return select(storage);
    }

    let current = std::env::current_dir().unwrap();
    let current: &str = current.to_str().unwrap();

    return storage
        .projects
        .into_iter()
        .filter(|p| p.location == current)
        .last();
}

fn set_value(args: &Args, project: &Project, storage: Storage) -> Option<Storage> {
    if let Some(set) = &args.set {
        let mut updated_storage = storage.clone();
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

fn select<'a>(storage: Storage) -> Option<Project> {
    let options = SkimOptionsBuilder::default().build().unwrap();

    let mut list = String::new();

    for p in &storage.projects {
        list.push_str(&p.location);
        list.push('\n')
    }

    let item_reader = SkimItemReader::default();
    let items = item_reader.of_bufread(Cursor::new(list));

    let selected_item = Skim::run_with(&options, Some(items))
        .map(|out| out.selected_items)
        .unwrap_or_default();

    if let Some(item) = selected_item.first() {
        let text = item.text();

        return storage
            .projects
            .into_iter()
            .filter(|p| p.location == text)
            .last();
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

async fn run_logana(command: &str, parser: &str) {
    let args = vec!["logana", "-p", parser, "-c", command];
    let args = logana::core::config::Args::parse_from(args);
    logana::run(args).await;
}
