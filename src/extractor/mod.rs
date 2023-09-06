
pub mod markdown;

pub trait Extractor {
    fn extract_commands(content: String) -> Option<Vec<String>>;
}
