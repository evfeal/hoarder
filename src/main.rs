use std::fs;
use std::path::{Path, PathBuf};
use clap::{Arg, Command};
use glob::glob;

fn main() {
    let matches = Command::new("Hoarder")
        .version("1.0")
        .author("Evan Alvarez")
        .about("The all-in-one renaming tool for people with too many files")
        .arg(Arg::new("paths")
            .value_name("PATHS")
            .help("Path to the file(s) or directories to be renamed")
            .required(true)
            .num_args(1..))
        .arg(Arg::new("prefix")
            .short('p')
            .long("prefix")
            .value_name("PREFIX")
            .help("Prefix to add to filenames")
            .conflicts_with("suffix"))
        .arg(Arg::new("suffix")
            .short('s')
            .long("suffix")
            .value_name("SUFFIX")
            .help("Suffix to add to filenames"))
        .get_matches();

    let paths: Vec<&String> = matches.get_many("paths").unwrap().collect();
    let prefix = matches.get_one::<String>("prefix");
    let suffix = matches.get_one::<String>("suffix");

    for path in paths {
        rename_files(path, prefix, suffix);
    }
}

fn rename_files(pattern: &str, prefix: Option<&String>, suffix: Option<&String>) {
    for entry in glob(pattern).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                if path.is_file() {
                    rename_single_file(&path, prefix, suffix);
                } else if path.is_dir() {
                    rename_in_directory(&path, prefix, suffix);
                }
            }
            Err(e) => println!("Error: {:?}", e),
        }
    }
}

fn rename_single_file(path: &Path, prefix: Option<&String>, suffix: Option<&String>) {
    if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
        let new_name = get_new_name(name, prefix, suffix);
        let new_path = path.with_file_name(new_name);
        perform_rename(path, &new_path);
    }
}

fn rename_in_directory(dir: &Path, prefix: Option<&String>, suffix: Option<&String>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                rename_single_file(&path, prefix, suffix);
            }
        }
    } else {
        eprintln!("Error reading directory: {:?}", dir);
    }
}

fn get_new_name(name: &str, prefix: Option<&String>, suffix: Option<&String>) -> String {
    match (prefix, suffix) {
        (Some(p), None) => format!("{}{}", p, name),
        (None, Some(s)) => {
            let (name_without_ext, ext) = name.rsplit_once('.').unwrap_or((name, ""));
            if ext.is_empty() {
                format!("{}{}", name, s)
            } else {
                format!("{}{}.{}", name_without_ext, s, ext)
            }
        },
        _ => name.to_string(),
    }
}

fn perform_rename(old_path: &Path, new_path: &Path) {
    if let Err(e) = fs::rename(old_path, new_path) {
        eprintln!("Error renaming {:?}: {}", old_path, e);
    } else {
        println!("Renamed {:?} to {:?}", old_path, new_path);
    }
}
