use super::Extractor;
use tree_sitter::{Parser, Query, QueryCursor};
pub struct Markdown;

impl Extractor for Markdown {
    fn extract_commands(content: String) -> Option<Vec<String>> {
        let mut parser = Parser::new();
        let language = tree_sitter_md::language();
        parser
            .set_language(language)
            .expect("Error loading markdown grammar");
        let tree = parser.parse(&content, None).unwrap();
        let mut cursor = QueryCursor::new();

        let query = Query::new(language, "(code_fence_content) @capture").ok()?;
        let captures = cursor.captures(&query, tree.root_node(), content.as_bytes());
        let commands: Vec<String> = captures
            .into_iter()
            .flat_map(|(c, _)| c.captures)
            .map(|c| c.node.utf8_text(content.as_bytes()))
            .filter(|c| c.is_ok())
            .map(|c| c.unwrap().trim())
            .flat_map(|c| c.lines().collect::<Vec<_>>())
            .filter(|c| !c.starts_with('#')) // remove shell comments
            .map(|c| c.to_owned())
            .collect();

        Some(commands)
    }
}

#[cfg(test)]
mod tests {
    use super::Markdown;
    use crate::extractor::Extractor;

    #[test]
    fn should_find_commands() {
        let data = "# this a cool tool you can build it with the following command: 
``` command
cargo build
```
and also you can run the tests with
``` command
cargo test
```
"
        .to_string();
        let commands = Markdown::extract_commands(data);

        assert_eq!(
            commands,
            Some(vec!["cargo build".to_string(), "cargo test".to_string()])
        )
    }

    #[test]
    fn should_find_commands_in_one_block() {
        let data = "# this a cool tool you can build it with the following command: 
``` command
cargo build
# And build a release with
cargo build --release
```
"
        .to_string();
        let commands = Markdown::extract_commands(data);

        assert_eq!(
            commands,
            Some(vec!["cargo build".to_string(), "cargo build --release".to_string()])
        )
    }
}
