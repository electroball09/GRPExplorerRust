use crate::metadata::{ObjectType, YKey};
use crate::Bigfile;
use byteorder::WriteBytesExt;
use enum_as_inner::EnumAsInner;
use gltf_json as json;
use std::collections::{BTreeMap, HashMap};
use std::io::Cursor;
use std::mem;
use json::validation::USize64;
use std::borrow::Cow;
use byte_unit::*;
use crate::ui::*;

use byteorder::LittleEndian as ENDIAN;

mod exp_mesh; use exp_mesh::*;
mod exp_world; use exp_world::*;
mod exp_gao; use exp_gao::*;
mod exp_texture; use exp_texture::*;
mod exp_mat; use exp_mat::*;
mod exp_col; use exp_col::*;
mod exp_way; use exp_way::*;
mod gltf_export_window; pub use gltf_export_window::*;

#[derive(Debug, strum_macros::Display, strum::EnumIter, EnumAsInner, PartialEq, PartialOrd, Clone, Copy)]
pub enum WayExportStrategy {
    None,
    Triangulate,
    Extrude
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct GltfExportOptions {
    pub directional_light_intensity_multiplier  : f32,
    pub invert_directional_lights               : bool,
    pub spot_light_intentisy_multiplier         : f32,
    pub spot_light_range_multiplier             : f32,
    pub invert_spot_lights                      : bool,
    pub point_light_intensity_multiplier        : f32,
    pub point_light_range_multiplier            : f32,
    pub skybox_emissive_multiplier              : f32,

    pub export_collision                        : bool,

    pub export_empty_gaos                       : bool,

    pub way_export_strategy                     : WayExportStrategy,
}

impl Default for GltfExportOptions {
    fn default() -> Self {
        Self {
            directional_light_intensity_multiplier: 1.0,
            spot_light_intentisy_multiplier: 1.0,
            point_light_intensity_multiplier: 1.0,
            skybox_emissive_multiplier: 1.0,
            spot_light_range_multiplier: 1.0,
            point_light_range_multiplier: 1.0,
            invert_directional_lights: false,
            invert_spot_lights: false,
            export_collision: true,
            export_empty_gaos: false,
            way_export_strategy: WayExportStrategy::None,
        }
    }
}

impl GltfExportOptions {
    pub fn blender() -> Self {
        Self {
            directional_light_intensity_multiplier: 10000.0,
            spot_light_intentisy_multiplier: 8000.0,
            point_light_intensity_multiplier: 8000.0,
            skybox_emissive_multiplier: 1.5,
            spot_light_range_multiplier: 1000.0,
            point_light_range_multiplier: 1000.0,
            invert_directional_lights: true,
            invert_spot_lights: true,
            export_collision: false,
            ..Default::default()
        }
    }

    pub fn ue4() -> Self {
        Self {
            directional_light_intensity_multiplier: 4.0,
            spot_light_intentisy_multiplier: 3.5,
            point_light_intensity_multiplier: 3.5,
            invert_directional_lights: false,
            invert_spot_lights: false,
            export_collision: true,
            way_export_strategy: WayExportStrategy::Extrude,
            ..Default::default()
        }
    }

    pub fn ue5() -> Self {
        Self {
            directional_light_intensity_multiplier: 1.0,
            spot_light_intentisy_multiplier: 1.5,
            point_light_intensity_multiplier: 1.5,
            invert_directional_lights: false,
            invert_spot_lights: false,
            export_collision: true,
            way_export_strategy: WayExportStrategy::Extrude,
            ..Default::default()
        }
    }
}

struct ExportContext<'a> {
    pub key: YKey,
    pub bf: &'a Bigfile,
    pub cursor: &'a mut Cursor<&'a mut Vec<u8>>,
    pub root: &'a mut json::Root,
    pub buffer_js: &'a json::Index<json::Buffer>,
    pub index_cache: HashMap<YKey, Vec<u32>>,
    pub options: GltfExportOptions,
    pub export_subworlds: bool,
}

