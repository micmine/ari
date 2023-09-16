use std::io::Cursor;

use skim::{
    prelude::{SkimItemReader, SkimOptionsBuilder},
    Skim,
};

use crate::{Project, Storage};

pub fn select(storage: &Storage) -> (Option<Project>, String) {
    let options = SkimOptionsBuilder::default().build().unwrap();

    let mut list = String::new();

    for location in storage.projects.keys() {
        list.push_str(location);
        list.push('\n')
    }

    let item_reader = SkimItemReader::default();
    let items = item_reader.of_bufread(Cursor::new(list));

    let selected_item = Skim::run_with(&options, Some(items))
        .map(|out| out.selected_items)
        .unwrap_or_default();

    if let Some(item) = selected_item.first() {
        let text = item.text();
        let text = text.as_ref();

        return (storage.projects.get(text).cloned(), text.to_string());
    }

    (None, String::new())
}
