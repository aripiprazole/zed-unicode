//! MIT License
//!
//! Copyright (c) 2024 Marshall Bowers

use std::fs;
use zed_extension_api::{
    self as zed, settings::LspSettings, Command, LanguageServerId, Result, Worktree,
};

struct UnicodeExtension {
    cached_ls_binary_path: Option<String>,
}

impl UnicodeExtension {
    fn target_triple(&self, binary: &str) -> Result<String, String> {
        let (platform, arch) = zed::current_platform();
        let (arch, os) = {
            let arch = match arch {
                zed::Architecture::Aarch64 => "aarch64",
                zed::Architecture::X8664 if binary == "unicode-ls" => "x86_64",
                _ => return Err(format!("unsupported architecture: {arch:?}")),
            };

            let os = match platform {
                zed::Os::Mac => "apple-darwin",
                zed::Os::Linux => "unknown-linux-gnu",
                zed::Os::Windows => "pc-windows-msvc",
            };

            (arch, os)
        };

        Ok(format!("{binary}-{arch}-{os}"))
    }

    fn download(
        &self,
        language_server_id: &LanguageServerId,
        binary: &str,
        repo: &str,
    ) -> Result<String> {
        let release = zed::latest_github_release(
            repo,
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )?;

        let target_triple = self.target_triple(binary)?;

        let asset_name = format!("{target_triple}.zip");
        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == asset_name)
            .ok_or_else(|| format!("no asset found matching {:?}", asset_name))?;

        let version_dir = format!("{binary}-{}", release.version);
        let binary_path = format!("{version_dir}/{binary}");

        if !fs::metadata(&binary_path).map_or(false, |stat| stat.is_file()) {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );

            zed::download_file(
                &asset.download_url,
                &version_dir,
                zed::DownloadedFileType::Zip,
            )
            .map_err(|err| format!("failed to download file: {err}"))?;

            let entries = fs::read_dir(".")
                .map_err(|err| format!("failed to list working directory {err}"))?;

            for entry in entries {
                let entry = entry.map_err(|err| format!("failed to load directory entry {err}"))?;
                if let Some(file_name) = entry.file_name().to_str() {
                    if file_name.starts_with(binary) && file_name != version_dir {
                        fs::remove_dir_all(entry.path()).ok();
                    }
                }
            }
        }

        zed::make_file_executable(&binary_path)?;

        Ok(binary_path)
    }

    fn language_server_binary_path(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<String, String> {
        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );

        if let Some(path) = worktree.which("unicode-ls") {
            return Ok(path.clone());
        }

        let target_triple = self.target_triple("unicode-ls")?;
        if let Some(path) = worktree.which(&target_triple) {
            return Ok(path.clone());
        }

        if let Some(path) = &self.cached_ls_binary_path {
            if fs::metadata(path).map_or(false, |stat| stat.is_file()) {
                return Ok(path.clone());
            }
        }

        let binary_path =
            self.download(language_server_id, "unicode-ls", "aripiprazole/zed-unicode")?;

        self.cached_ls_binary_path = Some(binary_path.clone());

        Ok(binary_path)
    }
}

impl zed::Extension for UnicodeExtension {
    fn new() -> Self {
        Self {
            cached_ls_binary_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<Command> {
        let ls_binary_path = self.language_server_binary_path(language_server_id, worktree)?;
        let settings = LspSettings::for_worktree("unicode", worktree)
            .map(|lsp_settings| lsp_settings.settings)
            .unwrap_or_default()
            .unwrap_or(serde_json::json!({
                "include_all_symbols": false
            }));

        let args = settings
            .get("include_all_symbols")
            .and_then(|x| x.as_bool())
            .filter(|x| *x)
            .map(|_| vec!["--include_all_symbols".into()])
            .unwrap_or_default();

        Ok(Command {
            args,
            command: ls_binary_path,
            env: worktree.shell_env(),
        })
    }
}

zed::register_extension!(UnicodeExtension);
