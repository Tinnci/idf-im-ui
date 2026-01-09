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

fn run_command(program: &str, args: &[&str]) -> Result<()> {
    let status = Command::new(program)
        .args(args)
        .status()?;

    if !status.success() {
        anyhow::bail!("Command failed: {} {:?}", program, args);
    }

    Ok(())
}
