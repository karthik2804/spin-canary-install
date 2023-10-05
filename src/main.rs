mod links;

use anyhow::{bail, Result};
use clap::Parser;
use links::create_url_map;
use std::process::Command as Cmd;

/// Returns build information, similar to: 0.1.0 (2be4034 2022-03-31).
const VERSION: &str = concat!(
    env!("CARGO_PKG_VERSION"),
    " (",
    env!("VERGEN_GIT_SHA"),
    " ",
    env!("VERGEN_GIT_COMMIT_DATE"),
    ")"
);

#[derive(Debug, Parser)]
#[clap(name = "spin canary-install", version = VERSION)]
struct App {
    app_name: String,
}
fn main() -> Result<(), anyhow::Error> {
    let args = App::parse();
    let url_map = create_url_map();
    match url_map.get(args.app_name.as_str()) {
        Some(url) => {
            let mut cmd = Cmd::new(spin_bin_path()?);
            cmd.arg("plugins").arg("install").arg("-u").arg(url);
            let mut child = cmd.spawn()?;
            // Wait for the child process to finish
            child.wait()?;
        }
        None => bail!(
            "unknown plugin name. Known plugins are {}",
            url_map
                .keys()
                .map(|key| key.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        ),
    }
    Ok(())
}

fn spin_bin_path() -> Result<String> {
    Ok(std::env::var("SPIN_BIN_PATH")?)
}
