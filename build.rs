#[warn(clippy::all)]
use platforms::*;

/// Generate `cargo` production build output
pub fn gen_build() {
    let output = std::process::Command::new("git")
        .args(&["rev-parse", "--short", "HEAD"])
        .output();

    let commit = match output {
        Ok(out) if out.status.success() => {
            let sha = String::from_utf8_lossy(&out.stdout).trim().to_owned();
            std::borrow::Cow::from(sha)
        }
        Ok(out) => {
            println!(
                "cargo:warning=Git command failed with status: {}",
                out.status
            );
            std::borrow::Cow::from("unknown")
        }
        Err(e) => {
            println!("cargo:warning=Failed to execute git command: {}", e);
            std::borrow::Cow::from("unknown")
        }
    };

    println!(
        "cargo:rustc-env=RUST_KB_CENTER_VERSION={}",
        get_version(&commit)
    )
}

fn get_plattform() -> String {
    let env_dash = if TARGET_ENV.is_some() { "-" } else { "" };

    format!(
        "{}-{}{}{}",
        TARGET_ARCH.as_str(),
        TARGET_OS.as_str(),
        env_dash,
        TARGET_ENV.map(|t| t.as_str()).unwrap_or("")
    )
}

fn get_version(commit: &str) -> String {
    let commit_dash = if commit.is_empty() { "" } else { "-" };

    format!(
        "{}{}{}-{}",
        std::env::var("CARGO_PKG_VERSION").unwrap_or_default(),
        commit_dash,
        commit,
        get_plattform()
    )
}

fn main() {
    gen_build();
}
