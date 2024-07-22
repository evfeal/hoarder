mod image;
mod organize;
mod simple;

use anyhow::Result;
use clap::{Arg, Command};
use glob::glob;
use rayon::prelude::*;
use std::path::{Path, PathBuf};

fn main() -> Result<()> {
    let matches = Command::new("Hoarder")
        .version("1.0")
        .author("Evan Alvarez")
        .about("The all-in-one renaming tool for people with too many files")
        .arg(
            Arg::new("paths")
                .value_name("PATH")
                .help("Path(s) to the file(s) or directories to be renamed (supports wildcards)")
                .required(true)
                .num_args(1..),
        )
        .arg(
            Arg::new("prefix")
                .short('p')
                .long("prefix")
                .value_name("PREFIX")
                .help("Prefix to add to filenames")
                .conflicts_with("suffix"),
        )
        .arg(
            Arg::new("suffix")
                .short('s')
                .long("suffix")
                .value_name("SUFFIX")
                .help("Suffix to add to filenames"),
        )
        .arg(
            Arg::new("directory")
                .short('d')
                .long("directory")
                .help("Organize files into a directory based structure")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    let path_patterns: Vec<&String> = matches.get_many("paths").unwrap().collect();
    let prefix = matches.get_one::<String>("prefix");
    let suffix = matches.get_one::<String>("suffix");
    let use_directory_structure = matches.get_flag("directory");

    path_patterns.par_iter().for_each(|pattern| {
        if let Err(e) = process_pattern(pattern, use_directory_structure, prefix, suffix) {
            eprintln!("Error processing pattern {}: {:?}", pattern, e);
        }
    });

    Ok(())
}

fn process_pattern(
    pattern: &str,
    use_directory_structure: bool,
    prefix: Option<&String>,
    suffix: Option<&String>,
) -> Result<()> {
    glob(pattern)?
        .par_bridge()
        .try_for_each(|entry| -> Result<()> {
            let path = entry?;
            if use_directory_structure {
                organize::organize_files(&path)?;
            } else {
                process_path(&path, prefix, suffix)?;
            }
            Ok(())
        })
}

fn process_path(path: &PathBuf, prefix: Option<&String>, suffix: Option<&String>) -> Result<()> {
    if path.is_dir() {
        walkdir::WalkDir::new(path)
            .into_iter()
            .par_bridge()
            .try_for_each(|entry| -> Result<()> {
                let entry = entry?;
                if entry.file_type().is_file() {
                    process_file(&entry.path(), prefix, suffix)?;
                }
                Ok(())
            })
    } else {
        process_file(path, prefix, suffix)
    }
}

fn process_file(path: &Path, prefix: Option<&String>, suffix: Option<&String>) -> Result<()> {
    if image::is_image(path) {
        if let Some(new_path) = image::process_image(path) {
            println!("Processed image: {:?} to {:?}", path, new_path);
        }
    } else {
        simple::rename_single_file(path, prefix, suffix)?;
    }
    Ok(())
}
