use crate::objects::{MeshData, MeshMetadata};
use byteorder::WriteBytesExt;
use gltf_json as json;
use std::collections::BTreeMap;
use std::mem;
use json::validation::Checked::Valid;
use json::validation::USize64;
use std::borrow::Cow;

use byteorder::LittleEndian as ENDIAN;

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

pub fn export_mesh_to_gltf(_msh: &MeshMetadata, msd: &MeshData) {
    let mut root = json::Root::default();
    
    let vtx_len: usize = 12 + 8;
    let face_len: usize = 2;
    let vbuf_len: usize = (msd.num_vertices as usize) * vtx_len;
    let fbuf_len: usize = (msd.num_indices as usize) * face_len;

    let mut buf: Vec<u8> = Vec::with_capacity(vbuf_len + fbuf_len);
    {
        let wr = &mut buf;
        for i in 0..(msd.num_vertices as usize) {
            let vtx = &msd.vertex_data.pos[i];
            let uv = &msd.vertex_data.uv0[i];
            wr.write_f32::<ENDIAN>(vtx.x).unwrap();
            wr.write_f32::<ENDIAN>(vtx.z).unwrap(); //flip y and z coords
            wr.write_f32::<ENDIAN>(vtx.y).unwrap();
            wr.write_f32::<ENDIAN>(uv.x).unwrap();
            wr.write_f32::<ENDIAN>(uv.y).unwrap();
        }
        for i in 0..(msd.num_indices as usize) / 3 {
            let face = &msd.faces[i];
            wr.write_u16::<ENDIAN>(face.f0).unwrap();
            wr.write_u16::<ENDIAN>(face.f1).unwrap();
            wr.write_u16::<ENDIAN>(face.f2).unwrap();
        }
    }

    let js_buffer = root.push(json::Buffer {
        byte_length: USize64::from(vbuf_len + fbuf_len),
        extensions: Default::default(),
        extras: Default::default(),
        name: None,
        uri: None,
    });

    let vtx_view = root.push(json::buffer::View {
        buffer: js_buffer,
        byte_length: USize64::from(vbuf_len),
        byte_offset: None,
        byte_stride: Some(json::buffer::Stride(vtx_len)),
        extensions: Default::default(),
        extras: Default::default(),
        name: None,
        target: Some(Valid(json::buffer::Target::ArrayBuffer))
    });

    let face_view = root.push(json::buffer::View {
        buffer: js_buffer,
        byte_length: USize64::from(fbuf_len),
        byte_offset: Some(USize64::from(vbuf_len)),
        byte_stride: Some(json::buffer::Stride(face_len)),
        extensions: Default::default(),
        extras: Default::default(),
        name: None,
        target: Some(Valid(json::buffer::Target::ArrayBuffer))
    });

    let (min, max) = msd.bounding_box();
    let min: &[f32] = &min.to_array();
    let max: &[f32] = &max.to_array();
    let pos_acc = root.push(json::Accessor {
        buffer_view: Some(vtx_view),
        byte_offset: Some(USize64(0)),
        count: USize64::from(msd.num_vertices as usize),
        component_type: Valid(json::accessor::GenericComponentType(json::accessor::ComponentType::F32)),
        extensions: Default::default(),
        extras: Default::default(),
        type_: Valid(json::accessor::Type::Vec3),
        min: Some(json::Value::from(min)),
        max: Some(json::Value::from(max)),
        name: None,
        normalized: false,
        sparse: None
    });

    let uv_acc = root.push(json::Accessor {
        buffer_view: Some(vtx_view),
        byte_offset: Some(USize64(12)),
        count: USize64::from(msd.num_vertices as usize),
        component_type: Valid(json::accessor::GenericComponentType(json::accessor::ComponentType::F32)),
        extensions: Default::default(),
        extras: Default::default(),
        type_: Valid(json::accessor::Type::Vec2),
        min: None,
        max: None,
        name: None,
        normalized: false,
        sparse: None
    });

    let face_acc = root.push(json::Accessor {
        buffer_view: Some(face_view),
        byte_offset: Some(USize64(0)),
        count: USize64::from(msd.num_indices as usize),
        component_type: Valid(json::accessor::GenericComponentType(json::accessor::ComponentType::U16)),
        extensions: Default::default(),
        extras: Default::default(),
        type_: Valid(json::accessor::Type::Scalar),
        min: None,
        max: None,
        name: None,
        normalized: false,
        sparse: None
    });
    
    let primitive = json::mesh::Primitive {
        attributes: {
            let mut map = BTreeMap::new();
            map.insert(Valid(json::mesh::Semantic::Positions), pos_acc);
            map.insert(Valid(json::mesh::Semantic::TexCoords(0)), uv_acc);
            map
        },
        extensions: Default::default(),
        extras: Default::default(),
        indices: Some(face_acc),
        material: None,
        mode: Valid(json::mesh::Mode::Triangles),
        targets: None
    };

    let mesh = root.push(json::Mesh {
        extensions: Default::default(),
        extras: Default::default(),
        name: None,
        primitives: vec![primitive],
        weights: None
    });

    let node = root.push(json::Node {
        mesh: Some(mesh),
        ..Default::default()
    });

    root.push(json::Scene {
        extensions: Default::default(),
        extras: Default::default(),
        name: None,
        nodes: vec![node]
    });

    let json_string = json::serialize::to_string(&root).unwrap();
    let json_offset = align_to_multiple_of_four(json_string.len());
    let glb = gltf::binary::Glb {
        header: gltf::binary::Header {
            magic: *b"glTF",
            version: 2,
            length: (json_offset + vbuf_len + fbuf_len) as u32
        },
        bin: Some(Cow::Owned(to_padded_byte_vector(buf))),
        json: Cow::Owned(json_string.into_bytes())
    };
    let writer = std::fs::File::create("tool_output\\test.glb").unwrap();
    glb.to_writer(writer).unwrap();
}