use zed_extension_api as zed;

/// Known webroot candidates: (webroot, path to Drupal.php relative to worktree root).
/// Used both for detecting Drupal and for finding the webroot when composer.json
/// doesn't have installer-paths.
const KNOWN_WEBROOTS: &[(&str, &str)] = &[
    ("web", "web/core/lib/Drupal.php"),
    ("docroot", "docroot/core/lib/Drupal.php"),
    (".", "core/lib/Drupal.php"),
];

/// Result of a successful Drupal project detection.
pub struct DrupalProject {
    /// The webroot directory, e.g. "web", "docroot", or "." for project root.
    pub drupal_root: String,
}

/// Detect whether the worktree is a Drupal project.
///
/// Reads `composer.json` once and checks:
/// 1. `require` / `require-dev` contains `drupal/core`
/// 2. Infers the webroot from `extra.installer-paths` (most reliable)
/// 3. Falls back to probing known locations (`web`, `docroot`, `.`)
///
/// Returns `None` if this is not a Drupal project or the webroot cannot be determined.
pub fn detect(worktree: &zed::Worktree) -> Option<DrupalProject> {
    // Read composer.json once — used for both detection and webroot resolution.
    let composer_content = worktree.read_text_file("composer.json").ok();

    if let Some(ref content) = composer_content {
        if let Some(project) = detect_from_composer(content, worktree) {
            return Some(project);
        }
    }

    // composer.json missing or doesn't reference drupal/core — probe known paths.
    detect_from_filesystem(worktree)
}

/// Try to detect and resolve a Drupal project using composer.json content.
fn detect_from_composer(content: &str, worktree: &zed::Worktree) -> Option<DrupalProject> {
    let json: serde_json::Value = serde_json::from_str(content).ok()?;

    if !has_drupal_core_dependency(&json) {
        return None;
    }

    eprintln!("[Drupal] Detected via drupal/core in composer.json");

    // Try installer-paths first (most reliable).
    // If composer.json declares drupal/core* AND has installer-paths, trust it —
    // Drupal.php may not exist yet (e.g. before `composer install`).
    if let Some(webroot) = parse_webroot_from_installer_paths(&json) {
        eprintln!("[Drupal] Webroot from installer-paths: {}", webroot);
        return Some(DrupalProject { drupal_root: webroot });
    }

    // installer-paths absent or webroot not verified — fall back to filesystem probe.
    detect_from_filesystem(worktree)
}

/// Composer package names that indicate a Drupal project.
/// `drupal/core-recommended` is the meta-package used by `drupal/recommended-project`.
const DRUPAL_CORE_PACKAGES: &[&str] = &["drupal/core", "drupal/core-recommended"];

/// Check whether the JSON has a Drupal core package in `require` or `require-dev`.
fn has_drupal_core_dependency(json: &serde_json::Value) -> bool {
    for section in &["require", "require-dev"] {
        if let Some(deps) = json.get(section).and_then(|v| v.as_object()) {
            if DRUPAL_CORE_PACKAGES.iter().any(|pkg| deps.contains_key(*pkg)) {
                return true;
            }
        }
    }
    false
}

/// Parse the webroot from `extra.installer-paths` by finding the entry mapped to
/// `"type:drupal-core"` and stripping the `/core` suffix.
///
/// Example:
/// ```json
/// { "extra": { "installer-paths": { "web/core": ["type:drupal-core"] } } }
/// ```
/// → returns `Some("web")`
fn parse_webroot_from_installer_paths(json: &serde_json::Value) -> Option<String> {
    let installer_paths = json
        .get("extra")?
        .get("installer-paths")?
        .as_object()?;

    for (path, types) in installer_paths {
        let is_drupal_core = types
            .as_array()
            .map_or(false, |arr| arr.iter().any(|t| t.as_str() == Some("type:drupal-core")));

        if is_drupal_core {
            let webroot = if path.ends_with("/core") {
                path[..path.len() - 5].to_string()
            } else if path == "core" {
                ".".to_string()
            } else {
                path.clone()
            };
            return Some(webroot);
        }
    }

    None
}

