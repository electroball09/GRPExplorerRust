use super::*;
use glam::{Vec2, Vec3, Vec4};
use gltf::{Semantic};
use gltf_json::{self as json, validation::Checked};
use json::validation::Checked::Valid;

pub struct GltfPrimitiveBuild<'a> {
    pub pos_pre_transformed: Box<dyn Iterator<Item = Vec3> + 'a>, 
    pub indices: Box<dyn Iterator<Item = u32> + 'a>,
    pub uv0: Option<Box<dyn Iterator<Item = Vec2> + 'a>>,
    pub uv1: Option<Box<dyn Iterator<Item = Vec2> + 'a>>,
    pub normals: Option<Box<dyn Iterator<Item = Vec3> + 'a>>,
    pub tangents: Option<Box<dyn Iterator<Item = Vec3> + 'a>>,
    pub colors: Option<Box<dyn Iterator<Item = Vec4> + 'a>>,
    pub weights: Option<Box<dyn Iterator<Item = [(u32, f32); 4]> + 'a>>,
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
    let mut num_vertices: usize = 0;
    for p in build.pos_pre_transformed {
        let p = Vec3::new(-p.x, p.z, p.y);

        min = min.min(p);
        max = max.max(p);

        ct.cursor.write_f32::<ENDIAN>(p.x).expect("write error");
        ct.cursor.write_f32::<ENDIAN>(p.y).expect("write error");
        ct.cursor.write_f32::<ENDIAN>(p.z).expect("write error");

        num_vertices += 1;
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
        count: USize64::from(num_vertices),
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
            count: USize64::from(num_vertices),
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
            count: USize64::from(num_vertices),
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

    // glTF spec says tangents must be VEC4
    //  xyz for tangent
    //  w for bitangent handedness
    //  idgaf about any of this so im just using 1 for w yolo
    if let Some(tangents) = build.tangents {
        let tangents_start = ct.cursor.position();
        for tangent in tangents {
            let tangent = Vec3::new(-tangent.x, tangent.z, tangent.y);

            ct.cursor.write_f32::<ENDIAN>(tangent.x).expect("write error");
            ct.cursor.write_f32::<ENDIAN>(tangent.y).expect("write error");
            ct.cursor.write_f32::<ENDIAN>(tangent.z).expect("write error");
            ct.cursor.write_f32::<ENDIAN>(1.0).expect("write error");
        }
        let tangents_len = ct.cursor.position() - tangents_start;

        let tangents_view = ct.root.push(json::buffer::View {
            buffer: *ct.buffer_js,
            byte_length: USize64(tangents_len.into()),
            byte_offset: Some(tangents_start.into()),
            byte_stride: Some(json::buffer::Stride(16)),
            name: None,
            target: Some(Valid(json::buffer::Target::ArrayBuffer)),
            extensions: None,
            extras: None
        });

        check_buffer_view!(ct, "tangents_view");

        let tangents_acc = ct.root.push(json::Accessor {
            buffer_view: Some(tangents_view),
            byte_offset: Some(USize64(0)),
            count: USize64::from(num_vertices),
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
        check_buffer_accessor!(ct, "tan_acc");
        attributes.insert(Checked::Valid(Semantic::Tangents), tangents_acc);
    }

    if let Some(normals) = build.normals {
        let normals_start = ct.cursor.position();
        for normal in normals {
            let normal = Vec3::new(-normal.x, normal.z, normal.y);

            ct.cursor.write_f32::<ENDIAN>(normal.x).expect("write error");
            ct.cursor.write_f32::<ENDIAN>(normal.y).expect("write error");
            ct.cursor.write_f32::<ENDIAN>(normal.z).expect("write error");
        }
        let normals_len = ct.cursor.position() - normals_start;

        let normals_view = ct.root.push(json::buffer::View {
            buffer: *ct.buffer_js,
            byte_length: USize64(normals_len.into()),
            byte_offset: Some(normals_start.into()),
            byte_stride: Some(json::buffer::Stride(12)),
            name: None,
            target: Some(Valid(json::buffer::Target::ArrayBuffer)),
            extensions: None,
            extras: None
        });

        check_buffer_view!(ct, "normals_view");

        let normals_acc = ct.root.push(json::Accessor {
            buffer_view: Some(normals_view),
            byte_offset: Some(USize64(0)),
            count: USize64::from(num_vertices),
            component_type: Valid(json::accessor::GenericComponentType(json::accessor::ComponentType::F32)),
            extensions: Default::default(),
            extras: Default::default(),
            type_: Valid(json::accessor::Type::Vec3),
            min: None,
            max: None,
            name: None,
            normalized: false,
            sparse: None
        });
        check_buffer_accessor!(ct, "nrm_acc");
        attributes.insert(Checked::Valid(Semantic::Normals), normals_acc);
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
            byte_offset: Some(colors_start.into()),
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
            count: USize64::from(num_vertices),
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

    if let Some(weights) = build.weights {
        let weights_start = ct.cursor.position();
        for weight in weights {
            for i in 0..4 {
                ct.cursor.write_u32::<ENDIAN>(weight[i].0).expect("write error");
            }
            for i in 0..4 {
                ct.cursor.write_f32::<ENDIAN>(weight[i].1).expect("write error");
            }
        }
        let weights_len = ct.cursor.position() - weights_start;

        let weights_view = ct.root.push(json::buffer::View {
            buffer: *ct.buffer_js,
            byte_length: USize64(weights_len.into()),
            byte_offset: Some(weights_start.into()),
            byte_stride: Some(json::buffer::Stride(8)),
            name: None,
            target: Some(Valid(json::buffer::Target::ArrayBuffer)),
            extensions: None,
            extras: None
        });
    
        check_buffer_view!(ct, "weights_view");

        let joints_acc = ct.root.push(json::Accessor {
            buffer_view: Some(weights_view),
            byte_offset: Some(USize64(0)),
            count: USize64::from(num_vertices),
            component_type: Valid(json::accessor::GenericComponentType(json::accessor::ComponentType::U32)),
            extensions: Default::default(),
            extras: Default::default(),
            type_: Valid(json::accessor::Type::Vec4),
            min: None,
            max: None,
            name: None,
            normalized: false,
            sparse: None
        });
        check_buffer_accessor!(ct, "joints_acc");
    
        let weights_acc = ct.root.push(json::Accessor {
            buffer_view: Some(weights_view),
            byte_offset: Some(USize64(4)),
            count: USize64::from(num_vertices),
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
        check_buffer_accessor!(ct, "weights_acc");

        attributes.insert(Checked::Valid(Semantic::Weights(0)), weights_acc);
        attributes.insert(Checked::Valid(Semantic::Joints(0)), joints_acc);
    }

    let idx_start = ct.cursor.position();
    let mut num_indices: usize = 0;
    for idx in build.indices {
        ct.cursor.write_u32::<ENDIAN>(idx).expect("write error");
        num_indices += 1;
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
        count: USize64::from(num_indices),
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