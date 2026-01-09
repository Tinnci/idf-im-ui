# Installation & Distribution Strategy

## Overview

The ESP-IDF Installation Manager (EIM) is distributed through multiple channels to reach different user audiences across Windows, macOS, and Linux platforms. This document outlines the current distribution strategy and proposes xtask enhancements.

## Current Distribution Channels

### 1. Platform-Specific Installers (Built by CI/CD)

**Linux:**
- Debian (`.deb`) packages via GitHub Releases
- RPM (`.rpm`) packages via GitHub Releases
- Configured in: `src-tauri/tauri.conf.json` → `bundle.linux`

**macOS:**
- `.app` bundle (code-signed)
- DMG (disk image) for distribution
- Configured in: `src-tauri/tauri.conf.json` → `bundle.macOS`
- Code signing with developer identity: `QWXF6GB4AV`

**Windows:**
- `.msi` (Windows Installer)
- `.exe` (standalone executable)
- Configured in: `src-tauri/tauri.conf.json` → `bundle`

### 2. Package Managers

**Homebrew (macOS/Linux):**
- Workflow: `.github/workflows/update-homebrew.yml`
- Automatic updates on release
- Reaches macOS and Linux developers

**Windows Package Managers:**
- Scoop
- Chocolatey
- Workflow: `.github/workflows/update-windows-packages.yml`

**Linux Repositories:**
- Debian/Ubuntu: auto-updated
- RHEL/CentOS: auto-updated
- Workflow: `.github/workflows/update-linux-repos.yml`

### 3. GitHub Releases

**Direct Download:**
- All platform binaries published on GitHub Releases
- Includes:
  - Linux x64 (.deb, .rpm)
  - Linux ARM64 (.deb, .rpm)
  - macOS x64 (.dmg, .app.tar.gz)
  - macOS ARM64 (.dmg, .app.tar.gz)
  - Windows x64 (.msi, .exe)

### 4. Build Artifacts

**CI/CD Pipeline:**
- `.github/workflows/build.yaml` - Main build workflow
- Runs on: schedule, PR, release, manual dispatch
- Platforms: Linux x64, Linux ARM64, macOS x64, macOS ARM64, Windows x64
- Creates offline installer archives

**Offline Installers:**
- Workflow: `.github/workflows/build_offline_installer_archives.yaml`
- Bundles ESP-IDF dependencies
- Large files hosted separately

## User Installation Paths

### Path 1: End-User GUI Installation (No Development Tools Needed)

```
User → Download EIM from:
       ├─ GitHub Releases
       ├─ Homebrew (macOS/Linux)
       ├─ Scoop/Chocolatey (Windows)
       └─ Native package managers (Linux)
       
       → Install with platform-specific installer
       → Open GUI application
       → Configure ESP-IDF
```

### Path 2: Developer Installation (From Source)

```
Developer → Clone repository
            → Install Rust, Node.js
            → Run: cargo xtask build
            → Output: Compiled binary in target/
```

### Path 3: Docker/Container Installation

```
CI/CD System → Use pre-built container
              → Contains EIM + dependencies
              → No build required
```

## xtask Enhancement Opportunities

### Current xtask Capabilities

```bash
cargo xtask dev              # Development only
cargo xtask build            # Builds single platform binary
cargo xtask check            # Verifies compilation
cargo xtask all              # Full quality pipeline
```

### Proposed xtask Enhancements for Installation

#### 1. `cargo xtask install`

**Install to system location:**
```rust
cargo xtask install              # Install to ~/.local/bin (Linux) or /usr/local/bin (macOS)
cargo xtask install --system     # System-wide installation (requires sudo)
cargo xtask uninstall            # Remove installed binary
```

**Implementation:**
```rust
fn install_app(system: bool) -> Result<()> {
    // 1. Build release binary
    run_command("cargo", &["tauri", "build"])?;
    
    // 2. Detect target installation directory
    let install_path = if system {
        "/usr/local/bin"  // macOS/Linux
        // "C:\\Program Files" (Windows)
    } else {
        "~/.local/bin"    // Linux
        // "~/AppData/Local/bin" (Windows)
    };
    
    // 3. Copy binary to installation directory
    // 4. Create symlink or add to PATH
    // 5. Create desktop shortcut (Linux/macOS)
    
    Ok(())
}
```

**Use Case:** Developers who want EIM on their system path

#### 2. `cargo xtask package`

**Create distribution packages:**
```bash
cargo xtask package             # Platform-specific package (deb/rpm/msi)
cargo xtask package --format=deb
cargo xtask package --format=rpm
cargo xtask package --format=msi
```

**Benefits:**
- Unified packaging workflow
- Version consistency
- Pre-release testing
- Local distribution testing

#### 3. `cargo xtask dist`

**Build all distribution formats:**
```bash
cargo xtask dist                # All packages: deb, rpm, msi, dmg
cargo xtask dist --target=aarch64  # For specific architecture
```

**Output:**
```
dist/
├── eim-0.6.0-x86_64.deb
├── eim-0.6.0-aarch64.deb
├── eim-0.6.0-x86_64.rpm
├── eim-0.6.0-aarch64.rpm
├── eim-0.6.0-x64.msi
└── eim-0.6.0.dmg
```

#### 4. `cargo xtask sign`

**Code signing automation:**
```bash
cargo xtask sign --identity="Developer ID Application"  # macOS
cargo xtask sign --key=path/to/key.pem                 # Windows
```

**Features:**
- macOS: Sign with developer identity
- Windows: Sign with code signing certificate
- Verification and notarization support

#### 5. `cargo xtask release`

