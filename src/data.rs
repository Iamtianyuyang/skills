use anyhow::Result;
use std::{fs, path::PathBuf};

pub fn skills_dir() -> PathBuf {
    dirs::home_dir().unwrap_or_default().join("skills").join("data")
}

pub fn categories() -> Vec<String> {
    let mut cats = Vec::new();
    if let Ok(rd) = fs::read_dir(skills_dir()) {
        for entry in rd.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if entry.file_type().map(|t| t.is_dir()).unwrap_or(false)
                && !name.starts_with('.')
            {
                cats.push(name);
            }
        }
    }
    cats.sort();
    cats
}

pub fn entries(category: &str) -> Vec<String> {
    let mut entries = Vec::new();
    if let Ok(rd) = fs::read_dir(skills_dir().join(category)) {
        for entry in rd.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if entry.file_type().map(|t| t.is_file()).unwrap_or(false)
                && name.ends_with(".md")
            {
                entries.push(name.trim_end_matches(".md").to_string());
            }
        }
    }
    entries.sort();
    entries
}

pub fn read_entry(category: &str, name: &str) -> Result<String> {
    Ok(fs::read_to_string(skills_dir().join(category).join(format!("{name}.md")))?)
}

pub fn save_entry(category: &str, name: &str, content: &str) -> Result<()> {
    let dir = skills_dir().join(category);
    fs::create_dir_all(&dir)?;
    fs::write(dir.join(format!("{name}.md")), content)?;
    Ok(())
}

pub fn delete_entry(category: &str, name: &str) -> Result<()> {
    fs::remove_file(skills_dir().join(category).join(format!("{name}.md")))?;
    Ok(())
}
