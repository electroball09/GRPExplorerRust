use crate::metadata::ObjectType;
use crate::Bigfile;
use byteorder::WriteBytesExt;
use gltf_json as json;
use std::collections::BTreeMap;
use std::io::Cursor;
use std::mem;
use json::validation::Checked::Valid;
use json::validation::USize64;
use std::borrow::Cow;

use byteorder::LittleEndian as ENDIAN;

mod exp_mesh; use exp_mesh::*;

pub struct ExportContext<'a> {
    pub key: u32,
    pub bf: &'a Bigfile,
    pub cursor: &'a mut Cursor<&'a mut Vec<u8>>,
    pub root: &'a mut json::Root,
    pub buffer_js: &'a json::Index<json::Buffer>,
}

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
        buffer_js: &mut buffer_idx
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
        }
        _ => { }
    };

    ct.root.buffers[0].byte_length = USize64(ct.cursor.position());

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