/// Probe the filesystem for known Drupal webroot locations.
fn detect_from_filesystem(worktree: &zed::Worktree) -> Option<DrupalProject> {
    for (webroot, drupal_php) in KNOWN_WEBROOTS {
        if worktree.read_text_file(drupal_php).is_ok() {
            eprintln!("[Drupal] Detected via filesystem probe: {}", drupal_php);
            return Some(DrupalProject {
                drupal_root: webroot.to_string(),
            });
        }
    }

    eprintln!("[Drupal] Not detected in {}", worktree.root_path().as_str());
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- has_drupal_core_dependency ---

    #[test]
    fn test_drupal_core_in_require() {
        let json: serde_json::Value = serde_json::from_str(r#"{
            "require": { "drupal/core": "^10" }
        }"#).unwrap();
        assert!(has_drupal_core_dependency(&json));
    }

    #[test]
    fn test_drupal_core_in_require_dev() {
        let json: serde_json::Value = serde_json::from_str(r#"{
            "require-dev": { "drupal/core": "^10" }
        }"#).unwrap();
        assert!(has_drupal_core_dependency(&json));
    }

    #[test]
    fn test_no_drupal_core() {
        let json: serde_json::Value = serde_json::from_str(r#"{
            "require": { "symfony/console": "^6" }
        }"#).unwrap();
        assert!(!has_drupal_core_dependency(&json));
    }

    #[test]
    fn test_drupal_core_recommended_in_require() {
        let json: serde_json::Value = serde_json::from_str(r#"{
            "require": { "drupal/core-recommended": "^10" }
        }"#).unwrap();
        assert!(has_drupal_core_dependency(&json));
    }

    #[test]
    fn test_drupal_core_string_in_description_not_detected() {
        // The old string-search approach would match this; JSON parsing does not.
        let json: serde_json::Value = serde_json::from_str(r#"{
            "description": "A project that mentions drupal/core in its description"
        }"#).unwrap();
        assert!(!has_drupal_core_dependency(&json));
    }

    // --- parse_webroot_from_installer_paths ---

    #[test]
    fn test_parse_webroot_standard() {
        let json: serde_json::Value = serde_json::from_str(r#"{
            "extra": {
                "installer-paths": {
                    "web/core": ["type:drupal-core"],
                    "web/modules/contrib/{$name}": ["type:drupal-module"]
                }
            }
        }"#).unwrap();
        assert_eq!(parse_webroot_from_installer_paths(&json), Some("web".to_string()));
    }

    #[test]
    fn test_parse_webroot_docroot() {
        let json: serde_json::Value = serde_json::from_str(r#"{
            "extra": {
                "installer-paths": {
                    "docroot/core": ["type:drupal-core"]
                }
            }
        }"#).unwrap();
        assert_eq!(parse_webroot_from_installer_paths(&json), Some("docroot".to_string()));
    }

    #[test]
    fn test_parse_webroot_root() {
        let json: serde_json::Value = serde_json::from_str(r#"{
            "extra": {
                "installer-paths": {
                    "core": ["type:drupal-core"]
                }
            }
        }"#).unwrap();
        assert_eq!(parse_webroot_from_installer_paths(&json), Some(".".to_string()));
    }

    #[test]
    fn test_parse_webroot_custom() {
        let json: serde_json::Value = serde_json::from_str(r#"{
            "extra": {
                "installer-paths": {
                    "public/core": ["type:drupal-core"]
                }
            }
        }"#).unwrap();
        assert_eq!(parse_webroot_from_installer_paths(&json), Some("public".to_string()));
    }

    #[test]
    fn test_parse_webroot_no_drupal_core_type() {
        let json: serde_json::Value = serde_json::from_str(r#"{
            "extra": {
                "installer-paths": {
                    "web/modules/{$name}": ["type:drupal-module"]
                }
            }
        }"#).unwrap();
        assert_eq!(parse_webroot_from_installer_paths(&json), None);
    }

    #[test]
    fn test_parse_webroot_missing_installer_paths() {
        let json: serde_json::Value = serde_json::from_str(r#"{ "extra": {} }"#).unwrap();
        assert_eq!(parse_webroot_from_installer_paths(&json), None);
    }
}
