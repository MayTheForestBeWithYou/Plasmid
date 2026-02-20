# Plasmid
**Plasmid** is a declarative, cross-platform system environment manager written in Rust.

It treats your computer like a host cell and your configuration like genetic material. Whether you are on **Linux**, **macOS**, or **Windows**, Plasmid ensures your environment expresses the exact traits you defined in your Git repository.

It handles **Dotfiles** (via symlinks), **Software** (via package manager), and **Secrets** (via templating) in a single binary.

## Key Features
* **Mirror Mode (Zero-Config):** Your repository structure _is_ your configuration. Plasmid mirrors your repo layout to your Home directory.
* **Symlinking:** We never symlink directories. Plasmid "explodes" directories, walking the tree and linking individual files. This allows you to mix managed and unmanged files in the same folder safely.
* **Profiles:** Switch contexts easily. Apply a base configuration, then layer on specific profiles like `--profile work` or `--profile personal`.
* **Windows-First Architecture:** First-class Windows support. If Admin priviliges or Developer Mode are missing, Plasmid intelligently falls back to **Hardlinks**, ensuring your setup works even on locked-down corporate laptops.
* **Secret Injection:** Use `plasmid.local.toml` and `.plz` templates to inject API keys and emails into your config without committing them to Git.

## Installation
Plasmid is currently unavailable. This section will be updated when the first release is available.

## License
Distributed under the MIT License. See [LICENSE](LICENSE) for more information.
