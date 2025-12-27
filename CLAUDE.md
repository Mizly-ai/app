<!-- OPENSPEC:START -->
# OpenSpec Instructions

These instructions are for AI assistants working in this project.

Always open `@/openspec/AGENTS.md` when the request:
- Mentions planning or proposals (words like proposal, spec, change, plan)
- Introduces new capabilities, breaking changes, architecture shifts, or big performance/security work
- Sounds ambiguous and you need the authoritative spec before coding

Use `@/openspec/AGENTS.md` to learn:
- How to create and apply change proposals
- Spec format and conventions
- Project structure and guidelines

Keep this managed block so 'openspec update' can refresh the instructions.

<!-- OPENSPEC:END -->

# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## COMMUNICATION RESTRICTIONS

- Never use the phrase "你說的對" (you're right) or similar affirming statements
- Focus on technical analysis and solutions rather than agreeing with statements
- Provide direct, actionable feedback without unnecessary affirmations

## SPECIALIZED TOOLS POLICY

⚠️ **IMPORTANT**: Use the following specialized tools instead of traditional Unix commands: (Install if missing)

| Task Type | Must Use | Do Not Use |
|-----------|----------|------------|
| Find Files | `fd` | `find`, `ls -R` |
| Search Text | `rg` (ripgrep) | `grep`, `ag` |
| Analyze Code Structure | `ast-grep` | `grep`, `sed` |
| Interactive Selection | `fzf` | Manual filtering |
| Process JSON | `jq` | `python -m json.tool` |
| Process YAML/XML | `yq` | Manual parsing |

## CODING STANDARDS

### Vue Components (*.vue)
⚠️ **CRITICAL RULES**:
- **No `<style>` tags allowed**: All Vue components must NEVER use `<style>` or `<style scoped>` tags
- **Must use Tailwind CSS**: Always use Tailwind CSS utility classes for styling
- **No CSS file generation**: No CSS files should be created except for Tailwind configuration

### Vue Composables (`src/composables/`)

#### When to Create a Composable
- **Reusable logic**: Code used by 2+ components (e.g., `useListNavigation`)
- **Complex business logic**: Extract from views to keep them focused on UI
- **Testability**: Logic that needs unit testing independent of UI

#### Naming Conventions
- File: `use<Feature>.js` (e.g., `useListNavigation.js`, `useAiChat.js`)
- Export: `export function use<Feature>() { ... }`

#### Standard Pattern
```javascript
import { ref, computed, onMounted, onUnmounted } from 'vue'

export function useFeatureName(options = {}) {
  // 1. Destructure options
  const { someOption, anotherOption } = options

  // 2. Define reactive state
  const state = ref(initialValue)

  // 3. Define computed properties
  const derivedState = computed(() => /* ... */)

  // 4. Define methods
  const doSomething = () => { /* ... */ }

  // 5. Lifecycle hooks (if needed)
  onMounted(() => { /* ... */ })
  onUnmounted(() => { /* ... */ })

  // 6. Return public API
  return {
    // State
    state,
    derivedState,
    // Methods
    doSomething
  }
}
```

#### Existing Composables
| Composable | Purpose | Used By |
|------------|---------|---------|
| `useGlobalKeyboard` | Global keyboard shortcuts | All views |
| `useListNavigation` | List up/down navigation, scroll | 6 views |
| `useWindowDrag` | Window drag behavior | Layout |
| `useCollectionList` | Collections list operations | `collections/index.vue` |
| `useCollectionDetail` | Collection detail operations | `collections/show.vue` |
| `useFileSelection` | Local/Google Drive file selection | `collections/new.vue` |
| `useCollectionForm` | Collection form validation | `collections/new.vue` |
| `useAiChat` | AI chat messaging and streaming | `chats/show.vue` |
| `useGoogleDriveBrowser` | Google Drive folder browsing | `google-drive/index.vue` |
| `useHomeSettings` | Home page settings items | `home/index.vue` |
| `useLocaleSelector` | Language selection | `settings/locales/index.vue` |

### JavaScript/TypeScript Code
- **Must follow Biome rules**: All `*.js` files and `<script setup>` blocks in Vue components must comply with Biome formatting rules
- **Formatting requirements**:
  - Use 2-space indentation
  - Use single quotes
  - No trailing commas
  - Add semicolons when necessary

### Rust Backend (`src-tauri/src/`)

#### Configuration Management
- **Centralized in `config.rs`**: All environment variables must be accessed through `env_config()`
- **Never use `std::env::var()` directly** in other modules
- Add new env vars to `EnvConfig` struct in `config.rs`

```rust
// ✅ Correct
use crate::config::env_config;
let config = env_config();
let api_url = &config.api_base_url;

// ❌ Wrong
let api_url = std::env::var("API_URL").unwrap();
```

#### Error Handling
- **API errors**: Use `ApiResponseExt` trait from `api/error.rs`
- **Use `thiserror`** for custom error types
- Return `Result<T, String>` for Tauri commands (for frontend compatibility)

```rust
use crate::api::error::ApiResponseExt;

// ✅ Correct - unified error handling
let data: MyType = response.handle_json().await.map_err(|e| e.to_string())?;

// ❌ Wrong - manual status check
if !response.status().is_success() {
    return Err(format!("Error: {}", response.text().await?));
}
```

#### Module Organization
| Rule | Description |
|------|-------------|
| Feature as directory | `oauth/`, `google_drive/`, `api/` |
| Types in `types.rs` | Shared types within a module |
| Re-export in `mod.rs` | Only export what's needed externally |
| Commands near business logic | e.g., `google_drive/api.rs` contains both API calls and Tauri commands |

#### Background Tasks (`sync/`, `polling/`)
- Split large loops into **independent functions** that return `bool` (work done)
- Use `tokio::select!` for cancellable waits
- Emit events to frontend via `app_handle.emit()`

```rust
// ✅ Correct - split into testable functions
async fn sync_pending_collections(...) -> bool { ... }
async fn sync_pending_documents(...) -> bool { ... }

// Main loop
loop {
    let mut has_work = false;
    has_work |= sync_pending_collections(&app, &db, &api).await;
    has_work |= sync_pending_documents(&app, &db, &api).await;
    // ...
}
```

#### Tauri Commands
- Declare with `#[tauri::command]`
- Register in `lib.rs` under `tauri::generate_handler![]`
- Use `State<'_, Arc<T>>` for shared state access

## Project Overview

This is a Tauri + Vue 3 desktop application that uses:
- **Frontend**: Vue 3 with Composition API (`<script setup>` SFCs), Vite as build tool
- **Backend**: Rust with Tauri framework for native desktop capabilities
- **Styling**: Tailwind CSS 4 (NO custom CSS allowed)
- **Package Manager**: Bun (as evidenced by bun.lock and package.json scripts)
- **Code Formatter**: Biome for JavaScript/TypeScript formatting

## Key Commands

### Development
```bash
# Start the Tauri development server (frontend + backend)
bun run tauri dev

# Start only the Vite dev server (frontend only)
bun run dev
```

### Building
```bash
# Build the complete desktop application
bun run tauri build

# Build only the frontend
bun run build
```

### Tauri-specific commands
```bash
# Run any Tauri CLI command
bun run tauri [command]
```

## Architecture

### Frontend Structure
- **Entry Point**: `src/main.js` - Initializes Vue app
- **Main Component**: `src/App.vue` - Root Vue component with example Tauri IPC communication
- **Tauri API**: Uses `@tauri-apps/api` for communicating with Rust backend via the `invoke` function

### Backend Structure (`src-tauri/src/`)
- **Entry Point**: `main.rs` → `lib.rs` - Application bootstrap and command registration
- **Core Modules**:
  - `config.rs` - Application configuration constants
  - `gemini/` - Gemini API client for file search
  - `db/` - SQLite database layer
  - `sync/` - Background synchronization tasks
  - `polling/` - Document status polling
  - `commands/` - Tauri command handlers
  - `settings/` - API key and settings management
  - `window/` - Window management (cross-platform)
- **See**: `src-tauri/README.md` for detailed architecture

### Configuration Files
- **Tauri Config**: `src-tauri/tauri.conf.json` - Defines:
  - Dev server URL: `http://localhost:1420`
  - Application identifier: `ai.mizly.app`
  - Build commands use `bun` instead of npm
- **Vite Config**: `vite.config.js` - Sets up:
  - Port 1420 for dev server (must match Tauri config)
  - HMR on port 1421
  - Ignores `src-tauri` directory during watch
- **Biome Config**: `biome.json` - JavaScript formatting with:
  - 2-space indentation
  - Single quotes
  - No trailing commas
  - Semicolons as needed

## Inter-Process Communication (IPC)

The frontend communicates with the Rust backend through Tauri's IPC system:
1. Frontend calls `invoke("command_name", { args })` from `@tauri-apps/api/core`
2. Backend handles the command via functions marked with `#[tauri::command]`
3. Commands must be registered in `tauri::generate_handler![]` macro in `lib.rs`

Example flow in the template:
- Frontend: `invoke("greet", { name: "value" })` in App.vue
- Backend: `greet(name: &str) -> String` function in lib.rs
- The function is registered in the invoke_handler

## Development Notes

- The project uses Rust's Cargo build system for the backend (see `src-tauri/Cargo.toml`)
- Frontend assets are built to `dist/` directory
- Tauri looks for the built frontend in `../dist` relative to `src-tauri`
- The dev server must be running on port 1420 for Tauri dev mode to work correctly
