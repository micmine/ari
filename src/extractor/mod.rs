use std::collections::BTreeMap;

pub mod readme;

pub trait Extractor {
    fn extract_commands(content: String) -> Option<Vec<String>>;
}
