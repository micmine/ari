use std::io::Cursor;

use skim::{
    prelude::{SkimItemReader, SkimOptionsBuilder},
    Skim,
};

use crate::{Storage, Project};


pub fn select(storage: Storage) -> Option<Project> {
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
