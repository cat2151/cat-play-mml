use anyhow::{Context, Result};
use std::process::Command;

use crate::app::VerbosityConfig;

/// Spawns a server process in detached mode
pub fn spawn_server_process(verbosity: &VerbosityConfig) -> Result<()> {
    let exe_path = std::env::current_exe().context("Failed to get current executable path")?;

    verbosity.println("Starting server process...");

    // Spawn the server as a detached child process
    #[cfg(target_os = "windows")]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NEW_PROCESS_GROUP: u32 = 0x00000200;
        const DETACHED_PROCESS: u32 = 0x00000008;

        Command::new(exe_path)
            .arg("--server")
            .creation_flags(CREATE_NEW_PROCESS_GROUP | DETACHED_PROCESS)
            .spawn()
            .context("Failed to spawn server process")?;
    }

    #[cfg(not(target_os = "windows"))]
    {
        Command::new(exe_path)
            .arg("--server")
            .spawn()
            .context("Failed to spawn server process")?;
    }

    verbosity.println("Server process spawned successfully");
    Ok(())
}
