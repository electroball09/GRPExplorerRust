use std::env;
use std::fs;
use std::path::Path;
use build_print::info;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let source_dir = Path::new(&env::current_dir().unwrap()).join("src\\ggl\\shader_source\\");
    let output_path = Path::new(&out_dir).join("shadergen.rs");

    info!("{:?}", source_dir);

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

    println!("cargo::rerun-if-changed=build.rs");
    for entry in fs::read_dir(&source_dir).unwrap() {
        println!("cargo::rerun-if-changed={}{}", &source_dir.to_str().unwrap(), &entry.unwrap().file_name().to_str().unwrap())
    }
}