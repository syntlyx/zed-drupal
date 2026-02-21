pub mod drupal_lsp;

use std::fs;
use zed_extension_api::{self as zed, LanguageServerId, Result};

// Common trait for language server managers that use npm packages
pub trait NpmLanguageServerManager {
    fn language_server_id(&self) -> &LanguageServerId;
    fn package_name(&self) -> &str;
    fn server_path(&self) -> &str;
    fn did_find_server(&self) -> bool;
    fn set_did_find_server(&mut self, value: bool);

    fn server_exists(&self) -> bool {
        fs::metadata(self.server_path()).map_or(false, |stat| stat.is_file())
    }

    fn server_script_path(&mut self) -> Result<String> {
        let server_exists = self.server_exists();
        if self.did_find_server() && server_exists {
            return Ok(self.server_path().to_string());
        }

        let id = self.language_server_id().clone();
        let package_name = self.package_name().to_string();
        let server_path_str = self.server_path().to_string();

        zed::set_language_server_installation_status(
            &id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );

        let version = zed::npm_package_latest_version(&package_name)?;

        if !server_exists
            || zed::npm_package_installed_version(&package_name)?.as_ref() != Some(&version)
        {
            zed::set_language_server_installation_status(
                &id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );

            eprintln!("[Drupal] Installing {}@{}...", package_name, version);

            let result = zed::npm_install_package(&package_name, &version);
            match result {
                Ok(()) => {
                    if !self.server_exists() {
                        return Err(format!(
                            "installed package '{}' did not contain expected path '{}'",
                            package_name, server_path_str
                        ));
                    }
                    eprintln!(
                        "[Drupal] Successfully installed {}@{}",
                        package_name, version
                    );
                }
                Err(error) => {
                    if !self.server_exists() {
                        return Err(error);
                    }
                }
            }
        }

        self.set_did_find_server(true);
        Ok(server_path_str)
    }
}
