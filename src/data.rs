use anyhow::Result;
use std::{fs, path::PathBuf};

const EXCLUDED: &[&str] = &["src", "target", ".git"];

pub fn skills_dir() -> PathBuf {
    dirs::home_dir().unwrap_or_default().join("skills")
}

pub fn categories() -> Vec<String> {
    let mut cats = Vec::new();
    if let Ok(rd) = fs::read_dir(skills_dir()) {
        for entry in rd.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if entry.file_type().map(|t| t.is_dir()).unwrap_or(false)
                && !name.starts_with('.')
                && !EXCLUDED.contains(&name.as_str())
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
                && !name.starts_with('.')
            {
                entries.push(name);
            }
        }
    }
    entries.sort();
    entries
}

pub fn read_entry(category: &str, name: &str) -> Result<String> {
    Ok(fs::read_to_string(skills_dir().join(category).join(name))?)
}

pub fn save_entry(category: &str, name: &str, content: &str) -> Result<()> {
    let dir = skills_dir().join(category);
    fs::create_dir_all(&dir)?;
    fs::write(dir.join(name), content)?;
    Ok(())
}

pub fn delete_entry(category: &str, name: &str) -> Result<()> {
    fs::remove_file(skills_dir().join(category).join(name))?;
    Ok(())
}
