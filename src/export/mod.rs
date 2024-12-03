use log::*;
use std::path::PathBuf;
use std::fs::File;
use std::io::Write;
use image::ColorType;

use crate::objects::{self, TextureMetaType};
use crate::objects::TextureFormat;

use crate::objects::YetiObject;
use crate::util::dds_header::DdsHeader;

mod gltf_export; pub use gltf_export::*;

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

pub fn pick_exp_path_no_ext(obj: &YetiObject) -> Option<String> {
    let path = pick_export_folder()?;
    let path = path.to_str()?;
    Some(format!("{}\\{:#010X} {}", path, obj.get_key(), obj.get_name()))
}

pub fn exp_feu(path: String, feu: &crate::objects::Feu) {
    if let Ok(mut file) = File::create(&path) {
        info!("exporting feu to {}", &path);
        
        file.write(&[b'F', b'W', b'S']).unwrap();
        file.write(&feu.feu_data[3..]).unwrap();
    }
}

pub fn exp_texture(path_no_ext: String, tga: &objects::TextureMetadataObject, txd: &objects::TextureData) {
    if let TextureMetaType::Metadata(ref meta) = tga.meta {
        match meta.format {
            TextureFormat::Dxt1 => {
                if let Ok(mut file) = File::create(format!("{}.dds", path_no_ext)) {
                    let dds = DdsHeader::dxt1(meta.height.into(), meta.width.into());
                    dds.write_to(&mut file).unwrap();
                    file.write(&txd.texture_data[..]).unwrap();
                }
            },
            TextureFormat::Dxt5 => {
                if let Ok(mut file) = File::create(format!("{}.dds", path_no_ext)) {
                    let dds = DdsHeader::dxt5(meta.height.into(), meta.width.into());
                    dds.write_to(&mut file).unwrap();
                    file.write(&txd.texture_data[..]).unwrap();
                }
            },
            TextureFormat::Rgba8 => {
                let path = format!("{}.bmp", path_no_ext);
                image::save_buffer(path, &txd.texture_data, meta.width as u32, meta.height as u32, ColorType::Rgba8).unwrap();
            },
            TextureFormat::Bgra8 => {
                let path = format!("{}.bmp", path_no_ext);
                image::save_buffer(path, &txd.texture_data, meta.width as u32, meta.height as u32, ColorType::Rgba8).unwrap();
            },
            TextureFormat::Gray => {
                let path = format!("{}.bmp", path_no_ext);
                image::save_buffer(path, &txd.texture_data, meta.width as u32, meta.height as u32, ColorType::L8).unwrap();
            },
            _ => {
                error!("texture format export not supported! ({:?})", &meta.format);
            }
        }
    }
}

pub fn exp_msd_as_obj(path: String, msd: &crate::objects::MeshData) {
    if let Ok(mut file) = File::create(&path) {
        info!("exporting mesh to {}", &path);

        for vert in &msd.vertex_data.pos {
            write!(file, "v {} {} {}\n", vert.x, vert.y, vert.z).unwrap(); // swap y and z for coordinate correctness
        }

        for uv in &msd.vertex_data.uv0 {
            write!(file, "vt {} {}\n", uv.x, uv.y).unwrap();
        }

        for face in &msd.faces {
            write!(file, "f {} {} {}\n", face.f0 + 1, face.f1 + 1, face.f2 + 1).unwrap();
        }
    }
}