# Mizly

A desktop app for searching your documents with AI. Upload files, organize them into stores, and ask questions in natural language.

Built with Tauri, Vue 3, and Gemini API.

## Features

- **Document Stores** - Organize files into searchable collections
- **AI-Powered Search** - Query documents using natural language
- **Multi-format Support** - PDF, text, and more
- **Offline-First** - Local SQLite database with cloud sync
- **Multi-language** - English, Japanese, Traditional Chinese
- **Cross-Platform** - macOS, Windows, Linux

## Requirements

- [Bun](https://bun.sh/)
- [Rust](https://www.rust-lang.org/tools/install)
- [Gemini API Key](https://aistudio.google.com/apikey)

## Quick Start

```bash
# Install dependencies
bun install

# Run in development mode
bun run tauri dev
```

## Build

```bash
bun run tauri build
```

## Configuration

Set your Gemini API key in the app settings (Settings > API Key).

## Tech Stack

| Layer | Technology |
|-------|------------|
| Frontend | Vue 3, Vite, Tailwind CSS 4, Pinia |
| Backend | Rust, Tauri 2, SQLite |
| AI | Gemini API |

## Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `Cmd+N` | New store |
| `Cmd+D` | Delete |
| `Tab` | Open chat |
| `Esc` | Go back |
| `↑/↓` | Navigate |
| `Enter` | Select |

## Project Structure

```
src/           # Vue frontend
src-tauri/     # Rust backend
```

## Built With

This project was built with [Claude Code](https://claude.ai/code).

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
