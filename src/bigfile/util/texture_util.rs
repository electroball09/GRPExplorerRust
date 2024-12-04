use crate::{bigfile::*, objects::*};

pub fn unwrap_tga_key(key: u32, bf: &Bigfile) -> Option<u32> {
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