use crate::{bigfile::*, objects::*};

pub fn unwrap_tga_key(key: YKey, bf: &Bigfile) -> Option<YKey> {
    let mut key = key;
    while {
        match &bf.object_table[&key].archetype {
            ObjectArchetype::TextureMetadata(tga) => {
                if let TextureMetaType::Passthrough = tga.meta {
                    true
                } else {
                    false
                }
            },
            _ => return None
        }
    } {
        key = bf.object_table[&key].references[0];
    };
    Some(key)
}