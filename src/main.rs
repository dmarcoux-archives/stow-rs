use std::env;
use std::os::unix::fs;

fn main() -> std::io::Result<()> {
    symlink("README.md", "symlinked_README.md")
}

fn symlink(source: &str, destination: &str) -> std::io::Result<()> {
    let current_dir = env::current_dir()?;
    // TODO: Handle panic from .unwrap() and instead match
    let target_dir = current_dir.parent().unwrap();

    let source = current_dir.join(source);

    let destination = target_dir.join(destination);

    fs::symlink(source, destination)?;

    Ok(())
}
