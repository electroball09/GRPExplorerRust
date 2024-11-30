use crate::objects::*;
use super::*;
use gltf_json as json;
use json::validation::Checked::Valid;
use json::validation::USize64;

pub fn gltf_msh<'a>(ct: &'a mut ExportContext) -> Vec<json::Index<json::Mesh>> {
    gltf_export_init!(ct);

    let msd_key = ct.bf.object_table[&ct.key].references[0];

    let msh = match &ct.bf.object_table[&ct.key].archetype {
        ObjectArchetype::MeshMetadata(msh) => msh,
        _ => panic!("wrong object type!")
    };
    let msh_name = ct.bf.file_table[&ct.key].get_name().to_string();

    let msd = match &ct.bf.object_table[&msd_key].archetype {
        ObjectArchetype::MeshData(msd) => msd,
        _ => panic!("wrong object type!")
    };
    let _msd_name = ct.bf.file_table[&msd_key].get_name().to_string();

    let vtx_size: usize = 12 + 16;
    let face_size: usize = 2;

    let mut vec = Vec::new();

    let buffer_start = ct.cursor.position() as usize;
    for idx in 0..msd.num_vertices as usize {
        let vtx = msd.vertex_data.pos[idx];
        let uv0 = msd.vertex_data.uv0[idx];
        let uv1 = msd.vertex_data.uv1[idx];

        ct.cursor.write_f32::<ENDIAN>(-vtx.x).expect("write error"); // negate x coord for blender
        ct.cursor.write_f32::<ENDIAN>(vtx.z).expect("write error"); // flip y and z coords for blender
        ct.cursor.write_f32::<ENDIAN>(vtx.y).expect("write error");
        ct.cursor.write_f32::<ENDIAN>(uv0.x).expect("write error");
        ct.cursor.write_f32::<ENDIAN>(uv0.y).expect("write error");
        ct.cursor.write_f32::<ENDIAN>(uv1.x).expect("write error");
        ct.cursor.write_f32::<ENDIAN>(uv1.y).expect("write error");
    }
    let vbuf_len = ct.cursor.position() as usize - buffer_start;

    let vtx_view = ct.root.push(json::buffer::View {
        buffer: *ct.buffer_js,
        byte_length: USize64::from(vbuf_len),
        byte_offset: Some(USize64::from(buffer_start)),
        byte_stride: Some(json::buffer::Stride(vtx_size)),
        extensions: Default::default(),
        extras: Default::default(),
        name: None,
        target: Some(Valid(json::buffer::Target::ArrayBuffer))
    });

    let (min, max) = msd.bounding_box();
    let min: &[f32] = &min.to_array();
    let max: &[f32] = &max.to_array();
    let pos_acc = ct.root.push(json::Accessor {
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

    let uv0_acc = ct.root.push(json::Accessor {
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

    let uv1_acc = ct.root.push(json::Accessor {
        buffer_view: Some(vtx_view),
        byte_offset: Some(USize64(20)),
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

    let mut prims = Vec::new();

    for idx in 0..msh.submeshes.len() {
        let submesh = &msh.submeshes[idx];
        let sbf_start = ct.cursor.position() as usize;
        for v in 0..submesh.face_num as usize {
            assert!(submesh.face_start % 3 == 0);
            let idx = ((submesh.face_start / 3) as usize) + v;

            let face = &msd.faces[idx];
            ct.cursor.write_u16::<ENDIAN>(face.f0).unwrap();
            ct.cursor.write_u16::<ENDIAN>(face.f1).unwrap();
            ct.cursor.write_u16::<ENDIAN>(face.f2).unwrap();
        }
        let sbf_len: usize = ct.cursor.position() as usize - sbf_start;

        let face_view = ct.root.push(json::buffer::View {
            buffer: *ct.buffer_js,
            byte_length: USize64::from(sbf_len),
            byte_offset: Some(USize64::from(sbf_start)),
            byte_stride: Some(json::buffer::Stride(face_size)),
            extensions: Default::default(),
            extras: Default::default(),
            name: None,
            target: Some(Valid(json::buffer::Target::ArrayBuffer))
        });
    
        let face_acc = ct.root.push(json::Accessor {
            buffer_view: Some(face_view),
            byte_offset: Some(USize64(0)),
            count: USize64::from(submesh.face_num as usize * 3 as usize),
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
                map.insert(Valid(json::mesh::Semantic::TexCoords(0)), uv0_acc);
                map.insert(Valid(json::mesh::Semantic::TexCoords(1)), uv1_acc);
                map
            },
            extensions: Default::default(),
            extras: Default::default(),
            indices: Some(face_acc),
            material: None,
            mode: Valid(json::mesh::Mode::Triangles),
            targets: None
        };

        prims.push(primitive);
    };
    
    let mesh = ct.root.push(json::Mesh {
        extensions: Default::default(),
        extras: Default::default(),
        name: Some(format!("{:#010X} {}", ct.key, msh_name)),
        primitives: prims,
        weights: None
    });

    insert_cache!(ct, &ct.key, mesh);

    vec.push(mesh);

    vec
}