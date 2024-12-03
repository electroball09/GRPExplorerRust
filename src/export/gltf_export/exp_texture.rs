use crate::objects::*;
use crate::util::texture_util;
use super::*;
use gltf_json as json;
use json::validation::Checked::Valid;
use json::validation::USize64;

pub fn gltf_tga<'a>(ct: &'a mut ExportContext) -> Vec<json::Index<json::Texture>> {
    gltf_export_init!(ct);

    let (meta, txd_key) = match &ct.bf.object_table[&ct.key].archetype {
        ObjectArchetype::TextureMetadata(tga) => {
            let first_ref = ct.bf.object_table[&ct.key].references[0];
            match tga.meta {
                TextureMetaType::Metadata(meta) => (meta, first_ref),
                TextureMetaType::Passthrough => {
                    match &ct.bf.object_table[&first_ref].archetype {
                        ObjectArchetype::TextureMetadata(tga) => {
                            let first_ref = ct.bf.object_table[&first_ref].references[0];
                            match tga.meta {
                                TextureMetaType::Metadata(meta) => (meta, first_ref),
                                _ => panic!("wtf we have a weird texture here! {:#010X} {:#010X}", ct.key, first_ref)
                            }
                        },
                        _ => panic!("wrong object type!")
                    }
                }
                TextureMetaType::None => panic!("wtf we have a weird texture here! {:#010X}", ct.key)
            }
        },
        _ => panic!("wrong object type!") 
    };
    
    let txd = match &ct.bf.object_table[&txd_key].archetype {
        ObjectArchetype::TextureData(txd) => txd,
        _ => panic!("not a texture data file??")
    };

    let name = Some(format!("{:#010X} {}", ct.key, ct.bf.file_table[&txd_key].get_name_ext().to_string()));
    
    let mut data = texture_util::decompress_texture(&meta, txd);

    if data.len() != meta.width as usize * meta.height as usize * 4 as usize {
        log::warn!("skipping texture {:#010X} due to bad data size! {} != {}", ct.key, data.len(), meta.width as usize * meta.height as usize * 4 as usize);
        return Vec::new();
    }

    if meta.is_normal_map() {
        data = data.chunks_exact(4).flat_map(|ch| [ch[1], ch[3], ch[2], 255]).collect();
    }

    let tex_start = ct.cursor.position();
    let color_type = match meta.format {
        TextureFormat::Bgra8 => image::ExtendedColorType::Bgra8,
        _ => image::ExtendedColorType::Rgba8
    };
    image::write_buffer_with_format(ct.cursor, &data, meta.width as u32, meta.height as u32, color_type, image::ImageFormat::Png).unwrap();
    let tex_end = ct.cursor.position();

    let tex_view = ct.root.push(json::buffer::View {
        buffer: *ct.buffer_js,
        byte_length: USize64(tex_end - tex_start),
        byte_offset: Some(USize64(tex_start)),
        target: Some(Valid(json::buffer::Target::ArrayBuffer)),
        byte_stride: Some(json::buffer::Stride(16)),
        name: name.clone(), 
        extensions: Default::default(),
        extras: Default::default()
    });

    let source = ct.root.push(json::Image {
        buffer_view: Some(tex_view),
        mime_type: Some(json::image::MimeType("image/png".to_string())),
        name: name.clone(),
        uri: None,
        extensions: Default::default(),
        extras: Default::default()
    });

    let sampler = ct.root.push(json::texture::Sampler {
        mag_filter: Some(Valid(json::texture::MagFilter::Linear)),
        min_filter: Some(Valid(json::texture::MinFilter::Linear)),
        name: name.clone(),
        wrap_s: Valid(json::texture::WrappingMode::Repeat),
        wrap_t: Valid(json::texture::WrappingMode::Repeat),
        extensions: Default::default(),
        extras: Default::default()
    });

    let texture = ct.root.push(json::Texture {
        name: name.clone(),
        sampler: Some(sampler),
        source,
        extensions: Default::default(),
        extras: Default::default()
    });
    
    insert_cache!(ct, &ct.key, texture);
    
    vec![texture]
}