use infer;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// Public entry point used by 'main.rs'
pub fn organize_directory(path: &Path, dry_run: bool) -> io::Result<()> {
    validate_directory(path)?;

    let files = collect_files(path)?;

    for file in files {
        process_file(path, &file, dry_run)?;
    }

    Ok(())
}

/// Ensure the provided path is a directory
fn validate_directory(path: &Path) -> io::Result<()> {
    if !path.is_dir() {
        return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Provided path is not a directory",
        ));
    }
    Ok(())
}

/// Collect all files (not directories) in the given path
fn collect_files(path: &Path) -> io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let file_path = entry.path();

        if file_path.is_file() {
            files.push(file_path);
        }
    }

    Ok(files)
}

/// Process a single file: categorize and move it
fn process_file(base_path: &Path, file_path: &PathBuf, dry_run: bool) -> io::Result<()> {
    let category = categorize_file(file_path);
    let target_dir = base_path.join(category);
    let file_name = match file_path.file_name() {
        Some(name) => name,
        None => {
            return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("Could not get file name from {:?}", file_path),
            ));
        }
    };
    let target_path = target_dir.join(file_name);

    if dry_run {
        print_dry_run(file_path, &target_path);
        return Ok(());
    }

    ensure_directory_exists(&target_dir)?;
    move_file(file_path, &target_path)?;

    Ok(())
}

/// Determine the category folder based on binary content (Magic Bytes)
/// and falls back to the file extension if content is unknown.
fn categorize_file(path: &Path) -> &'static str {
    // 1. Try to identify the file by its internal binary signature
    if let Ok(Some(kind)) = infer::get_from_path(path){
        return match kind.extension() {
            "pdf" => "PDFs",
            "docx" | "doc" => "Documents",
            "pptx" | "ppt" => "Presentations",
            "jpg" | "png" | "jpeg" => "Images", 
            "zip" | "tar" | "gz" | "rar" => "Archives",
            _ => "Other",
        };
    }
    
    // 2. Fallback: If binary check fails (e.g., file is empty), check the extension
    match path.extension().and_then(|ext| ext.to_str()) {
        Some("pdf") => "PDFs",
        Some("docx") | Some("doc") => "Documents",
        Some("pptx") | Some("ppt") => "Presentations",
        Some("jpg") | Some("png") => "Images",
        Some("zip") | Some("tar") | Some("gz") => "Archives",
        _ => "Other",
    }
}

/// Create the target directory if it does not exist
fn ensure_directory_exists(path: &Path) -> io::Result<()> {
    if !path.exists() {
        fs::create_dir_all(path)?;
    }
    Ok(())
}

/// Move the file to its new location
fn move_file(from: &Path, to: &Path) -> io::Result<()> {
    fs::rename(from, to)?;
    println!("Moved {:?} -> {:?}", from, to);
    Ok(())
}

/// Print what would happen in dry-run
fn print_dry_run(from: &Path, to: &Path) {
    println!("[DRY RUN] Would move {:?} -> {:?}", from, to);
}
