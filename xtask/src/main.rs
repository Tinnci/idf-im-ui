use anyhow::Result;
use clap::{Parser, Subcommand};
use std::env;
use std::process::Command;

#[derive(Parser)]
#[command(name = "xtask")]
#[command(about = "Build automation for idf-im-ui", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Build the Tauri application
    #[command(name = "build")]
    Build {
        /// Build target (x86_64, aarch64, etc.)
        #[arg(long)]
        target: Option<String>,
    },

    /// Run Tauri in development mode
    #[command(name = "dev")]
    Dev,

    /// Check code without building
    #[command(name = "check")]
    Check,

    /// Format code
    #[command(name = "fmt")]
    Fmt,

    /// Run clippy linter
    #[command(name = "lint")]
    Lint,

    /// Run tests
    #[command(name = "test")]
    Test,

    /// Clean build artifacts
    #[command(name = "clean")]
    Clean,

    /// Install the application
    #[command(name = "install")]
    Install,

    /// Setup system dependencies
    #[command(name = "setup")]
    Setup,

    /// Full build pipeline (check → fmt → lint → build)
    #[command(name = "all")]
    All {
        /// Build target (optional)
        #[arg(long)]
        target: Option<String>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Build { target } => build_app(target)?,
        Commands::Dev => dev_app()?,
        Commands::Check => check_code()?,
        Commands::Fmt => format_code()?,
        Commands::Lint => lint_code()?,
        Commands::Test => test_code()?,
        Commands::Clean => clean_build()?,
        Commands::Install => install_app()?,
        Commands::Setup => setup_system()?,
        Commands::All { target } => {
            println!("Running full build pipeline...\n");
            check_code()?;
            format_code()?;
            lint_code()?;
            build_app(target)?;
            println!("\n✅ Full pipeline completed successfully!");
        }
    }

    Ok(())
}

fn build_app(target: Option<String>) -> Result<()> {
    println!("🔨 Building Tauri application...");
    
    // Set pre-build environment if needed
    env::set_var("TAURI_SKIP_WEBVIEW_DOWNLOAD", "false");
    
    // Build with cargo directly to exclude offline feature
    // (avoids lzma-rust2 compilation which has compatibility issues)
    let mut args = vec!["tauri", "build"];
    let target_arg;
    
    if let Some(t) = target {
        target_arg = format!("--target={}", t);
        args.push(&target_arg);
    }

    run_command("cargo", &args)?;
    println!("✅ Build completed!");
    Ok(())
}

fn dev_app() -> Result<()> {
    println!("🚀 Starting development server...");
    
    // Set development environment variables
    env::set_var("TAURI_SKIP_WEBVIEW_DOWNLOAD", "false");
    
    run_command("cargo", &["tauri", "dev"])?;
    Ok(())
}

fn check_code() -> Result<()> {
    println!("📋 Checking code...");
    
    // Check Rust code
    run_command("cargo", &["check", "--all"])?;
    
    println!("✅ Check passed!");
    Ok(())
}

fn format_code() -> Result<()> {
    println!("📐 Formatting code...");
    
    // Format Rust code
    run_command("cargo", &["fmt", "--all"])?;
    
    println!("✅ Code formatted!");
    Ok(())
}

fn lint_code() -> Result<()> {
    println!("🔍 Running linter...");
    
    // Run clippy for Rust linting
    run_command("cargo", &["clippy", "--all", "--", "-D", "warnings"])?;
    
    println!("✅ Linting passed!");
    Ok(())
}

fn test_code() -> Result<()> {
    println!("🧪 Running tests...");
    
    // Run all tests
    run_command("cargo", &["test", "--all"])?;
    
    println!("✅ Tests passed!");
    Ok(())
}

fn clean_build() -> Result<()> {
    println!("🧹 Cleaning build artifacts...");
    
    run_command("cargo", &["clean"])?;
    
    println!("✅ Clean completed!");
    Ok(())
}

fn install_app() -> Result<()> {
    println!("📦 Installing application...");
    
    run_command("cargo", &["tauri", "build"])?;
    
    println!("✅ Installation completed!");
    Ok(())
}

fn setup_system() -> Result<()> {
    println!("🔧 Setting up system dependencies...\n");
    
    let os = std::env::consts::OS;
    match os {
        "linux" => setup_linux()?,
        "macos" => setup_macos()?,
        "windows" => setup_windows()?,
        _ => {
            eprintln!("❌ Unsupported OS: {}", os);
            anyhow::bail!("Setup not available for this OS");
        }
    }
    
    println!("\n✅ System setup complete!");
    println!("💡 You can now run: cargo xtask dev");
    Ok(())
}

