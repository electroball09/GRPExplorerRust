pub mod yeti_script;
pub mod ini;
pub mod curve;
pub mod otf;
pub mod layer;
pub mod gameobject;

use yeti_script::YetiScript;
use ini::YetiIni;
use curve::*;

use crate::bigfile::metadata::ObjectType;

use self::{otf::Otf, layer::YetiLayer, gameobject::GameObject};

pub struct YetiObject {
    loaded: bool,
    pub references: Vec<u32>,
    pub archetype: ObjectArchetype,
}

impl YetiObject {
    pub fn is_loaded(&self) -> bool {
        self.loaded
    }

    pub fn load_from_buf(&mut self, buf: &[u8]) -> Result<(), String> {
        match self.archetype.load_from_buf(buf) {
            Ok(()) => { self.loaded = true; Ok(()) },
            Err(error) => Err(error)
        }
    }

    pub fn unload(&mut self) {
        self.archetype.unload();
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
            Self::NoImpl => { }
        }
    }
}

pub fn get_archetype_for_type(obj_type: &ObjectType) -> YetiObject {
    let archetype = match obj_type {
        ObjectType::zc_ => ObjectArchetype::Script(YetiScript::default()),
        ObjectType::ini => ObjectArchetype::Ini(YetiIni::default()),
        ObjectType::cur => ObjectArchetype::Curve(YetiCurve::default()),
        ObjectType::otf => ObjectArchetype::Otf(Otf::default()),
        ObjectType::lay => ObjectArchetype::Layer(YetiLayer::default()),
        ObjectType::gao => ObjectArchetype::GameObject(GameObject::default()),
        _ => ObjectArchetype::NoImpl
    };

    YetiObject {
        loaded: false,
        archetype,
        references: Vec::new()
    }
}