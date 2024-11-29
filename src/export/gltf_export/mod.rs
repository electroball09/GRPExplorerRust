use crate::metadata::ObjectType;
use crate::Bigfile;
use byteorder::WriteBytesExt;
use gltf_json as json;
use std::collections::{BTreeMap, HashMap};
use std::io::Cursor;
use std::mem;
use json::validation::USize64;
use std::borrow::Cow;
use byte_unit::*;

use byteorder::LittleEndian as ENDIAN;

mod exp_mesh; use exp_mesh::*;
mod exp_world; use exp_world::*;
mod exp_gao; use exp_gao::*;
mod exp_texture; use exp_texture::*;

pub struct ExportContext<'a> {
    pub key: u32,
    pub bf: &'a Bigfile,
    pub cursor: &'a mut Cursor<&'a mut Vec<u8>>,
    pub root: &'a mut json::Root,
    pub buffer_js: &'a json::Index<json::Buffer>,
    pub index_cache: HashMap<u32, Vec<u32>>
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
pub(crate) use gltf_export_init;
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
pub(crate) use insert_cache;
macro_rules! ct_with_key {
    ($ct:expr, $key:expr, $code:block) => {
        let old_key = $ct.key;
        $ct.key = $key;
        $code;
        $ct.key = old_key;
    }
}
pub(crate) use ct_with_key;

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

pub fn export(key: u32, bf: &Bigfile) {
    let mut root = json::Root::default();

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
            nodes = gltf_gao(&mut ct);
        },
        ObjectType::wor => {
            nodes = gltf_wor(&mut ct);
        }
        _ => { }
    };

    assert!(ct.cursor.position() <= u32::MAX as u64);
    ct.root.buffers[0].byte_length = USize64(ct.cursor.position());

    log::info!("exported {} nodes", nodes.len());
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
    let writer = std::fs::File::create("tool_output\\test.glb").unwrap();
    glb.to_writer(writer).unwrap();
}

// pub fn export_mesh_to_gltf(_msh: &MeshMetadata, msd: &MeshData) {
//     let mut root = json::Root::default();

//     let node = root.push(json::Node {
//         mesh: Some(mesh),
//         ..Default::default()
//     });

//     root.push(json::Scene {
//         extensions: Default::default(),
//         extras: Default::default(),
//         name: None,
//         nodes: vec![node]
//     });

//     let json_string = json::serialize::to_string(&root).unwrap();
//     let json_offset = align_to_multiple_of_four(json_string.len());
//     let glb = gltf::binary::Glb {
//         header: gltf::binary::Header {
//             magic: *b"glTF",
//             version: 2,
//             length: (json_offset + vbuf_len + fbuf_len) as u32
//         },
//         bin: Some(Cow::Owned(to_padded_byte_vector(buf))),
//         json: Cow::Owned(json_string.into_bytes())
//     };
//     let writer = std::fs::File::create("tool_output\\test.glb").unwrap();
//     glb.to_writer(writer).unwrap();
// }