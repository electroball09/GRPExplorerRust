pub mod yeti_script;
pub mod ini;
pub mod curve;
pub mod otf;
pub mod layer;
pub mod gameobject;
pub mod feu;
pub mod ai_const;

use std::io::Cursor;

use byteorder::{LittleEndian, ReadBytesExt};
use yeti_script::YetiScript;
use ini::YetiIni;
use curve::*;

use crate::bigfile::metadata::{ObjectType, FileEntry};

use self::{otf::Otf, layer::YetiLayer, gameobject::GameObject, feu::Feu, ai_const::AIConstList};

pub struct YetiObject {
    loaded: bool,
    key: u32,
    name: String,
    pub references: Vec<u32>,
    pub archetype: ObjectArchetype,
}

impl Default for YetiObject {
    fn default() -> Self {
        Self {
            loaded: false,
            key: 0xFFFFFFFF,
            name: String::default(),
            references: Vec::new(),
            archetype: ObjectArchetype::NoImpl
        }
    }
}

impl YetiObject {
    pub fn from_file_entry(entry: &FileEntry) -> Self {
        Self {
            key: entry.key,
            name: String::from(entry.get_name()),
            archetype: Self::type_to_archetype(&entry.object_type),
            ..Default::default()
        }
    }

    pub fn type_to_archetype(obj_type: &ObjectType) -> ObjectArchetype {
        match obj_type {
            ObjectType::zc_ => ObjectArchetype::Script(YetiScript::default()),
            ObjectType::ini => ObjectArchetype::Ini(YetiIni::default()),
            ObjectType::cur => ObjectArchetype::Curve(YetiCurve::default()),
            ObjectType::otf => ObjectArchetype::Otf(Otf::default()),
            ObjectType::lay => ObjectArchetype::Layer(YetiLayer::default()),
            ObjectType::gao => ObjectArchetype::GameObject(GameObject::default()),
            ObjectType::feu => ObjectArchetype::Feu(Feu::default()),
            ObjectType::cst => ObjectArchetype::ConstList(AIConstList::default()),
            _ => ObjectArchetype::NoImpl
        }
    }

    pub fn is_loaded(&self) -> bool {
        self.loaded
    }

    pub fn get_key(&self) -> u32 {
        self.key
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn load_from_buf(&mut self, buf: &[u8]) -> Result<(), String> {
        let mut cursor = Cursor::new(buf);
        let num_refs = cursor.read_u32::<LittleEndian>().unwrap();
        let mut refs: Vec<u32> = Vec::with_capacity(num_refs as usize);
        dbg!(num_refs);
        let mut i = 0;
        while i < num_refs {
            refs.push(cursor.read_u32::<LittleEndian>().unwrap());
            i += 1;
        }
        self.references = refs;

        match self.archetype.load_from_buf(&buf[cursor.position() as usize..]) {
            Ok(()) => { self.loaded = true; Ok(()) },
            Err(error) => Err(error)
        }
    }

    pub fn unload(&mut self) {
        self.archetype.unload();
        self.references.clear();
        self.references.shrink_to(1);
        self.loaded = false;
    }
}

pub enum ObjectArchetype {
    NoImpl,
    Script(YetiScript),
    Ini(YetiIni),
    Curve(YetiCurve),
    Otf(Otf),
    Layer(YetiLayer),
    GameObject(GameObject),
    Feu(Feu),
    ConstList(AIConstList),
}

impl ObjectArchetype {
    pub fn load_from_buf(&mut self, buf: &[u8]) -> Result<(), String> {
        match self {
            Self::Script(script) => script.load_from_buf(buf),
            Self::Ini(ini) => ini.load_from_buf(buf),
            Self::Curve(curve) => curve.load_from_buf(buf),
            Self::Otf(otf) => otf.load_from_buf(buf),
            Self::Layer(layer) => layer.load_from_buf(buf),
            Self::GameObject(gao) => gao.load_from_buf(buf),
            Self::Feu(feu) => feu.load_from_buf(buf),
            Self::ConstList(list) => list.load_from_buf(buf),
            Self::NoImpl => { Ok(()) }
        }
    }

    pub fn unload(&mut self) {
        match self {
            Self::Script(script) => script.unload(),
            Self::Ini(ini) => ini.unload(),
            Self::Curve(curve) => curve.unload(),
            Self::Otf(otf) => otf.unload(),
            Self::Layer(layer) => layer.unload(),
            Self::GameObject(gao) => gao.unload(),
            Self::Feu(feu) => feu.unload(),
            Self::ConstList(list) => list.unload(),
            Self::NoImpl => { }
        }
    }
}