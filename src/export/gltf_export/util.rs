use super::*;
use glam::{Vec2, Vec3, Vec4};
use gltf::Semantic;
use gltf_json::{self as json, validation::Checked};
use json::validation::Checked::Valid;

pub struct GltfPrimitiveBuild<'a> {
    pub pos_pre_transformed: &'a [Vec3],
    pub indices: &'a [u32],
    pub uv0: Option<&'a [Vec2]>,
    pub uv1: Option<&'a [Vec2]>,
    // pub normals:  Option<&'a Vec<Vec3>>,
    // pub tangents: Option<&'a Vec<Vec3>>,
    pub colors: Option<&'a [Vec4]>,
}

pub fn write_primitive(ct: &'_ mut ExportContext, build: GltfPrimitiveBuild) -> json::mesh::Primitive {
    let mut attributes: BTreeMap<Checked<Semantic>, json::Index<json::Accessor>> = BTreeMap::new();

    while ct.cursor.position() % 4 != 0 {
        ct.cursor.write_u8(0).unwrap();
    }

    let vtx_start = ct.cursor.position();
    
    assert!(vtx_start % 4 == 0);

    let mut min = Vec3::splat(f32::INFINITY);
    let mut max = Vec3::splat(-f32::INFINITY);
    for p in build.pos_pre_transformed {
        let p = Vec3::new(-p.x, p.z, p.y);

        min = min.min(p);
        max = max.max(p);

        ct.cursor.write_f32::<ENDIAN>(-p.x).expect("write error");
        ct.cursor.write_f32::<ENDIAN>(p.z).expect("write error");
        ct.cursor.write_f32::<ENDIAN>(p.y).expect("write error");
    }

    let vtx_len = ct.cursor.position() - vtx_start;

    let vtx_view = ct.root.push(json::buffer::View {
        buffer: *ct.buffer_js,
        byte_length: USize64(vtx_len.into()),
        byte_offset: Some(vtx_start.into()),
        byte_stride: Some(json::buffer::Stride(12)),
        name: None,
        target: Some(Valid(json::buffer::Target::ArrayBuffer)),
        extensions: None,
        extras: None
    });
    
    check_buffer_view!(ct, "vtx_view");

    let pos_acc = ct.root.push(json::Accessor {
        buffer_view: Some(vtx_view),
        byte_offset: Some(USize64(0)),
        count: USize64::from(build.pos_pre_transformed.len()),
        component_type: Valid(json::accessor::GenericComponentType(json::accessor::ComponentType::F32)),
        extensions: Default::default(),
        extras: Default::default(),
        type_: Valid(json::accessor::Type::Vec3),
        min: Some(json::Value::from(min.to_array()).into()),
        max: Some(json::Value::from(max.to_array()).into()),
        name: None,
        normalized: false,
        sparse: None
    });
    check_buffer_accessor!(ct, "pos_acc");
    attributes.insert(Checked::Valid(Semantic::Positions), pos_acc);

    if let Some(uv0) = build.uv0 {
        let uv0_start = ct.cursor.position();
        for uv in uv0 {
            ct.cursor.write_f32::<ENDIAN>(uv.x).expect("write error");
            ct.cursor.write_f32::<ENDIAN>(uv.y).expect("write error");
        }
        let uv0_len = ct.cursor.position() - uv0_start;

        let uv0_view = ct.root.push(json::buffer::View {
            buffer: *ct.buffer_js,
            byte_length: USize64(uv0_len.into()),
            byte_offset: Some(uv0_start.into()),
            byte_stride: Some(json::buffer::Stride(8)),
            name: None,
            target: Some(Valid(json::buffer::Target::ArrayBuffer)),
            extensions: None,
            extras: None
        });
    
        check_buffer_view!(ct, "uv0_view");
    
        let uv0_acc = ct.root.push(json::Accessor {
            buffer_view: Some(uv0_view),
            byte_offset: Some(USize64(0)),
            count: USize64::from(build.pos_pre_transformed.len()),
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
        check_buffer_accessor!(ct, "uv0_acc");
        attributes.insert(Checked::Valid(Semantic::TexCoords(0)), uv0_acc);
    }

    if let Some(uv1) = build.uv1 {
        let uv1_start = ct.cursor.position();
        for uv in uv1 {
            ct.cursor.write_f32::<ENDIAN>(uv.x).expect("write error");
            ct.cursor.write_f32::<ENDIAN>(uv.y).expect("write error");
        }
        let uv1_len = ct.cursor.position() - uv1_start;

        let uv1_view = ct.root.push(json::buffer::View {
            buffer: *ct.buffer_js,
            byte_length: USize64(uv1_len.into()),
            byte_offset: Some(uv1_start.into()),
            byte_stride: Some(json::buffer::Stride(8)),
            name: None,
            target: Some(Valid(json::buffer::Target::ArrayBuffer)),
            extensions: None,
            extras: None
        });
    
        check_buffer_view!(ct, "uv1_view");
    
        let uv1_acc = ct.root.push(json::Accessor {
            buffer_view: Some(uv1_view),
            byte_offset: Some(USize64(0)),
            count: USize64::from(build.pos_pre_transformed.len()),
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
        check_buffer_accessor!(ct, "uv1_acc");
        attributes.insert(Checked::Valid(Semantic::TexCoords(1)), uv1_acc);
    }

    if let Some(colors) = build.colors {
        let colors_start = ct.cursor.position();
        for color in colors {
            ct.cursor.write_f32::<ENDIAN>(color.x).expect("write error");
            ct.cursor.write_f32::<ENDIAN>(color.y).expect("write error");
            ct.cursor.write_f32::<ENDIAN>(color.z).expect("write error");
            ct.cursor.write_f32::<ENDIAN>(color.w).expect("write error");
        }
        let colors_len = ct.cursor.position() - colors_start;

        let colors_view = ct.root.push(json::buffer::View {
            buffer: *ct.buffer_js,
            byte_length: USize64(colors_len.into()),
            byte_offset: Some(colors_len.into()),
            byte_stride: Some(json::buffer::Stride(16)),
            name: None,
            target: Some(Valid(json::buffer::Target::ArrayBuffer)),
            extensions: None,
            extras: None
        });
    
        check_buffer_view!(ct, "colors_view");
    
        let colors_acc = ct.root.push(json::Accessor {
            buffer_view: Some(colors_view),
            byte_offset: Some(USize64(0)),
            count: USize64::from(build.pos_pre_transformed.len()),
            component_type: Valid(json::accessor::GenericComponentType(json::accessor::ComponentType::F32)),
            extensions: Default::default(),
            extras: Default::default(),
            type_: Valid(json::accessor::Type::Vec4),
            min: None,
            max: None,
            name: None,
            normalized: false,
            sparse: None
        });
        check_buffer_accessor!(ct, "col_acc");
        attributes.insert(Checked::Valid(Semantic::Colors(0)), colors_acc);
    }
    
    let idx_start = ct.cursor.position();

    for idx in build.indices {
        ct.cursor.write_u32::<ENDIAN>(*idx).expect("write error");
    }

    let idx_len = ct.cursor.position() - idx_start;

    let idx_view = ct.root.push(json::buffer::View {
        buffer: *ct.buffer_js,
        byte_length: USize64(idx_len),
        byte_offset: Some(USize64(idx_start)),
        byte_stride: None, // index buffers are tightly packed, no stride is needed.
        name: None,
        target: Some(Valid(json::buffer::Target::ElementArrayBuffer)),
        extensions: None,
        extras: None
    });
    
    check_buffer_view!(ct, "idx_view");

    let idx_acc = ct.root.push(json::Accessor {
        buffer_view: Some(idx_view),
        byte_offset: Some(USize64(0)),
        count: USize64::from(build.indices.len()),
        component_type: Valid(json::accessor::GenericComponentType(json::accessor::ComponentType::U32)),
        extensions: Default::default(),
        extras: Default::default(),
        type_: Valid(json::accessor::Type::Scalar),
        min: None,
        max: None,
        name: None,
        normalized: false,
        sparse: None
    });
    check_buffer_accessor!(ct, "idx_acc");
        
    let primitive = json::mesh::Primitive {
        attributes,
        extensions: Default::default(),
        extras: Default::default(),
        indices: Some(idx_acc),
        material: None,
        mode: Valid(json::mesh::Mode::Triangles),
        targets: None
    };

    primitive
}