fn setup_linux() -> Result<()> {
    println!("📦 Detecting Linux distribution...");
    
    let os_release = std::fs::read_to_string("/etc/os-release")
        .unwrap_or_default();
    
    if os_release.contains("ubuntu") || os_release.contains("debian") {
        setup_debian_ubuntu()?;
    } else if os_release.contains("fedora") || os_release.contains("rhel") || os_release.contains("centos") {
        setup_fedora_rhel()?;
    } else if os_release.contains("arch") || os_release.contains("cachyos") || os_release.contains("manjaro") {
        setup_arch()?;
    } else {
        println!("⚠️  Unknown Linux distribution. Please install the following packages:");
        println!("   - libwebkit2gtk-4.1-dev (or webkit2gtk3-devel)");
        println!("   - libjavascriptcoregtk-4.1-dev (or libjavascriptcoregtk4.1-devel)");
        println!("   - libglib2.0-dev (or glib2-devel)");
        println!("   - build-essential (or base-devel)");
    }
    
    Ok(())
}

fn setup_debian_ubuntu() -> Result<()> {
    println!("📦 Installing dependencies for Debian/Ubuntu...");
    println!("   (This will require sudo)");
    
    let deps = vec![
        "libwebkit2gtk-4.1-dev",
        "libjavascriptcoregtk-4.1-dev",
        "libglib2.0-dev",
        "build-essential",
        "curl",
        "wget",
        "libssl-dev",
        "pkg-config",
    ];
    
    println!("   Running: sudo apt-get update");
    run_command("sudo", &["apt-get", "update"])?;
    
    println!("   Running: sudo apt-get install -y {:?}", deps.join(" "));
    let mut args = vec!["apt-get", "install", "-y"];
    args.extend(&deps);
    run_command("sudo", &args)?;
    
    Ok(())
}

fn setup_fedora_rhel() -> Result<()> {
    println!("📦 Installing dependencies for Fedora/RHEL/CentOS...");
    println!("   (This will require sudo)");
    
    let deps = vec![
        "webkit2gtk3-devel",
        "libjavascriptcoregtk4.1-devel",
        "glib2-devel",
        "gcc",
        "gcc-c++",
        "make",
        "curl",
        "wget",
        "openssl-devel",
        "pkg-config",
    ];
    
    println!("   Running: sudo dnf install -y {:?}", deps.join(" "));
    let mut args = vec!["dnf", "install", "-y"];
    args.extend(&deps);
    run_command("sudo", &args)?;
    
    Ok(())
}

fn setup_arch() -> Result<()> {
    println!("📦 Installing dependencies for Arch/CachyOS/Manjaro...");
    println!("   (This will require sudo)");
    
    let deps = vec![
        "webkit2gtk-4.1",
        "glib2",
        "base-devel",
        "curl",
        "wget",
        "openssl",
        "pkg-config",
    ];
    
    println!("   Running: sudo pacman -S --noconfirm {:?}", deps.join(" "));
    let mut args = vec!["pacman", "-S", "--noconfirm"];
    args.extend(&deps);
    
    // Ignore errors as many packages may already be installed
    let status = Command::new("sudo")
        .args(&args)
        .status()?;
    
    if status.success() {
        println!("   ✅ Arch dependencies installed");
    } else {
        println!("   ⚠️  Some packages were already installed or not found (this is OK)");
    }
    
    Ok(())
}

fn setup_macos() -> Result<()> {
    println!("📦 Installing dependencies for macOS...");
    
    // Check if Homebrew is installed
    let homebrew_check = Command::new("which")
        .arg("brew")
        .status();
    
    if homebrew_check.is_err() || !homebrew_check?.success() {
        println!("⚠️  Homebrew not found. Installing Homebrew first...");
        let install_script = "/bin/bash -c \"$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)\"";
        run_command("/bin/bash", &["-c", install_script])?;
    }
    
    let deps = vec![
        "webkit2gtk",
        "libsoup",
        "cairo",
        "pango",
        "glib",
    ];
    
    println!("   Running: brew install {:?}", deps.join(" "));
    let mut args = vec!["install"];
    args.extend(&deps);
    run_command("brew", &args)?;
    
    Ok(())
}

fn setup_windows() -> Result<()> {
    println!("❌ Automatic setup not available for Windows");
    println!("\n📖 Please follow the official Tauri setup guide:");
    println!("   https://tauri.app/v1/guides/getting-started/prerequisites");
    println!("\n💡 Required tools:");
    println!("   - Microsoft Visual Studio C++ build tools");
    println!("   - WebView2 Runtime");
    println!("   - Rust toolchain");
    
    Ok(())
}

fn run_command(program: &str, args: &[&str]) -> Result<()> {
    let status = Command::new(program)
        .args(args)
        .status()?;

    if !status.success() {
        anyhow::bail!("Command failed: {} {:?}", program, args);
    }

    Ok(())
}
