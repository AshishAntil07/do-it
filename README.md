# DoIt

A fast, minimalist CLI todo and task management application written in Rust.

## Features

- **Todo Management**: Create, search, complete, and delete todos with priorities
- **Lessons/Quotes**: Store and search memorable quotes and lessons learned
- **Archive**: Keep track of cold storage data
- **Interactive Mode**: Prompts for missing information when not provided via CLI
- **Non-Interactive Mode**: Full control via command-line arguments
- **Priority System**: Organize todos by priority levels
- **Local Storage**: All data stored locally in `~/.do-it/data/`

## Installation

### From Release (Recommended)

Download the latest binary for your platform from the [Releases](https://github.com/AshishAntil07/do-it/releases) page:

- **Linux (x86_64)**: `DoIt-linux-x86_64`
- **macOS (Intel)**: `DoIt-macos-x86_64`
- **macOS (Apple Silicon)**: `DoIt-macos-aarch64`
- **Windows (x86_64)**: `DoIt-windows-x86_64.exe`

Make the binary executable (Linux/macOS):
```bash
chmod +x DoIt-*
sudo mv DoIt-* /usr/local/bin/doit
```

### From Source

Requires Rust 1.70 or later:

```bash
git clone git@github.com:AshishAntil07/do-it.git
cd do-it
cargo build --release
sudo cp target/release/DoIt /usr/local/bin/doit
```

## Usage

### Todos

#### Add a Todo

**Interactive mode** (prompts for missing fields):
```bash
doit add
```

**Non-interactive mode** (all arguments provided):
```bash
doit add my-task-id -t "Implement login feature" -d "Add OAuth2 authentication" -p 4
```

Aliases: `new`, `todo`

#### Search Todos

**Basic search**:
```bash
doit search "login"
```

**Filter by ID**:
```bash
doit search -i my-task-id
```

**Filter by status and priority**:
```bash
doit search -c -p 4                 # Completed high-priority todos
doit search -b                      # Incomplete todos only
doit search -d                      # Todos with descriptions
```

Alias: `find`

#### Complete a Todo

```bash
doit check task-1,task-2,task-3
```

Aliases: `complete`, `done`

#### Delete a Todo

```bash
doit delete task-1,task-2
```

Alias: `remove`

### Lessons/Quotes

#### Add a Lesson

**Interactive**:
```bash
doit lessons add
```

**Non-interactive**:
```bash
doit quotes add lesson-1 -l "Always write tests first"
```

Aliases: `quotes`, `lessons`

#### Search Lessons

```bash
doit lessons search "tests"
doit quotes find -i lesson-1
```

#### Delete Lessons

```bash
doit lessons delete lesson-1,lesson-2
```

Alias: `remove`

### Archive

#### Add to Archive

**Interactive**:
```bash
doit archive add
```

**Non-interactive**:
```bash
doit archive add archive-1 -d "Old project notes"
```

#### Search Archive

```bash
doit archive search "project"
doit archive find -i archive-1
```

#### Delete from Archive

```bash
doit archive delete archive-1,archive-2
```

### Global Options

```bash
doit --debug [command]    # Enable debug mode
doit --version            # Show version
doit --help               # Show help
```

## Data Storage

All data is stored locally in:
```
~/.do-it/data/
├── todo/           # Todo items
├── lessons/        # Lessons and quotes
├── archive/        # Archived data
└── config.json     # Application configuration
```

## Development

### Project Structure

This is a Rust workspace with 5 crates:

- **cli** - Command-line interface and argument parsing
- **domain** - Business logic for todos, lessons, and archive
- **data** - Data persistence layer (file I/O)
- **ui** - Interactive prompts and user input
- **shared** - Common types and constants

### Building

```bash
cargo build              # Debug build
cargo build --release    # Optimized build
```

### Running

```bash
cargo run -- add         # Run with arguments
cargo run -- --help      # Show help
```

### Linting & Formatting

```bash
cargo clippy             # Run linter
cargo fmt                # Format code (2-space indentation)
cargo fmt -- --check     # Check formatting without applying
```

## License

[MIT](https://github.com/AshishAntil07/do-it/blob/main/LICENSE)

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
