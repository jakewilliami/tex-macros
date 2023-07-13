// We want to copy the article to our local texmf directory
// https://www.ugr.es/~ftorralbo/blog/programming/local-texmf/
//
// First, we need to find it!  We can do this manually, or calling
// out to the kpsewhich tool

use std::fs;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use home;

// Get texmf from kpsewhich
// kpsewhich -var-value TEXMFHOME
pub fn texmf() -> Option<PathBuf> {
    let mut cmd = Command::new("kpsewhich");
    cmd.arg("-var-value");
    cmd.arg("TEXMFHOME");

    let output = cmd
        .stdout(Stdio::piped())
        .output()
        .expect("Failed to execute `kpsewhich`");

    if output.status.success() {
        let mut texmf_home = String::from_utf8_lossy(&output.stdout)
            .into_owned();

        // Strip trailing new line
        // https://stackoverflow.com/a/55041833/12069968
        if texmf_home.ends_with('\n') {
            texmf_home.pop();
            if texmf_home.ends_with('\r') {
                texmf_home.pop();
            }
        }

        Some(PathBuf::from(texmf_home))
    } else {
        None
    }
}

pub fn texmf_exists() -> bool {
    texmf().is_some()
}

// Get texmf manually
#[cfg(target_os = "macos")]
pub fn texmf_manual() -> PathBuf {
    home::home_dir().expect("Cannot get home directory")
        .join("Library").join("texmf")
}
#[cfg(any(target_os = "linux", target_os = "windows"))]
pub fn texmf_manual() -> PathBuf {
    home::home_dir().expect("Cannot get home directory")
        .join("texmf")
}

pub fn texmf_exists_manual() -> bool {
    texmf_manual().as_path().exists()
}

pub fn texmf_local_resources() -> PathBuf {
    let local_dir = texmf().expect("Cannot get texmf dir")
        .join("tex").join("latex").join("local");

    // Make directory if it doesn't exist
    if !local_dir.as_path().exists() {
        fs::create_dir_all(&local_dir).unwrap();
    }

    local_dir
}
