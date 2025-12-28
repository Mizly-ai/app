# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.2] - 2025-12-28

### Bug Fixes
- Fix updater download progress calculation (now accumulates bytes correctly)
- Add error handling and status display during download process
- Update error messages to show actual error details (EN, JA, ZH-TW)

## [0.1.1] - 2025-12-27

### Bug Fixes
- Cross-platform build compatibility
- Move chrono to general dependencies (was accidentally in macOS-only)
- Add Windows/Linux support for open_directory and open_file commands
- Wrap macOS-specific RunEvent::Reopen in cfg attribute
- Comment out Apple code signing in workflow (optional for open source)

## [0.1.0] - 2025-12-27

### Features
- Initial release of Mizly desktop application
- Tauri 2.0 + Vue 3 architecture
- AI-powered file search with Gemini integration
- Google Drive file support
- SQLite local database
- Background sync and polling
- Cross-platform support (macOS, Windows)
- Auto-update functionality
