use std::fs;
use std::time::SystemTime;
use zed_extension_api::{self as zed, Result};

struct SpyglassmcExtension;

impl SpyglassmcExtension {
    fn format_windows_path(path: String) -> String {
        let mut path = path;
        if path.starts_with('/') && path.chars().nth(2) == Some(':') {
            path.remove(0);
        }
        path.replace('/', "\\")
    }

    fn should_update(path: &str, interval_seconds: u64) -> bool {
        if let Ok(metadata) = fs::metadata(path) {
            if let Ok(modified) = metadata.modified() {
                if let Ok(duration) = SystemTime::now().duration_since(modified) {
                    return duration.as_secs() > interval_seconds;
                }
            }
            false
        } else {
            true
        }
    }
}

impl zed::Extension for SpyglassmcExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let root = _worktree.root_path();
        eprintln!("Worktree root: {}", root);
        let server_filename = "zed_spyglass_server.js";
        let download_url =
            "https://github.com/VoidDonpig/Spyglass-LSP-Index/releases/download/latest/index.js";

        let needs_update = Self::should_update(server_filename, 86400);
        let exists = fs::metadata(server_filename).is_ok();

        if needs_update || !exists {
            match zed::download_file(
                download_url,
                server_filename,
                zed::DownloadedFileType::Uncompressed,
            ) {
                Ok(_) => {}
                Err(e) => {
                    if !exists {
                        return Err(format!("Failed to download LSP: {e}"));
                    }
                }
            }
        }

        let work_dir = std::env::current_dir().map_err(|e| e.to_string())?;
        let full_path =
            Self::format_windows_path(work_dir.join(server_filename).to_string_lossy().to_string());

        Ok(zed::Command {
            command: zed::node_binary_path()?,
            args: vec![full_path, "--stdio".to_string()],
            env: Default::default(),
        })
    }
}

zed::register_extension!(SpyglassmcExtension);
