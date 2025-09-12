use crate::objects::*;
use crate::util::texture_util;
use super::*;
use gltf_json as json;
use json::validation::Checked::Valid;
use json::validation::USize64;
use crate::bigfile::util::*;

#[derive(Default, PartialEq, Debug)]
pub enum TextureTransformHint {
    #[default]
    None,
    NormalMap,
    ChannelToAlpha(usize),
    ChannelToAlphaAndClear(usize, f32),
    ChannelToAlphaInvertAndClear(usize),
    ChannelModify(TextureChannelIdentifier, TextureChannelIdentifier, TextureChannelIdentifier, TextureChannelIdentifier),
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum TextureChannelIdentifier {
    Channel(usize),
    Clear(u8)
}

impl TextureChannelIdentifier {
    fn transform_value(&self, channel: u8, data: &[u8]) -> u8 {
        match self {
            Self::Channel(nchan) => data[*nchan],
            Self::Clear(value) => *value
        }
    }
}

pub fn gltf_tga<'a>(ct: &'a mut ExportContext, hint: TextureTransformHint) -> Vec<json::Index<json::Texture>> {
    gltf_export_init!(ct);

    while ct.cursor.position() % 4 != 0 {
        ct.cursor.write_u8(0).unwrap();
    }

    let key = match unwrap_tga_key(ct.key, ct.bf) {
        Some(key) => key,
        None => return Vec::new()
    };
    let meta = &ct.bf.object_table[&key].archetype.as_texture_metadata().unwrap().meta.as_metadata().unwrap();

    let txd_key = ct.bf.object_table[&key].references[0];
    let txd = &ct.bf.object_table[&txd_key].archetype.as_texture_data().unwrap();

    let name = Some(format!("{:#010X} {}", ct.key, ct.bf.file_table[&txd_key].get_name_ext().to_string()));
    
    let data = texture_util::decompress_texture(&meta, txd);

    if data.len() != meta.width as usize * meta.height as usize * 4 as usize {
        log::warn!("skipping texture {:#010X} due to bad data size! {} != {}", ct.key, data.len(), meta.width as usize * meta.height as usize * 4 as usize);
        return Vec::new();
    }

    let data = match hint {
        TextureTransformHint::None => { data },
        TextureTransformHint::NormalMap => 
            data.chunks_exact(4).flat_map(|ch| [ch[1], ch[3], ch[2], 255]).collect(),
        TextureTransformHint::ChannelToAlpha(orig_channel) => 
            data.chunks_exact(4).flat_map(|ch| [ch[0], ch[1], ch[2], ch[orig_channel]]).collect(),
        TextureTransformHint::ChannelToAlphaAndClear(orig_channel, alpha_scale) => 
            data.chunks_exact(4).flat_map(|ch| [255, 255, 255, (ch[orig_channel] as f32 * alpha_scale) as u8]).collect(),
        TextureTransformHint::ChannelToAlphaInvertAndClear(orig_channel) => 
            data.chunks_exact(4).flat_map(|ch| [255, 255, 255, 255 - ch[orig_channel]]).collect(),
        TextureTransformHint::ChannelModify(cr, cg, cb, ca) =>
            data.chunks_exact(4).flat_map(|ch| [cr.transform_value(0, ch), cg.transform_value(0, ch), cb.transform_value(0, ch), ca.transform_value(0, ch)]).collect(),
    };

    if meta.is_normal_map() != (hint == TextureTransformHint::NormalMap) {
        log::warn!("texture: {:#010X}  is_normal_map: {}  hint: {:?}", key, meta.is_normal_map(), hint);
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
        target: None, // byte_stride and target must be None for image buffers
        byte_stride: None,
        name: name.clone(), 
        extensions: Default::default(),
        extras: Default::default()
    });

    check_buffer_view!(ct, "tex_view");

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
        name: name,
        sampler: Some(sampler),
        source,
        extensions: Default::default(),
        extras: Default::default()
    });
    
    insert_cache!(ct, &ct.key, texture);
    
    vec![texture]
}