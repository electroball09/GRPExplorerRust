use crate::objects::{TextureData, TextureFormat, TextureMetadata};

pub fn decompress_texture(meta: &TextureMetadata, txd: &TextureData) -> Vec<u8> {
    match meta.format {
        TextureFormat::Bgra8 | TextureFormat::Rgba8 | TextureFormat::Gray => {
            txd.texture_data.clone()
        },
        TextureFormat::Dxt1 => {
            let f = texpresso::Format::Bc1;
            let w: usize = meta.width.into();
            let h: usize = meta.height.into();
            let mut buf: Vec<u8> = vec![0; w * h * 4];
            f.decompress(&txd.texture_data, w, h, &mut buf);
            //let buf: Vec<u8> = buf.chunks_exact(4).map(|x| [x[0], x[1], x[2], x[3]]).collect::<Vec<[u8; 4]>>().iter().flat_map(|x| *x).collect();
            buf
        },
        TextureFormat::Dxt5 => {
            let f = texpresso::Format::Bc3;
            let w: usize = meta.width.into();
            let h: usize = meta.height.into();
            let mut buf: Vec<u8> = vec![0; w * h * 4];
            f.decompress(&txd.texture_data, w, h, &mut buf);
            //let buf: Vec<u8> = buf.chunks_exact(4).map(|x| [x[0], x[1], x[2], x[3]]).collect::<Vec<[u8; 4]>>().iter().flat_map(|x| *x).collect();
            buf
        },
        _ => { Vec::new() }
    }
}