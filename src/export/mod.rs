use std::path::PathBuf;
use std::fs::File;
use std::io::Write;

use crate::objects::YetiObject;

pub fn pick_extract_folder() -> Option<PathBuf> {
    rfd::FileDialog::new().pick_folder()
}

pub fn pick_export_folder() -> Option<PathBuf> {
    rfd::FileDialog::new().pick_folder()
}

pub fn pick_exp_path(obj: &YetiObject, ext: &str) -> Option<String> {
    let path = pick_export_folder()?;
    let path = path.to_str()?;
    Some(format!("{}\\{:#010X} {}{}", path, obj.get_key(), obj.get_name(), ext))
}

pub fn exp_feu(path: String, feu: &crate::objects::feu::Feu) {

    if let Ok(mut file) = File::create(&path) {
        println!("exporting feu to {}", &path);
        
        file.write(&[b'F', b'W', b'S']).unwrap();
        file.write(&feu.feu_data[3..]).unwrap();
    }
}

pub fn exp_texture_as_png(path: String, obj: &YetiObject, tga: &crate::objects::texture::TextureMetadata, txd: &crate::objects::texture::TextureData) {
    
}

pub fn exp_msd_as_obj(path: String, msd: &crate::objects::meshes::MeshData) {
    if let Ok(mut file) = File::create(&path) {
        println!("exporting mesh to {}", &path);

        for vert in &msd.vertices {
            write!(file, "v {} {} {}\n", vert.pos.x, vert.pos.y, vert.pos.z).unwrap(); // swap y and z for coordinate correctness
        }

        for face in &msd.faces {
            write!(file, "f {} {} {}\n", face[0] + 1, face[1] + 1, face[2] + 1).unwrap();
        }
    }
}