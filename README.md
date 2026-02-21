# Drupal Extension for Zed

Enhanced Drupal development support for [Zed](https://zed.dev) editor.

## Features

- **Automatic Drupal project detection** via `composer.json` or filesystem probe
- **PHP code standards** with PHPCS/PHPCBF integration
- **Drupal LSP server** (`drupal-lsp-server`) for:
  - Service autocompletion in YAML files
  - Go to definition for services and classes
  - Real-time validation of `.services.yml` files

> **PHP language support** (hover, go-to-definition, rename, etc.) is handled by
> [Intelephense](https://intelephense.com) via Zed's built-in PHP extension — no
> additional setup required for that.

## Installation

Currently the extension must be installed manually.

1. Clone the repository:

```bash
git clone https://github.com/syntlyx/zed-drupal.git
```

2. Install as a dev extension in Zed:
   - Open the Extensions panel: `Cmd+Shift+X` (macOS) / `Ctrl+Shift+X` (Linux/Windows)
   - Click **Install Dev Extension**
   - Select the cloned repository folder
   - Zed will build and install the extension automatically

## Configuration

Add a `.zed/settings.json` file to the root of your Drupal project.

<details>
<summary>Full example</summary>

```jsonc
{
  // Treat Drupal-specific file extensions as PHP / YAML
  "file_types": {
    "PHP": [
      "module",
      "install",
      "theme",
      "profile",
      "inc",
      "test",
      "php",
      "info"
    ]
  },
  "languages": {
    "PHP": {
      // Enable drupal-lsp-server alongside Intelephense.
      // Disable any LSPs you don't use to reduce noise.
      "language_servers": [
        "drupal-lsp-server",
        "intelephense",
        "!tailwindcss-language-server",
        "!phpactor",
        "!phptools",
        "!emmet-language-server",
        "..."
      ],
      "format_on_save": "on",
      "formatter": {
        "language_server": {
          "name": "drupal-lsp-server"
        }
      }
    },
    "YAML": {
      "language_servers": ["drupal-lsp-server", "..."]
    }
  },
  "lsp": {
    "intelephense": {
      "settings": {
        // Include the "drupal" stub for Drupal-specific globals and functions.
        "stubs": [
          "apache", "bcmath", "bz2", "calendar", "com_dotnet", "Core",
          "ctype", "curl", "date", "dba", "dom", "enchant", "exif",
          "FFI", "fileinfo", "filter", "fpm", "ftp", "gd", "gettext",
          "gmp", "hash", "iconv", "imap", "intl", "json", "ldap",
          "libxml", "mbstring", "meta", "mysqli", "oci8", "odbc",
          "openssl", "pcntl", "pcre", "PDO", "pgsql", "Phar", "posix",
          "pspell", "random", "readline", "Reflection", "session",
          "shmop", "SimpleXML", "snmp", "soap", "sockets", "sodium",
          "SPL", "sqlite3", "standard", "superglobals", "sysvmsg",
          "sysvsem", "sysvshm", "tidy", "tokenizer", "uri", "xml",
          "xmlreader", "xmlrpc", "xmlwriter", "xsl", "Zend OPcache",
          "zip", "zlib", "imagick",
          "drupal"
        ],
        "environment": {
          "phpVersion": "8.4",
          // Adjust paths to match your webroot (web, docroot, etc.)
          "includePaths": [
            "vendor",
            "web/core"
          ]
        },
        "files": {
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
            "**/.git/**",
            "**/.phpstan/**"
          ]
        }
      }
    }
  }
}
```

</details>

### Notes

- Replace `web/core` in `intelephense.environment.includePaths` with your actual webroot
  (`docroot/core` for Acquia-style projects, or `core` if Drupal is in the project root).
- The `drupal-lsp-server` is installed automatically from npm on first use.

## License

[MIT](LICENSE)
