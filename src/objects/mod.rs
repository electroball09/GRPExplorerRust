use std::{ops::{Deref, DerefMut}, rc::Rc, cell::{RefCell}};

pub mod yeti_script;

use yeti_script::YetiScript;

use crate::bigfile::metadata::ObjectType;

pub struct YetiObject {
    loaded: bool,
    pub archetype: ObjectArchetype,
}

impl YetiObject {
    pub fn is_loaded(&self) -> bool {
        self.loaded
    }
}

pub enum ObjectArchetype {
    NoImpl,
    Script(YetiScript),
}

impl ObjectArchetype {
    pub fn load_from_buf(&mut self, buf: &[u8]) -> Result<(), String> {
        match self {
            Self::Script(script) => script.load_from_buf(buf),
            Self::NoImpl => { Ok(()) }
        }
    }
}

pub fn get_archetype_for_type(obj_type: &ObjectType) -> YetiObject {
    let archetype = match obj_type {
        ObjectType::zc_ => ObjectArchetype::Script(YetiScript::default()),
        _ => ObjectArchetype::NoImpl
    };

    YetiObject {
        loaded: false,
        archetype
    }
}