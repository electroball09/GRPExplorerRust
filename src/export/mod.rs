use log::*;
use std::path::PathBuf;
use std::fs::File;
use std::io::Write;
use image::ColorType;

use crate::objects;
use crate::objects::texture::TextureFormat;

use crate::objects::YetiObject;
use crate::util::dds_header::DdsHeader;

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

pub fn exp_feu(path: String, feu: &crate::objects::feu::Feu) {
    if let Ok(mut file) = File::create(&path) {
        info!("exporting feu to {}", &path);
        
        file.write(&[b'F', b'W', b'S']).unwrap();
        file.write(&feu.feu_data[3..]).unwrap();
    }
}

pub fn exp_texture(path_no_ext: String, tga: &objects::texture::TextureMetadata, txd: &objects::texture::TextureData) {
    match tga.format {
        TextureFormat::Dxt1 => {
            if let Ok(mut file) = File::create(format!("{}.dds", path_no_ext)) {
                let dds = DdsHeader::dxt1(tga.height.into(), tga.width.into());
                dds.write_to(&mut file).unwrap();
                file.write(&txd.texture_data[..]).unwrap();
            }
        },
        TextureFormat::Dxt5 => {
            if let Ok(mut file) = File::create(format!("{}.dds", path_no_ext)) {
                let dds = DdsHeader::dxt5(tga.height.into(), tga.width.into());
                dds.write_to(&mut file).unwrap();
                file.write(&txd.texture_data[..]).unwrap();
            }
        },
        TextureFormat::Rgba32 => {
            let path = format!("{}.bmp", path_no_ext);
            image::save_buffer(path, &txd.texture_data, tga.width as u32, tga.height as u32, ColorType::Rgba8).unwrap();
        },
        TextureFormat::Bgra32 => {
            let path = format!("{}.bmp", path_no_ext);
            image::save_buffer(path, &txd.texture_data, tga.width as u32, tga.height as u32, ColorType::Rgba8).unwrap();
        },
        TextureFormat::Gray => {
            let path = format!("{}.bmp", path_no_ext);
            image::save_buffer(path, &txd.texture_data, tga.width as u32, tga.height as u32, ColorType::L8).unwrap();
        },
        _ => {
            error!("texture format export not supported! ({:?})", &tga.format);
        }
    }
}

pub fn exp_msd_as_obj(path: String, msd: &crate::objects::meshes::MeshData) {
    if let Ok(mut file) = File::create(&path) {
        info!("exporting mesh to {}", &path);

        for vert in &msd.vertex_data.pos {
            write!(file, "v {} {} {}\n", vert.x, vert.y, vert.z).unwrap(); // swap y and z for coordinate correctness
        }

        if let Some(uv0) = &msd.vertex_data.uv0 {
            for uv in uv0 {
                write!(file, "vt {} {}\n", uv.x, uv.y).unwrap();
            }
        }

        for face in &msd.faces {
            write!(file, "f {} {} {}\n", face.f0 + 1, face.f1 + 1, face.f2 + 1).unwrap();
        }
    }
}