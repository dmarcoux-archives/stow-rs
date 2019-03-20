extern crate clap;

use clap::{App, AppSettings, Arg};
use std::env;
use std::fs;
use std::os::unix::fs as UnixFs;
use std::path::{Path, PathBuf};

fn main() -> std::io::Result<()> {
    let matches = App::new("stow")
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .global_setting(AppSettings::ArgRequiredElseHelp)
        .global_setting(AppSettings::ColoredHelp)
        .global_setting(AppSettings::DeriveDisplayOrder)
        .arg(Arg::with_name("unstow")
             .help("Unstow the package(s)")
             .long("unstow")
             .short("U"))
        .arg(Arg::with_name("PACKAGE")
             .help("Package(s) to stow/unstow.")
             .long_help("Package(s) to stow/unstow. A package is a directory containing a collection of related files and directories.")
             .index(1)
             .multiple(true)
             .value_delimiter(" "))
        .get_matches();

    // TODO: Handle different stow dir with option
    let stow_dir = env::current_dir()?;
    // TODO: Handle panic from .unwrap() and instead match
    // TODO: Handle different target dir with option
    let target_dir = stow_dir.parent().unwrap();

    let packages: Vec<&str> = matches.values_of("PACKAGE").unwrap().collect();
    for package in packages {
        let package_dir = stow_dir.join(package);

        if matches.is_present("unstow") {
            unstow(package_dir, target_dir)?;
        } else {
            stow(package_dir, target_dir)?;
        }
    }

    Ok(())
}

// TODO: Handle errors
fn unstow(package_dir: PathBuf, target_dir: &Path) -> std::io::Result<()> {
    for entry in package_dir.read_dir()? {
        let entry = entry?.file_name();
        let source = package_dir.join(&entry);
        let destination = target_dir.join(entry);

        // If the destination is a symlink pointing to source, it is owned by stow and can be removed
        if fs::read_link(&destination)? == source {
            fs::remove_file(destination)?;
        } else {
            println!("Unstow target {} is not owned by stow", destination.display());
        }
    }

    Ok(())
}

// TODO: Handle errors
fn stow(package_dir: PathBuf, target_dir: &Path) -> std::io::Result<()> {
    for entry in package_dir.read_dir()? {
        let entry = entry?.file_name();
        let source = package_dir.join(&entry);
        let destination = target_dir.join(entry);

        UnixFs::symlink(source, destination)?;
    }

    Ok(())
}