**Full release workflow:**
```bash
cargo xtask release --version=0.7.0
  ├─ Update versions in all files
  ├─ Build all packages
  ├─ Sign packages
  ├─ Create GitHub release
  └─ Publish to package managers
```

## Distribution Workflow Improvements

### Current CI/CD Flow

```
Code Commit → Multiple specialized workflows:
           ├─ build.yaml (builds binaries)
           ├─ build_offline_installer_archives.yaml
           ├─ update-homebrew.yml
           ├─ update-linux-repos.yml
           └─ update-windows-packages.yml
           
Result: Binaries on GitHub Releases
```

### Proposed xtask-Integrated Flow

```
Code Commit → Single workflow:
           └─ Run: cargo xtask dist --all
              ├─ Calls xtask package (builds all formats)
              ├─ Calls xtask sign (signs all binaries)
              ├─ Calls xtask release (publishes)
              └─ Reports success/failure
              
Result: Same result, cleaner orchestration
```

## Installation Scripts Using xtask

### For End Users (Simple)

**macOS/Linux:**
```bash
# Option 1: Pre-built binary from GitHub
curl -L https://github.com/espressif/idf-im-ui/releases/download/v0.6.0/eim-0.6.0-x64.tar.gz | tar xz
sudo mv eim /usr/local/bin/

# Option 2: From source (requires Rust)
git clone https://github.com/espressif/idf-im-ui.git
cd idf-im-ui
cargo xtask install
```

**Windows:**
```powershell
# Option 1: MSI installer (GUI)
# Download and run: eim-0.6.0-x64.msi

# Option 2: From source (requires Rust)
git clone https://github.com/espressif/idf-im-ui.git
cd idf-im-ui
cargo xtask install
```

### For CI/CD Integration

**GitHub Actions:**
```yaml
- name: Install EIM
  run: cargo xtask install

- name: Configure ESP-IDF
  run: eim --version
```

## xtask Implementation Roadmap

### Phase 1: Core Installation (Now)
- [x] Existing: `cargo xtask dev` and `cargo xtask build`
- [ ] New: `cargo xtask install`

### Phase 2: Packaging (Short-term)
- [ ] `cargo xtask package` - Build OS-specific packages
- [ ] `cargo xtask dist` - Build all distribution formats
- [ ] Version management helpers

### Phase 3: Distribution (Medium-term)
- [ ] `cargo xtask sign` - Code signing
- [ ] `cargo xtask release` - Full release workflow
- [ ] Package manager integrations

### Phase 4: Advanced (Long-term)
- [ ] Container image building (Docker)
- [ ] Cloud distribution integration
- [ ] Analytics and telemetry

## Current Installation Reality Check

### What Espressif Currently Does

1. **Builds** applications using CI/CD (multiple workflows)
2. **Signs** code for macOS/Windows
3. **Publishes** to:
   - GitHub Releases
   - Package managers (Homebrew, Scoop, apt, yum)
   - Official repositories
4. **Maintains** separate update workflows for each platform

### How xtask Can Improve This

```
BEFORE: Manual coordination of 5+ specialized workflows
AFTER:  Single `cargo xtask release` command
```

Benefits:
- ✅ Consistency across all platforms
- ✅ Faster release cycles
- ✅ Easier for contributors to manage
- ✅ Type-safe build logic
- ✅ Documented in one place
- ✅ CI/CD integration simpler

## Recommended xtask Extension for Installation

### Implement `cargo xtask install` First

This is the quickest win and most useful for developers:

```rust
#[derive(Subcommand)]
enum Commands {
    // ... existing commands ...
    
    /// Install EIM to system
    #[command(name = "install")]
    Install {
        /// System-wide installation (requires sudo)
        #[arg(long)]
        system: bool,
        
        /// Custom installation path
        #[arg(long)]
        path: Option<String>,
    },
    
    /// Uninstall EIM from system
    #[command(name = "uninstall")]
    Uninstall {
        #[arg(long)]
        system: bool,
    },
}

fn install_app(system: bool, custom_path: Option<String>) -> Result<()> {
    println!("🚀 Installing EIM...");
    
    // Build release binary
    println!("Building release binary...");
    run_command("cargo", &["tauri", "build", "--release"])?;
    
    // Determine installation path
    let install_path = if let Some(path) = custom_path {
        path
    } else if system {
        determine_system_path()?
    } else {
        determine_user_path()?
    };
    
    // Create directory if needed
    fs::create_dir_all(&install_path)?;
    
    // Copy binary
    let binary_src = get_binary_location();
    let binary_dst = format!("{}/eim", install_path);
    fs::copy(&binary_src, &binary_dst)?;
    
    // Make executable
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&binary_dst, fs::Permissions::from_mode(0o755))?;
    }
    
    // Verify installation
    run_command(&binary_dst, &["--version"])?;
    
    println!("✅ Successfully installed to: {}", binary_dst);
    println!("💡 Tip: Add '{}' to your PATH", install_path);
    
    Ok(())
}
```

## Next Steps

### For Espressif Team

1. **Review** this installation strategy
2. **Decide** which xtask commands to implement
3. **Prioritize**: 
   - `install` (most useful)
   - `package` (improves CI/CD)
   - `sign` (improves security)
   - `release` (final goal)

### For Contributors

Use this guide to:
- Install EIM from source: `cargo xtask build`
- Install to system: `cargo xtask install` (when implemented)
- Build packages: `cargo xtask package` (when implemented)

## References

- [Tauri Bundler Documentation](https://tauri.app/develop/building/)
- [Building Installers](https://tauri.app/develop/building/distribution-bundles/)
- [Current Build Workflow](../.github/workflows/build.yaml)
- [Offline Installer Build](../.github/workflows/build_offline_installer_archives.yaml)
