use std::fs;

use zed::LanguageServerId;
use zed_extension_api::lsp::{Completion, CompletionKind, Symbol, SymbolKind};
use zed_extension_api::settings::LspSettings;
use zed_extension_api::{self as zed, CodeLabel, CodeLabelSpan, Result};

use sha2::{Digest, Sha256};

use crate::language_servers::util;

pub struct ExpertBinary {
    pub path: String,
    pub args: Option<Vec<String>>,
}

pub struct Expert {
    cached_binary_path: Option<String>,
}

impl Expert {
    pub const LANGUAGE_SERVER_ID: &'static str = "expert";

    pub fn new() -> Self {
        Self {
            cached_binary_path: None,
        }
    }

    pub fn language_server_binary(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<ExpertBinary> {
        let binary_settings = LspSettings::for_worktree("expert", worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.binary);
        let binary_args = binary_settings
            .as_ref()
            .and_then(|binary_settings| binary_settings.arguments.clone());

        if let Some(path) = binary_settings.and_then(|binary_settings| binary_settings.path) {
            return Ok(ExpertBinary {
                path,
                args: binary_args,
            });
        }

        if let Some(path) = worktree.which("expert") {
            return Ok(ExpertBinary {
                path,
                args: binary_args,
            });
        }

        if let Some(path) = &self.cached_binary_path {
            if fs::metadata(path).map_or(false, |stat| stat.is_file()) {
                return Ok(ExpertBinary {
                    path: path.clone(),
                    args: binary_args,
                });
            }
        }

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );
        let release = zed::latest_github_release(
            "elixir-lang/expert",
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: true,
            },
        )?;

        let (platform, arch) = zed::current_platform();
        let asset_name = format!(
            "expert_{os}_{arch}{extension}",
            os = match platform {
                zed::Os::Mac => "darwin",
                zed::Os::Linux => "linux",
                zed::Os::Windows => "windows",
            },
            arch = match arch {
                zed::Architecture::Aarch64 => "arm64",
                zed::Architecture::X8664 => "amd64",
                zed::Architecture::X86 =>
                    return Err(format!("unsupported architecture: {arch:?}")),
            },
            extension = match platform {
                zed::Os::Mac | zed::Os::Linux => "",
                zed::Os::Windows => ".exe",
            }
        );

        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == asset_name)
            .ok_or_else(|| format!("no asset found matching {:?}", asset_name))?;

        // Download to temporary location first so that we can generate the SHA256 checksum
        let tmp_dir = format!("{}-tmp", Self::LANGUAGE_SERVER_ID);
        fs::create_dir_all(&tmp_dir).map_err(|e| format!("failed to create directory: {e}"))?;
        let temp_file_path = format!("{tmp_dir}/temporary-download");

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::Downloading,
        );

        zed::download_file(
            &asset.download_url,
            &temp_file_path,
            zed::DownloadedFileType::Uncompressed,
        )
        .map_err(|e| format!("failed to download file: {e}"))?;

        // Calculate checksum of downloaded file
        let file_contents = fs::read(&temp_file_path)
            .map_err(|e| format!("failed to read downloaded file: {e}"))?;

        let checksum = format!("{:x}", Sha256::digest(&file_contents));

        // Create directory with checksum
        let checksum_dir = format!("{}-{}", Self::LANGUAGE_SERVER_ID, checksum);
        fs::create_dir_all(&checksum_dir)
            .map_err(|e| format!("failed to create directory: {e}"))?;

        let binary_path = format!("{checksum_dir}/expert");

        if !fs::metadata(&binary_path).map_or(false, |stat| stat.is_file()) {
            // Move from temp location to final location
            fs::rename(&temp_file_path, &binary_path)
                .map_err(|e| format!("failed to move file: {e}"))?;

            zed::make_file_executable(&binary_path)?;

            util::remove_outdated_versions(Self::LANGUAGE_SERVER_ID, &checksum_dir)?;
        }

        // Clean up temp file
        fs::remove_dir_all(&tmp_dir).ok();

        self.cached_binary_path = Some(binary_path.clone());
        Ok(ExpertBinary {
            path: binary_path,
            args: binary_args,
        })
    }

    pub fn label_for_completion(&self, completion: Completion) -> Option<CodeLabel> {
        match completion.kind? {
            CompletionKind::Module
            | CompletionKind::Class
            | CompletionKind::Interface
            | CompletionKind::Struct => {
                let name = completion.label;

                let detail = completion
                    .detail
                    .map(|detail| format!(" ({detail})"))
                    .unwrap_or_default();

                let defmodule = "defmodule ";
                let heredoc_start = r#"@doc """\n"#;
                let heredoc_end = r#"\n""""#;
                let code = format!("{defmodule}{name}{heredoc_start}{detail}{heredoc_end}");

                let name_start = defmodule.len();
                let name_end = name_start + name.len();
                let detail_start = name_end + heredoc_start.len();
                let detail_end = detail_start + detail.len();

                Some(CodeLabel {
                    code,
                    spans: vec![
                        CodeLabelSpan::code_range(name_start..name_end),
                        CodeLabelSpan::code_range(detail_start..detail_end),
                    ],
                    filter_range: (0..name.len()).into(),
                })
            }
            _ => None,
        }
    }

    pub fn label_for_symbol(&self, symbol: Symbol) -> Option<CodeLabel> {
        let name = &symbol.name;

        let (code, filter_range, display_range) = match symbol.kind {
            SymbolKind::Module | SymbolKind::Class | SymbolKind::Interface | SymbolKind::Struct => {
                let defmodule = "defmodule ";
                let code = format!("{defmodule}{name}");
                let filter_range = 0..name.len();
                let display_range = defmodule.len()..defmodule.len() + name.len();
                (code, filter_range, display_range)
            }
            _ => return None,
        };

        Some(CodeLabel {
            spans: vec![CodeLabelSpan::code_range(display_range)],
            filter_range: filter_range.into(),
            code,
        })
    }
}
