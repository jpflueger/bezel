//! `bezel` CLI. One binary that drives every toolchain so users never learn componentize-py, cargo-component or jco flags.
//! SKELETON: subcommand surface and the build pipeline shape (architecture §11). Bodies are Phase 0/1 issues (E6, E8).

use anyhow::{bail, Context, Result};
use clap::{Parser, Subcommand, ValueEnum};
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[command(name = "bezel", version, about = "Your language. Any frame.")]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Clone, Copy, ValueEnum)]
enum Sdk {
    Python,
    Rust,
    Ts,
}

#[derive(Subcommand)]
enum Cmd {
    /// Create a new app from a template
    New {
        name: String,
        #[arg(long, value_enum, default_value = "python")]
        sdk: Sdk,
    },
    /// Build, open a window, rebuild and hot-reload on change
    Dev {
        #[arg(long, default_value = ".")]
        path: PathBuf,
    },
    /// Build app.wasm (componentize → validate world → wasm-opt)
    Build {
        #[arg(long, default_value = ".")]
        path: PathBuf,
        #[arg(long)]
        release: bool,
    },
    /// Produce installers (and/or a web bundle) from app.wasm
    Pack {
        #[arg(long, default_value = ".")]
        path: PathBuf,
        #[arg(long)]
        web: bool,
        #[arg(long)]
        target: Vec<String>,
    },
    /// Diagnose toolchains, webview runtime and signing prerequisites
    Doctor,
}

fn main() -> Result<()> {
    tracing_subscriber::fmt().with_target(false).init();
    match Cli::parse().cmd {
        Cmd::New { name, sdk } => new_app(&name, sdk),
        Cmd::Dev { path } => dev(&path),
        Cmd::Build { path, release } => {
            build(&path, release)?;
            Ok(())
        }
        Cmd::Pack { path, web, target } => pack(&path, web, &target),
        Cmd::Doctor => doctor(),
    }
}

fn manifest(path: &Path) -> Result<bezel_core::policy::Manifest> {
    let s = std::fs::read_to_string(path.join("bezel.toml"))
        .context("no bezel.toml here; run `bezel new` or cd into an app")?;
    Ok(toml::from_str(&s)?)
}

fn new_app(name: &str, sdk: Sdk) -> Result<()> {
    // templates/ are embedded with include_str!; write bezel.toml, src/main.*, .github/workflows/release.yml
    let _ = (name, sdk as u8);
    bail!("TODO(E6): scaffold templates")
}

fn build(path: &Path, release: bool) -> Result<PathBuf> {
    let m = manifest(path)?;
    println!(
        "  building  {} · sdk={} · {}",
        m.app.name,
        m.app.sdk,
        if release { "release" } else { "dev" }
    );
    // 1. per-SDK toolchain: componentize-py | cargo build --target wasm32-wasip2 | jco componentize
    // 2. validate the produced component targets bezel:ui/app@0.1 (wasm-tools component targets)
    // 3. prune world imports to declared capabilities (ADR-0007)
    // 4. wasm-opt -Oz (release)
    bail!("TODO(E6): build pipeline")
}

fn dev(path: &Path) -> Result<()> {
    let _ = build(path, false)?;
    // spawn bezel-host-webview with --dev; watch src/ with `notify`; on change: build → send reload over the dev socket
    bail!("TODO(E6): dev loop")
}

fn pack(path: &Path, web: bool, targets: &[String]) -> Result<()> {
    let wasm = build(path, true)?;
    let _ = (wasm, web, targets);
    // desktop: precompile .cwasm per triple → embed into prebuilt signed host → sign → MSIX/DMG/AppImage
    // web:     jco transpile → bundle with runtime/dom → static folder + size report
    bail!("TODO(E8/E9): pack")
}

fn doctor() -> Result<()> {
    for (tool, hint) in [
        ("componentize-py", "pip install componentize-py"),
        ("cargo", "rustup"),
        ("jco", "npm i -g @bytecodealliance/jco"),
        ("wasm-tools", "cargo install wasm-tools"),
        ("wasm-opt", "binaryen"),
    ] {
        let ok = which(tool);
        println!(
            "  {}  {tool:<16} {}",
            if ok { "ok  " } else { "miss" },
            if ok { "" } else { hint }
        );
    }
    Ok(())
}
fn which(bin: &str) -> bool {
    std::env::var_os("PATH").is_some_and(|p| {
        std::env::split_paths(&p)
            .any(|d| d.join(bin).exists() || d.join(format!("{bin}.exe")).exists())
    })
}
