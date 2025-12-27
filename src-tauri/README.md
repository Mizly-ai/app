# Tauri Backend (src-tauri)

Rust backend for the Mizly Desktop Application.

## Directory Structure

```
src/
├── commands/               # Tauri command handlers
│   ├── chat.rs             # AI chat query commands
│   ├── documents.rs        # Document management commands
│   ├── mod.rs
│   ├── stores.rs           # Store CRUD commands
│   └── window.rs           # Window control commands
│
├── config.rs               # Application configuration (window dimensions)
│
├── db/                     # SQLite database layer
│   ├── connection.rs       # Database connection management
│   ├── documents.rs        # Document queries
│   ├── migrations.rs       # Schema migrations
│   ├── mod.rs
│   └── stores.rs           # Store queries
│
├── gemini/                 # Gemini API client
│   ├── client.rs           # HTTP client for Gemini API
│   ├── mod.rs
│   └── types.rs            # API request/response types
│
├── polling/                # Background polling
│   ├── document_status.rs  # Document processing status checker
│   └── mod.rs
│
├── settings/               # Application settings
│   └── mod.rs              # API key storage management
│
├── sync/                   # Background synchronization
│   ├── background_sync.rs  # Store/document upload sync
│   └── mod.rs
│
├── window/                 # Window management
│   ├── macos.rs            # macOS-specific (NSApplication)
│   └── mod.rs              # Cross-platform window utilities
│
├── shortcuts.rs            # Global keyboard shortcuts
├── state.rs                # Application state management
├── tray.rs                 # System tray setup
├── lib.rs                  # Application entry point
└── main.rs                 # Binary entry point
```

## Key Dependencies

| Crate | Purpose |
|-------|---------|
| `tauri` | Desktop framework |
| `rusqlite` | SQLite database |
| `reqwest` | HTTP client |
| `tokio` | Async runtime |
| `serde` | Serialization |

## Development

```bash
# Check compilation
cargo check

# Run with frontend
cd .. && bun run tauri dev

# Build release
cd .. && bun run tauri build
```

## Architecture Notes

### Settings Management

API keys are stored securely using the Tauri app data directory:

```rust
use crate::settings;

// Get API key
let key = settings::get_api_key_sync(&app_handle);

// Set API key (validates format)
settings::set_api_key(app_handle, api_key)?;
```

### Background Tasks

Background sync and polling run in separate async tasks:

- `sync/background_sync.rs` - Syncs pending stores and documents to Gemini API
- `polling/document_status.rs` - Polls document processing status

### Tauri Commands

All commands are registered in `lib.rs`:

```rust
tauri::generate_handler![
    // stores
    commands::stores::create_store,
    commands::stores::list_stores,
    // ... more commands
]
```
