mod image;
mod simple;
mod organize;

use clap::{Arg, Command};

fn main() {
    let matches = Command::new("Hoarder")
        .version("1.0")
        .author("Evan Alvarez")
        .about("The all-in-one renaming tool for people with too many files")
        .arg(
            Arg::new("paths")
                .value_name("PATH")
                .help("Path to the file(s) or directories to be renamed")
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

    let paths: Vec<&String> = matches.get_many("paths").unwrap().collect();
    let prefix = matches.get_one::<String>("prefix");
    let suffix = matches.get_one::<String>("suffix");
    let use_directory_structure = matches.get_flag("directory");

    for path in paths {
        if use_directory_structure {
            organize::organize_files(path);
        } else {
            simple::rename_files(path, prefix, suffix);
        }
    }
}
