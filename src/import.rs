use std::fs;

use crate::extractor::{markdown::Markdown, Extractor};

pub fn import() -> Option<(String, String)> {
    if let Some(command) = get_command_to_import() {
        println!("Under with key do you want to save the command?");

        let mut line = String::new();
        std::io::stdin().read_line(&mut line).ok()?;
        let line = line.trim();

        return Some((line.to_string(), command));
    }

    None
}

fn get_command_to_import() -> Option<String> {
    if let Ok(content) = fs::read_to_string("README.md") {
        if let Some(commands) = Markdown::extract_commands(content) {
            println!("The following commands can be imported");
            commands.clone()
                .into_iter()
                .map(|c| trim_line(c, 80))
                .enumerate()
                .for_each(|(line, c)| {
                    println!("{}: {}", line, c);
                });
            println!("To import the commands type the number before the command.");

            let mut line = String::new();
            std::io::stdin().read_line(&mut line).ok()?;
            let line = line.trim();
            let index: usize = line.parse().ok()?;

            return commands.get(index).cloned();
        }
    }

    None
}

fn trim_line(line: String, with: usize) -> String {
    let mut line = line.replace('\n', " ").replace('\r', "");
    if line.len() >= with {
        line = (line[..with]).to_string();
        line.push_str("...")
    }
    line
}
