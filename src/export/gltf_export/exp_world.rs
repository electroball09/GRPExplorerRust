use super::*;
use gltf_json as json;
pub fn gltf_wor<'a>(ct: &'a mut ExportContext) -> Vec<json::Index<json::Node>> {
    check_cache!(ct);

    let refs = &ct.bf.object_table[&ct.key].references;

    let mut nodes = Vec::new();

    for key in refs {
        if ct.bf.is_key_valid(*key) {
            match ct.bf.file_table[key].object_type {
                ObjectType::gol => {
                    ct_with_key!(ct, *key, {
                        for node in gltf_gol(ct).drain(..) {
                            nodes.push(node);
                        }
                    });
                },
                ObjectType::wil => {
                    for subworld in &ct.bf.object_table[key].references {
                        ct_with_key!(ct, *subworld, {
                            nodes.append(&mut gltf_wor(ct));
                        });
                    }
                },
                _ => { }
            };
        }
    }

    for node in nodes.iter() {
        insert_cache!(ct, &ct.key, *node);
    }

    nodes
}

pub fn gltf_gol<'a>(ct: &'a mut ExportContext) -> Vec<json::Index<json::Node>> {
    check_cache!(ct);
    let refs = &ct.bf.object_table[&ct.key].references;

    let mut nodes = Vec::new();

    for key in refs {
        if ct.bf.is_key_valid(*key) {
            ct_with_key!(ct, *key, {
                let mut subnodes = gltf_gao(ct);
                for node in subnodes.drain(..) {
                    nodes.push(node);
                }
            });
        }
    }

    for node in nodes.iter() {
        insert_cache!(ct, &ct.key, *node);
    }

    nodes
}