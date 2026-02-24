use std::fs;

use zed::LanguageServerId;
use zed_extension_api::lsp::{Completion, CompletionKind, Symbol, SymbolKind};
use zed_extension_api::settings::LspSettings;
use zed_extension_api::{self as zed, CodeLabel, CodeLabelSpan, Result};

use crate::language_servers::util;

pub struct ExpertBinary {
    pub path: String,
    pub args: Vec<String>,
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
        let binary_settings = LspSettings::for_worktree(Self::LANGUAGE_SERVER_ID, worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.binary);
        let binary_args = binary_settings
            .as_ref()
            .and_then(|binary_settings| binary_settings.arguments.clone())
            .unwrap_or_else(|| vec!["--stdio".to_string()]);

        if let Some(path) = binary_settings.and_then(|binary_settings| binary_settings.path) {
            return Ok(ExpertBinary {
                path,
                args: binary_args,
            });
        }

        if let Some(path) = worktree.which(Self::LANGUAGE_SERVER_ID) {
            return Ok(ExpertBinary {
                path,
                args: binary_args,
            });
        }

        if let Some(path) = &self.cached_binary_path {
            if fs::metadata(path).is_ok_and(|stat| stat.is_file()) {
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
        let release = match zed::latest_github_release(
            "elixir-lang/expert",
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: true,
            },
        ) {
            Ok(release) => release,
            Err(_) => {
                if let Some(path) = util::find_existing_binary(Self::LANGUAGE_SERVER_ID) {
                    self.cached_binary_path = Some(path.clone());
                    return Ok(ExpertBinary {
                        path,
                        args: binary_args,
                    });
                }
                return Err("failed to download latest github release".to_string());
            }
        };

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

        let checksum_asset = release
            .assets
            .iter()
            .find(|asset| asset.name == "expert_checksums.txt")
            .ok_or_else(|| "no checksums file found in release".to_string())?;

        let checksums_dir = format!("{}-checksums", Self::LANGUAGE_SERVER_ID);
        fs::create_dir_all(&checksums_dir)
            .map_err(|e| format!("failed to create directory: {e}"))?;

        let checksums_path = format!("{checksums_dir}/expert_checksums.txt");

        zed::download_file(
            &checksum_asset.download_url,
            &checksums_path,
            zed::DownloadedFileType::Uncompressed,
        )
        .map_err(|e| format!("failed to download checksums file: {e}"))?;

        let checksums_content = fs::read_to_string(&checksums_path)
            .map_err(|e| format!("failed to read checksums file: {e}"))?;

        fs::remove_dir_all(&checksums_dir)
            .map_err(|e| format!("failed to remove checksums directory: {e}"))?;

        let truncated_checksum = checksums_content
            .lines()
            .find(|line| line.ends_with(&asset_name))
            .and_then(|line| line.split_whitespace().next())
            .ok_or_else(|| format!("checksum not found for {}", asset_name))?
            .chars()
            .take(8)
            .collect::<String>();

        let expert_dir = format!("{}-{}", Self::LANGUAGE_SERVER_ID, truncated_checksum);
        fs::create_dir_all(&expert_dir).map_err(|e| format!("failed to create directory: {e}"))?;

        let binary_path = format!("{}/{}", expert_dir, Self::LANGUAGE_SERVER_ID);

        if !fs::metadata(&binary_path).is_ok_and(|stat| stat.is_file()) {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );

            zed::download_file(
                &asset.download_url,
                &binary_path,
                zed::DownloadedFileType::Uncompressed,
            )
            .map_err(|e| format!("failed to download file: {e}"))?;

            zed::make_file_executable(&binary_path)?;

            util::remove_outdated_versions(Self::LANGUAGE_SERVER_ID, &expert_dir)?;
        }

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
