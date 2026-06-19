use std::fs::File;
use std::path::{Path, PathBuf};
use std::process::Command;
use zip::write::SimpleFileOptions;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Building release binary...");
    let status = Command::new("cargo")
        .args(["build", "--release"])
        .status()?;

    if !status.success() {
        return Err("Cargo build failed".into());
    }

    let base_path = Path::new("target/release");

    let files_to_bundle = ["grp_explorer_rust.exe"];
    let folders_to_bundle = ["cfg"];

    let zip_path = "target/release/grp_explorer_rust.zip";

    println!("Creating zip archive at {}...", zip_path);
    let file = File::create(zip_path)?;
    let mut zip = zip::ZipWriter::new(file);

    let options = SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated);

    let mut files_to_copy = Vec::new();
    for file in files_to_bundle {
        let path = file;
        files_to_copy.push(PathBuf::from(path));
    }
    for folder in folders_to_bundle {
        let path = base_path.join(folder);
        for entry_result in std::fs::read_dir(&path)? {
            let entry = entry_result?;
            let path = entry.path().strip_prefix(base_path)?.to_string_lossy().replace("\\", "/");
            files_to_copy.push(path.into());
        }
    }

    for file in &files_to_copy {
        println!("file to copy {}", file.display());
    }
    
    for file in &files_to_copy {
        let path = std::fs::canonicalize(base_path.join(file))?;
        println!("copying file {}", path.display());
        let mut file_stream = File::open(&path)?;
        zip.start_file(file.to_string_lossy(), options)?;
        std::io::copy(&mut file_stream, &mut zip)?;
    }
    
    zip.finish()?;

    println!("Success! Zip file ready for release.");
    Ok(())
}