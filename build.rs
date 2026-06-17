use std::env;
use std::fs;
use std::path::Path;
use fs_extra::dir::{self, *};

fn main() {
    env::set_var("RUST_BACKTRACE", "full");

    let directories_to_copy = ["cfg\\"];

    for dir in directories_to_copy {
        copy_files_to_output(dir).expect("failed to copy config files");
    }

    copy_shader_sources_to_output(); 

    println!("cargo::rerun-if-changed=build.rs");
}

fn copy_files_to_output(dir: &str) -> Result<(), String> {
    let build_profile = env::var("PROFILE").unwrap();
    let source_dir = Path::new(dir);
    let output_path = Path::new(&env::var("CARGO_TARGET_DIR").unwrap_or_else(|_| "target\\".into())).join(&build_profile);

    dir::create_all(&output_path, false).map_err(|_| "fail to create output dir")?;

    let target_path = output_path.join(&source_dir);
    dbg!(&target_path);
    dir::remove(&target_path).map_err(|_| "Fail to remove output dir")?;
    
    let mut opt = CopyOptions::new();
    opt.overwrite = true;
    opt.copy_inside = true;

    dir::copy(source_dir, &output_path, &opt).map_err(|_| "failed to copy folder!")?;

    println!("cargo:rerun-if-changed={}", source_dir.display());
    for file in fs::read_dir(&source_dir).map_err(|_| "failed to read files in dir")? {
        let file = file.unwrap();
        dbg!(file.path());
        println!("cargo:rerun-if-changed={}", file.path().display());
    }

    Ok(())
}

fn copy_shader_sources_to_output() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let source_dir = Path::new("src\\ggl\\shader_source\\");
    let output_path = Path::new(&out_dir).join("shadergen.rs");  

    let sources = fs::read_dir(&source_dir).unwrap().filter_map(|x| {
        match x {
            Ok(entry) => {
                if entry.metadata().unwrap().is_file() {
                    return Some((entry.file_name().into_string().unwrap(), fs::read_to_string(entry.path()).unwrap()[..].to_string()));
                }
                None
            }
            Err(_) => None
        }
    }).map(|x| {
        format!("map.insert(\"{}\".to_string(), \"{}\".to_string());", x.0, x.1)
    }).collect::<Vec<String>>().join("\r\n");

    let sources = format!("{{{}}}", sources);

    fs::write(output_path, &sources).unwrap();
    
    println!("cargo::rerun-if-changed={}", source_dir.to_str().unwrap());
    for entry in fs::read_dir(&source_dir).unwrap() {
        println!("cargo::rerun-if-changed={}{}", &source_dir.to_str().unwrap(), &entry.unwrap().file_name().to_str().unwrap())
    }
}