macro_rules! gltf_export_init {
    ($ct:expr) => {
        if $ct.index_cache.contains_key(&$ct.key) {
            //log::info!("hit cached key {:#010X}", $ct.key);
            let mut vec = Vec::new();
            for idx in $ct.index_cache.get(&$ct.key).unwrap() {
                vec.push(json::Index::new(*idx));
            }
            return vec;
        }
    }
}
pub(self) use gltf_export_init;
macro_rules! insert_cache {
    ($ct:expr, $key:expr, $index:expr) => {
        let value = $index.value() as u32;
        if $ct.index_cache.contains_key($key) {
            $ct.index_cache.get_mut($key).unwrap().push(value);
        } else {
            $ct.index_cache.insert(*$key, vec![value]);
        }
    }
}
pub(self) use insert_cache;
macro_rules! ct_with_key {
    ($ct:expr, $key:expr, $code:block) => {
        let old_key = $ct.key;
        $ct.key = $key;
        $code;
        $ct.key = old_key;
    }
}
pub(self) use ct_with_key;

fn align_to_multiple_of_four(n: usize) -> usize {
    (n + 3) & !3
}

fn to_padded_byte_vector<T>(vec: Vec<T>) -> Vec<u8> {
    let byte_length = vec.len() * mem::size_of::<T>();
    let byte_capacity = vec.capacity() * mem::size_of::<T>();
    let alloc = vec.into_boxed_slice();
    let ptr = Box::<[T]>::into_raw(alloc) as *mut u8;
    let mut new_vec = unsafe { Vec::from_raw_parts(ptr, byte_length, byte_capacity) };
    while new_vec.len() % 4 != 0 {
        new_vec.push(0); // pad to multiple of four bytes
    }
    new_vec
}

pub fn gltf_export(key: YKey, bf: &Bigfile, options: GltfExportOptions) {
    let file_name = format!("{}.glb", bf.file_table[&key].get_name());

    let path = match rfd::FileDialog::new().add_filter("glTF2.0", &[".glb"]).set_file_name(&file_name).save_file() {
        Some(path) => { path },
        None => return
    };

    log::info!("begin glTF export to file {}", &path.display());
    log::debug!("options: {:?}", &options);

    let mut root = json::Root {
        extensions_used: vec!["KHR_lights_punctual".into()],
        extensions: Some(json::extensions::Root {
            khr_lights_punctual: Some(json::extensions::root::KhrLightsPunctual::default())
        }),
        ..Default::default()
    };

    let mut buf = Vec::new();
    let mut cursor = Cursor::new(&mut buf);
    let mut nodes = Vec::new();

    let mut buffer_idx = root.push(json::Buffer {
        byte_length: USize64::default(),
        name: None,
        uri: None,
        extensions: Default::default(),
        extras: Default::default()
    });

    let mut ct = ExportContext {
        key,
        bf: &*bf,
        cursor: &mut cursor,
        root: &mut root,
        buffer_js: &mut buffer_idx,
        index_cache: HashMap::new(),
        options,
        export_subworlds: true,
    };

    match bf.file_table[&key].object_type {
        ObjectType::msh => {
            let meshes = {
                gltf_msh(&mut ct)
            };

            for mesh in meshes {
                nodes.push(ct.root.push(json::Node {
                    mesh: Some(mesh),
                    ..Default::default()
                }));
            }
        },
        ObjectType::got => {
            nodes = gltf_got(&mut ct);
        },
        ObjectType::gao => {
            nodes = gltf_gao(&mut ct, true);
        },
        ObjectType::wor => {
            nodes = gltf_wor(&mut ct);
        },
        ObjectType::col => {
            nodes = gltf_col(&mut ct);
        }
        _ => { }
    };

    assert!(ct.cursor.position() <= u32::MAX as u64);
    ct.root.buffers[0].byte_length = USize64(ct.cursor.position());

    log::info!("exported {} nodes", ct.root.nodes.len());
    log::info!("buffer size: {:#}", Byte::from_u64(ct.cursor.position()));

    ct.root.push(json::Scene {
        extensions: Default::default(),
        extras: Default::default(),
        name: None,
        nodes,
    });

    let json_string = json::serialize::to_string(ct.root).unwrap();
    let json_offset = align_to_multiple_of_four(json_string.len());
    let glb = gltf::binary::Glb {
        header: gltf::binary::Header {
            magic: *b"glTF",
            version: 2,
            length: (json_offset + ct.cursor.position() as usize) as u32
        },
        bin: Some(Cow::Owned(to_padded_byte_vector(buf))),
        json: Cow::Owned(json_string.into_bytes())
    };
    let writer = std::fs::File::create(path).unwrap();
    glb.to_writer(writer).unwrap();

    log::info!("glTF export finished!");
}