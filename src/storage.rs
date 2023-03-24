use std::io::ErrorKind;

use serde::{Deserialize, Serialize};
use tokio::{
    fs::{self, File},
    io::AsyncWriteExt,
};

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Storage {
    pub projects: Vec<Project>,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Project {
    pub location: String,
    pub build: Option<String>,
    pub run: Option<String>,
    pub test: Option<String>,
}

pub async fn location() -> Option<String> {
    if let Some(mut dir) = dirs::config_dir() {
        dir.push("ari");
        if let Some(dir) = dir.to_str() {
            if fs::create_dir_all(dir).await.is_err() {
                return None;
            }
        }
        dir.push("ari");
        dir.set_extension("json");

        if let Some(file) = dir.to_str() {
            return Some(file.to_string());
        }
    }

    None
}

pub async fn load_storage(location: &str) -> Storage {
    match fs::read_to_string(&location).await {
        Ok(data) => serde_json::from_str(&data).unwrap_or_else(|error| {
            println!("Could not load storage file: {error:?}");
            Storage::default()
        }),
        Err(e) => {
            if e.kind() == ErrorKind::NotFound {
                save_storage(Storage::default(), location)
                    .await
                    .expect("Unable to write storage");
            }

            Storage::default()
        }
    }
}

/// Save storage to filesystem
pub async fn save_storage(storage: Storage, location: &str) -> Result<(), std::io::Error> {
    let config: String = serde_json::to_string_pretty(&storage)?;
    let mut file = File::create(location).await?;

    file.write_all(config.as_bytes()).await?;

    file.flush().await?;

    Ok(())
}
