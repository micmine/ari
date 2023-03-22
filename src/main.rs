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
    let storage = storage::load_storage(storage_location).await;

    if let Some(project) = get_project(&args, storage) {
        handle_action(args.action, &project, &args.parser).await;
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

async fn handle_action(action: ActionKind, project: &Project, parser: &str) {
    std::env::set_current_dir(&project.location).unwrap();
    match action {
        ActionKind::Build => run(&project.build, parser).await,
        ActionKind::Run => run(&project.run, parser).await,
        ActionKind::Test => run(&project.test, parser).await,
        ActionKind::GoTo => (),
    }
}

async fn run(command: &Option<String>, parser: &str) {
    let Some(command) = command else {
        println!("There is no command defined for this action");
        return;
    };
    let args = vec!["logana", "-p", parser, "-c", &command];
    let args = logana::core::config::Args::parse_from(args);
    logana::run(args).await;
}
