use std::fs;
use std::path::Path;

pub fn rename_single_file(path: &Path, prefix: Option<&String>, suffix: Option<&String>) {
    if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
        let new_name = get_new_name(name, prefix, suffix);
        let new_path = path.with_file_name(new_name);
        perform_rename(path, &new_path);
    }
}

fn get_new_name(name: &str, prefix: Option<&String>, suffix: Option<&String>) -> String {
    let (name_without_ext, ext) = name.rsplit_once('.').unwrap_or((name, ""));

    // Replace spaces with dots in the name
    let name_without_ext = name_without_ext.replace(' ', ".");

    let new_name = match (prefix, suffix) {
        (Some(p), Some(s)) => format!("{}{}{}", p, name_without_ext, s),
        (Some(p), None) => format!("{}{}", p, name_without_ext),
        (None, Some(s)) => format!("{}{}", name_without_ext, s),
        (None, None) => name_without_ext,
    };

    if ext.is_empty() {
        new_name
    } else {
        format!("{}.{}", new_name, ext)
    }
}

fn perform_rename(old_path: &Path, new_path: &Path) {
    if let Err(e) = fs::rename(old_path, new_path) {
        eprintln!("Error renaming {:?}: {}", old_path, e);
    } else {
        println!("Successfully renamed {:?} to {:?}", old_path, new_path);
    }
}
