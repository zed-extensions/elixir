use std::fs;

use zed_extension_api::Result;

pub(super) fn find_existing_binary(binary_name: &str) -> Option<String> {
    fs::read_dir(".")
        .ok()?
        .flatten()
        .filter_map(|entry| {
            let path = entry.path();
            if path.is_dir() {
                let binary_path = path.join(binary_name);
                if binary_path.is_file() {
                    return Some(binary_path.to_string_lossy().to_string());
                }
            }
            None
        })
        .next()
}

pub(super) fn remove_outdated_versions(
    language_server_id: &'static str,
    version_dir: &str,
) -> Result<()> {
    let entries = fs::read_dir(".").map_err(|e| format!("failed to list working directory {e}"))?;
    for entry in entries {
        let entry = entry.map_err(|e| format!("failed to load directory entry {e}"))?;
        if entry.file_name().to_str().is_none_or(|file_name| {
            file_name.starts_with(language_server_id) && file_name != version_dir
        }) {
            fs::remove_dir_all(entry.path()).ok();
        }
    }
    Ok(())
}
