use std::fs;

use zed_extension_api::{
    self as zed, CodeLabel, CodeLabelSpan, LanguageServerId, Result, Worktree,
    lsp::{Completion, CompletionKind, Symbol},
    serde_json::Value,
};

use crate::language_servers::{config, util};

struct LexicalBinary {
    path: String,
    args: Vec<String>,
}

pub struct Lexical {
    cached_binary_path: Option<String>,
}

impl Lexical {
    pub const LANGUAGE_SERVER_ID: &'static str = "lexical";

    pub fn new() -> Self {
        Self {
            cached_binary_path: None,
        }
    }

    pub fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<zed::Command> {
        let lexical = self.language_server_binary(language_server_id, worktree)?;

        Ok(zed::Command {
            command: lexical.path,
            args: lexical.args,
            env: Default::default(),
        })
    }

    fn language_server_binary(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<LexicalBinary> {
        let (platform, _arch) = zed::current_platform();
        if platform == zed::Os::Windows {
            return Err(format!("unsupported platform: {platform:?}"));
        }

        let binary_name = format!("{}/bin/start_lexical.sh", Self::LANGUAGE_SERVER_ID);
        let binary_settings = config::get_binary_settings(Self::LANGUAGE_SERVER_ID, worktree);
        let binary_args = config::get_binary_args(&binary_settings).unwrap_or_default();
        let port_wrapper = format!("{}/priv/port_wrapper.sh", Self::LANGUAGE_SERVER_ID);
        let debug_shell = format!("{}/bin/debug_shell.sh", Self::LANGUAGE_SERVER_ID);

        if let Some(binary_path) = config::get_binary_path(&binary_settings) {
            return Ok(LexicalBinary {
                path: binary_path,
                args: binary_args,
            });
        }

        if let Some(binary_path) = worktree.which(Self::LANGUAGE_SERVER_ID) {
            return Ok(LexicalBinary {
                path: binary_path,
                args: binary_args,
            });
        }

        if let Some(binary_path) = &self.cached_binary_path
            && fs::metadata(binary_path).is_ok_and(|stat| stat.is_file())
        {
            return Ok(LexicalBinary {
                path: binary_path.clone(),
                args: binary_args,
            });
        }

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );

        let release = match zed::latest_github_release(
            "lexical-lsp/lexical",
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        ) {
            Ok(release) => release,
            Err(_) => {
                if let Some(binary_path) =
                    util::find_existing_binary(Self::LANGUAGE_SERVER_ID, &binary_name)
                {
                    self.cached_binary_path = Some(binary_path.clone());
                    return Ok(LexicalBinary {
                        path: binary_path,
                        args: binary_args,
                    });
                }
                return Err("failed to download latest github release".to_string());
            }
        };

        let asset_name = format!(
            "{}-{version}.zip",
            Self::LANGUAGE_SERVER_ID,
            version = release.version,
        );

        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == asset_name)
            .ok_or_else(|| format!("no asset found matching {:?}", asset_name))?;

        let version_dir = format!("{}-{}", Self::LANGUAGE_SERVER_ID, release.version);
        let binary_path = format!("{}/{}", version_dir, binary_name);
        let port_path = format!("{}/{}", version_dir, port_wrapper);
        let debug_path = format!("{}/{}", version_dir, debug_shell);

        if !fs::metadata(&binary_path).is_ok_and(|stat| stat.is_file()) {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );

            zed::download_file(
                &asset.download_url,
                &version_dir,
                zed::DownloadedFileType::Zip,
            )
            .map_err(|e| format!("failed to download file: {e}"))?;

            zed::make_file_executable(&binary_path)?;
            zed::make_file_executable(&port_path)?;
            zed::make_file_executable(&debug_path)?;

            util::remove_outdated_versions(Self::LANGUAGE_SERVER_ID, &version_dir)?;
        }

        self.cached_binary_path = Some(binary_path.clone());
        Ok(LexicalBinary {
            path: binary_path,
            args: binary_args,
        })
    }

    pub fn language_server_initialization_options(
        &mut self,
        _worktree: &Worktree,
    ) -> Result<Option<Value>> {
        Ok(None)
    }

    pub fn language_server_workspace_configuration(
        &mut self,
        _worktree: &Worktree,
    ) -> Result<Option<Value>> {
        Ok(None)
    }

    pub fn label_for_completion(&self, completion: Completion) -> Option<CodeLabel> {
        match completion.kind? {
            CompletionKind::Module
            | CompletionKind::Class
            | CompletionKind::Interface
            | CompletionKind::Struct => {
                let name = completion.label;
                let defmodule = "defmodule ";
                let code = format!("{defmodule}{name}");

                Some(CodeLabel {
                    code,
                    spans: vec![CodeLabelSpan::code_range(
                        defmodule.len()..defmodule.len() + name.len(),
                    )],
                    filter_range: (0..name.len()).into(),
                })
            }
            CompletionKind::Function | CompletionKind::Constant => {
                let name = completion.label;
                let def = "def ";
                let code = format!("{def}{name}");

                Some(CodeLabel {
                    code,
                    spans: vec![CodeLabelSpan::code_range(def.len()..def.len() + name.len())],
                    filter_range: (0..name.len()).into(),
                })
            }
            CompletionKind::Operator => {
                let name = completion.label;
                let def_a = "def a ";
                let code = format!("{def_a}{name} b");

                Some(CodeLabel {
                    code,
                    spans: vec![CodeLabelSpan::code_range(
                        def_a.len()..def_a.len() + name.len(),
                    )],
                    filter_range: (0..name.len()).into(),
                })
            }
            _ => None,
        }
    }

    pub fn label_for_symbol(&self, _symbol: Symbol) -> Option<CodeLabel> {
        None
    }
}
