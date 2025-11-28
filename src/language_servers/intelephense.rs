use super::NpmLanguageServerManager;
use serde_json::Value;
use std::env;
use zed::Command;
use zed_extension_api::{self as zed, LanguageServerId, Result, Worktree};

const SERVER_PATH: &str = "node_modules/intelephense/lib/intelephense.js";
const PACKAGE_NAME: &str = "intelephense";

pub struct IntelephenseManager {
    id: LanguageServerId,
    did_find_server: bool,
}

impl IntelephenseManager {
    pub fn new(id: LanguageServerId) -> Self {
        Self {
            id,
            did_find_server: false,
        }
    }

    pub fn build_options(&self, drupal_root: &str) -> Value {
        let core_path = if drupal_root == "." {
            "core".to_string()
        } else {
            format!("{}/core", drupal_root)
        };

        let modules_path = if drupal_root == "." {
            "modules".to_string()
        } else {
            format!("{}/modules", drupal_root)
        };

        let themes_path = if drupal_root == "." {
            "themes".to_string()
        } else {
            format!("{}/themes", drupal_root)
        };

        eprintln!(
            "[Drupal] Intelephense config - root: {}, paths: vendor, {}, {}, {}",
            drupal_root, core_path, modules_path, themes_path
        );

        serde_json::json_internal!({
            "intelephense":{
                "stubs": [
                    "apache", "bcmath", "bz2", "calendar", "com_dotnet", "Core", "ctype",
                    "curl", "date", "dba", "dom", "enchant", "exif", "FFI", "fileinfo", "filter",
                    "fpm", "ftp", "gd", "gettext", "gmp", "hash", "iconv", "imap", "intl", "json",
                    "ldap", "libxml", "mbstring", "meta", "mysqli", "oci8", "odbc", "openssl",
                    "pcntl", "pcre", "PDO", "pgsql", "Phar", "posix", "pspell", "random", "readline",
                    "Reflection", "session", "shmop", "SimpleXML", "snmp", "soap", "sockets",
                    "sodium", "SPL", "sqlite3", "standard", "superglobals", "sysvmsg", "sysvsem",
                    "sysvshm", "tidy", "tokenizer", "uri", "xml", "xmlreader", "xmlrpc",
                    "xmlwriter", "xsl", "Zend OPcache", "zip", "zlib"
                ],
                "environment": {
                    "includePaths": [
                        "vendor",
                        core_path,
                        modules_path,
                        themes_path
                    ]
                },
                "files":{
                    "maxSize": 5000000,
                    "associations": [
                        "*.inc", "*.module", "*.install", "*.theme",
                        "*.profile", "*.test", "*.php", "*.info"
                    ],
                    "exclude": [
                        "**/node_modules/**",
                        "**/vendor/**/Tests/**",
                        "**/vendor/**/tests/**",
                        "**/core/tests/**",
                        "**/core/modules/*/tests/**",
                        "**/sites/simpletest/**",
                        "**/files/**",
                        "**/sites/default/files/**",
                        "**/.git/**"
                    ]
                },
                "format":{
                    "enable": true,
                    "braces": "k&r"
                }
            }
        })
    }

    pub fn command(&mut self, worktree: &Worktree) -> Result<Command> {
        if let Some(path) = worktree.which("intelephense") {
            eprintln!(
                "[Drupal] Using globally installed intelephense at: {}",
                path
            );
            return Ok(Command {
                command: path,
                args: vec!["--stdio".to_string()],
                env: Default::default(),
            });
        }

        eprintln!("[Drupal] Using local npm installation of intelephense");
        let server_path = self.server_script_path()?;
        let full_path = env::current_dir()
            .unwrap()
            .join(&server_path)
            .to_string_lossy()
            .to_string();

        eprintln!("[Drupal] Starting intelephense at: {}", full_path);

        Ok(Command {
            command: zed::node_binary_path()?,
            args: vec![full_path, "--stdio".to_string()],
            env: Default::default(),
        })
    }
}

impl NpmLanguageServerManager for IntelephenseManager {
    fn language_server_id(&self) -> &LanguageServerId {
        &self.id
    }

    fn package_name(&self) -> &str {
        PACKAGE_NAME
    }

    fn server_path(&self) -> &str {
        SERVER_PATH
    }

    fn did_find_server(&self) -> bool {
        self.did_find_server
    }

    fn set_did_find_server(&mut self, value: bool) {
        self.did_find_server = value;
    }
}
