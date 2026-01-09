# Tauri + xtask Integration Guide

This document explains how Tauri and xtask are integrated in the idf-im-ui project.

## Overview

The idf-im-ui project uses a **Cargo workspace** pattern with **xtask** build automation. This provides:

- **Type-safe build logic** in Rust instead of shell scripts
- **Centralized automation** for consistent developer experience
- **CI/CD friendly** with a single entry point for all build tasks
- **Cross-platform support** without complex shell scripting

## Project Structure

```
idf-im-ui/
├── Cargo.toml              # Workspace manifest
├── .cargo/config.toml      # Cargo configuration (includes xtask alias)
├── xtask/                  # Build automation crate
│   ├── Cargo.toml
│   ├── src/main.rs
│   └── README.md
└── src-tauri/              # Tauri application
    ├── Cargo.toml
    ├── Cargo.lock
    └── ...
```

## How It Works

### 1. Workspace Setup

The root `Cargo.toml` defines the workspace:

```toml
[workspace]
members = ["src-tauri", "xtask"]
resolver = "2"
```

This tells Cargo that `src-tauri` (the Tauri app) and `xtask` (build automation) are separate crates that share dependencies and build configuration.

### 2. Cargo Alias

The `.cargo/config.toml` file sets up a convenient alias:

```toml
[alias]
xtask = "run --package xtask --"
```

This allows you to invoke xtask with:
```bash
cargo xtask <command>
```

Instead of the longer:
```bash
cargo run --package xtask -- <command>
```

### 3. xtask Implementation

The `xtask/src/main.rs` file implements all build commands:

- **dev**: Starts Tauri development server
- **build**: Creates production build
- **check**: Verifies compilation
- **fmt**: Formats code
- **lint**: Runs clippy
- **test**: Runs tests
- **all**: Full pipeline

Each command:
1. Validates preconditions
2. Sets environment variables as needed
3. Runs the appropriate Cargo command
4. Reports success/failure

## Usage Patterns

### Daily Development

```bash
# Start development with hot-reload
cargo xtask dev

# In another terminal, run checks
cargo xtask check
cargo xtask fmt
cargo xtask lint
```

### Pre-Commit Checklist

```bash
# Run full quality pipeline
cargo xtask all
```

### Building for Release

```bash
# Standard build
cargo xtask build

# Cross-compile for specific target
cargo xtask build --target=aarch64
```

### CI/CD Integration

In GitHub Actions or other CI systems:

```bash
# Full verification pipeline
cargo xtask all --target=x86_64
cargo xtask all --target=aarch64
```

## Adding New Commands

To add a new xtask command:

### 1. Update `Commands` enum

```rust
#[derive(Subcommand)]
enum Commands {
    // ... existing commands ...
    
    /// My new command
    #[command(name = "mycommand")]
    MyCommand {
        #[arg(long)]
        option: Option<String>,
    },
}
```

### 2. Add command handler in match statement

```rust
fn main() -> Result<()> {
    // ... existing code ...
    Commands::MyCommand { option } => my_command(option)?,
}
```

### 3. Implement the command function

```rust
fn my_command(option: Option<String>) -> Result<()> {
    println!("🚀 Running my command...");
    
    if let Some(opt) = option {
        println!("With option: {}", opt);
    }
    
    run_command("cargo", &["mycommand"])?;
    println!("✅ Done!");
    Ok(())
}
```

### 4. Document in xtask/README.md

## Tauri Integration

The xtask crate doesn't directly depend on tauri-cli. Instead, it:

1. Invokes the `cargo tauri` command via subprocess
2. Sets environment variables for Tauri's behavior
3. Handles platform-specific logic

Example from `build_app()`:

```rust
fn build_app(target: Option<String>) -> Result<()> {
    println!("🔨 Building Tauri application...");
    
    // Set environment for Tauri
    env::set_var("TAURI_SKIP_WEBVIEW_DOWNLOAD", "false");
    
    let mut args = vec!["tauri", "build"];
    if let Some(t) = target {
        args.push(&format!("--target={}", t));
    }

    run_command("cargo", &args)?;
    println!("✅ Build completed!");
    Ok(())
}
```

## Workspace Benefits

### Shared Dependencies

Both `src-tauri` and `xtask` can share Rust dependencies, avoiding duplication and ensuring version consistency.

### Unified Build

```bash
cargo check --all    # Checks both crates
cargo fmt --all      # Formats both crates
cargo test --all     # Tests both crates
```

### Clear Separation of Concerns

- `src-tauri/`: The actual application
- `xtask/`: Build and development infrastructure

This keeps the main app crate lean and focused.

## Troubleshooting

### Cargo xtask alias not working

If `cargo xtask` doesn't work, your Cargo version may not support aliases in local config files. Use the full form instead:

```bash
cargo run --package xtask -- <command>
```

### Missing dependencies

If a xtask command fails due to missing tools:

1. Check the error message
2. Install the required system library (e.g., WebKit dev files for Tauri)
3. Re-run the command

### Build failures in CI

1. Ensure all system dependencies are installed
2. Run `cargo xtask all` locally first
3. Check GitHub Actions logs for specific failures

## Best Practices

✅ **Do:**
- Run `cargo xtask all` before committing
- Use `cargo xtask dev` for day-to-day development
- Keep xtask commands focused and single-purpose
- Document new commands in this guide
- Use xtask for all build automation

❌ **Don't:**
- Mix shell scripts with xtask
- Add heavy dependencies to xtask
- Create xtask commands that modify source files
- Use xtask for deployment (consider separate tool)

## Related Documentation

- [xtask/README.md](../xtask/README.md) - Detailed xtask command reference
- [AGENTS.md](../AGENTS.md) - AI agent guidance for this project
- [Tauri Documentation](https://tauri.app) - Official Tauri guide
- [Cargo Workspaces](https://doc.rust-lang.org/cargo/reference/workspaces.html) - Cargo workspace documentation
