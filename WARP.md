# WARP.md

This file provides guidance to WARP (warp.dev) when working with code in this repository.

## Project Overview

**DoIt** is a CLI todo/task management application written in Rust. It supports managing todos, lessons/quotes, and archived data through an interactive command-line interface using the `clap` library for argument parsing and `inquire` for interactive prompts.

## Architecture

This is a Rust workspace with 5 crates organized in a clean architecture pattern:

### Crate Structure

- **cli** (`cli/`) - Entry point and CLI argument parsing
  - Binary name: `DoIt`
  - Uses `clap` for command parsing
  - Delegates commands to domain layer via `match.rs`

- **domain** (`domain/`) - Business logic layer
  - Contains modules: `todo`, `lessons`, `archive`
  - Each module has submodules: `add`, `search`, `check/delete`
  - Pure business logic with no I/O concerns

- **data** (`data/`) - Data persistence layer
  - Handles file I/O operations
  - Data storage location: `~/.do-it/data/`
  - Subdirectories: `todo/`, `lessons/`, `archive/`
  - Configuration: `~/.do-it/data/config.json`

- **ui** (`ui/`) - User interaction layer
  - Uses `inquire` for interactive prompts (text input, editor, select)
  - Uses `owo-colors` for terminal coloring
  - Handles all user input collection

- **shared** (`shared/`) - Common types and constants
  - Core data structures: `Todo`, `PartialTodo`, `Priority`, `Config`, `AppState`
  - Constants for directory names and paths

### Data Flow

1. CLI parses arguments → 
2. Dispatcher (`match.rs`) routes to domain function → 
3. Domain calls UI for missing interactive input (if needed) → 
4. Domain calls data layer to persist → 
5. Data layer writes to `~/.do-it/data/`

### Key Design Patterns

- **Workspace**: All crates share a single `Cargo.lock` at root
- **Layer dependencies**: cli → domain → ui/data → shared (unidirectional)
- **Partial inputs**: `PartialTodo` allows CLI args or interactive prompts to fill missing fields
- **Aliases**: Commands have multiple aliases (e.g., `add`/`new`/`todo`)

## Development Commands

### Building
```bash
cargo build                    # Build debug binary
cargo build --release          # Build optimized binary
```

Binary location: `target/debug/DoIt` or `target/release/DoIt`

### Running
```bash
cargo run -- [command] [args]  # Run with arguments
cargo run -- add               # Interactive todo creation
cargo run -- search            # Search todos
```

### Linting & Formatting
```bash
cargo clippy                   # Run linter
cargo clippy --all-targets --all-features  # Comprehensive lint
cargo fmt                      # Format code (uses 2-space tabs per .rustfmt.toml)
cargo fmt -- --check           # Check formatting without applying
```

### Testing
```bash
cargo test                     # Run tests (currently no tests exist)
```

## Code Conventions

- **Indentation**: 2 spaces (configured in `.rustfmt.toml`)
- **Naming**: Uses Rust naming conventions (snake_case for functions/variables, PascalCase for types)
- **Error handling**: Functions return `Result<(), String>` for operations that can fail
- **Unused warnings**: Current codebase has many stub functions with unused parameters - prefix with `_` when intentional

## Common Patterns

### Adding a New Command
1. Add command definition in `cli/src/main.rs` using `Command::new()`
2. Add match arm in `cli/src/match.rs` to extract arguments
3. Create domain function in appropriate module under `domain/src/`
4. If interactive input needed, add UI functions in `ui/src/lib.rs`
5. If persistence needed, add data functions in `data/src/`

### Interactive vs Non-Interactive
Commands support both modes:
- **Interactive**: Missing arguments trigger prompts via `ui` crate
- **Non-interactive**: All arguments provided via CLI flags

Example:
```bash
cargo run -- add                           # Interactive mode
cargo run -- add my-id -t "Title" -d "Desc" -p "high"  # Non-interactive
```

## Known Issues

- Many domain functions are stubs (empty implementations)
- Unused variable warnings throughout (functions awaiting implementation)
- `data/src/lib.rs` has copy-paste bugs in `get_lessons_data_path()` and `get_archive_data_path()` - they return `TODO_DATA_DIR_NAME` instead of their respective constants
