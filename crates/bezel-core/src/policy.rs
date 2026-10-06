//! Capability policy (ADR-0007). Parsed from bezel.toml by the CLI, enforced here.

use serde::Deserialize;

#[derive(Clone, Debug, Deserialize, Default)]
pub struct Manifest {
    pub app: AppInfo,
    #[serde(default)]
    pub capabilities: Capabilities,
    #[serde(default)]
    pub window: WindowInfo,
}

#[derive(Clone, Debug, Deserialize, Default)]
pub struct AppInfo {
    pub name: String,
    pub id: String,
    pub version: String,
    pub entry: String,
    pub sdk: String,
}

#[derive(Clone, Debug, Deserialize, Default)]
pub struct Capabilities {
    #[serde(default, rename = "fs.read")]
    pub fs_read: Vec<String>,
    #[serde(default, rename = "fs.write")]
    pub fs_write: Vec<String>,
    #[serde(default)]
    pub network: Vec<String>,
    #[serde(default)]
    pub clipboard: bool,
    #[serde(default)]
    pub notifications: bool,
    /// Memory limit for the app instance, in MiB. Default 512.
    #[serde(default = "default_mem")]
    pub memory_mib: u64,
}
fn default_mem() -> u64 {
    512
}

#[derive(Clone, Debug, Deserialize, Default)]
pub struct WindowInfo {
    pub title: Option<String>,
    pub initial: Option<Size>,
}
#[derive(Clone, Debug, Deserialize, Default)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Capability {
    FsRead,
    FsWrite,
    Network,
    Clipboard,
    Notifications,
}

impl Capabilities {
    /// Fail closed: anything not declared is not granted.
    pub fn granted(&self, c: Capability) -> bool {
        match c {
            Capability::FsRead => !self.fs_read.is_empty(),
            Capability::FsWrite => !self.fs_write.is_empty(),
            Capability::Network => !self.network.is_empty(),
            Capability::Clipboard => self.clipboard,
            Capability::Notifications => self.notifications,
        }
    }
    /// Host:port allow-list check for wasi:http outgoing requests.
    pub fn network_allows(&self, host: &str, port: u16) -> bool {
        self.network
            .iter()
            .any(|e| e == &format!("{host}:{port}") || (port == 443 && e == host))
    }
}
