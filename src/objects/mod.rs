pub mod yeti_script;
pub mod ini;
pub mod curve;
pub mod otf;
pub mod layer;
pub mod gameobject;
pub mod feu;
pub mod ai_const;
pub mod dbk;
pub mod meshes;
pub mod texture;
pub mod sound;
use yeti_script::*;
use ini::*;
use curve::*;
use otf::*;
use layer::*;
use gameobject::*;
use feu::*;
use ai_const::*;
use dbk::*;
use meshes::*;
use texture::*;
use sound::*;

mod load_error;
pub use load_error::*;

use std::io::Cursor;
use byteorder::{LittleEndian, ReadBytesExt};
use crate::bigfile::metadata::{ObjectType, FileEntry};


pub struct YetiObject {
    load_refs: u32,
    key: u32,
    name: String,
    pub references: Vec<u32>,
    pub archetype: ObjectArchetype,
    pub load_error: Option<LoadError>
}

impl Default for YetiObject {
    fn default() -> Self {
        Self {
            load_refs: 0,
            key: 0xFFFFFFFF,
            name: String::default(),
            references: Vec::new(),
            archetype: ObjectArchetype::NoImpl,
            load_error: None
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
            ObjectType::dbk => ObjectArchetype::Dbk(DynamicBank::default()),
            ObjectType::msh => ObjectArchetype::MeshMetadata(MeshMetadata::default()),
            ObjectType::msd => ObjectArchetype::MeshData(MeshData::default()),
            ObjectType::tga => ObjectArchetype::TextureMetadata(TextureMetadata::default()),
            ObjectType::txd => ObjectArchetype::TextureData(TextureData::default()),
            ObjectType::snk => ObjectArchetype::SoundBank(SoundBank::default()),
            _ => ObjectArchetype::NoImpl
        }
    }

    pub fn is_loaded(&self) -> bool {
        self.load_refs > 0
    }

    pub fn get_key(&self) -> u32 {
        self.key
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn load_from_buf(&mut self, buf: &[u8]) -> Result<(), LoadError> {
        if self.is_loaded() {
            self.load_refs += 1;
            return Ok(());
        }

        let mut cursor = Cursor::new(buf);
        let num_refs = cursor.read_u32::<LittleEndian>().unwrap();
        let mut refs: Vec<u32> = Vec::with_capacity(num_refs as usize);
        //dbg!(num_refs);
        let mut i = 0;
        while i < num_refs {
            refs.push(cursor.read_u32::<LittleEndian>().unwrap());
            i += 1;
        }
        self.references = refs;

        if let Err(mut error) = self.archetype.load_from_buf(&buf[cursor.position() as usize..]) {
            self.archetype.unload();
            error.set_key(self.get_key());
            self.load_error = Some(error);
            return Err(self.load_error.clone().unwrap())
        }

        self.load_refs += 1;
        self.load_error = None;
        Ok(())
    }

    pub fn unload(&mut self) {
        if !self.is_loaded() { return; }

        self.load_refs -= 1;

        if self.load_refs == 0 {
            self.archetype.unload();
            self.references.clear();
            self.references.shrink_to(1);
        }
    }
}

trait ArchetypeImpl {
    fn load_from_buf(&mut self, buf: &[u8]) -> Result<(), LoadError>;
    fn unload(&mut self);
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
    Dbk(DynamicBank),
    MeshData(MeshData),
    MeshMetadata(MeshMetadata),
    TextureMetadata(TextureMetadata),
    TextureData(TextureData),
    SoundBank(SoundBank),
}

impl ObjectArchetype {
    pub fn load_from_buf(&mut self, buf: &[u8]) -> Result<(), LoadError> {
        match self {
            Self::Script(script) => script.load_from_buf(buf),
            Self::Ini(ini) => ini.load_from_buf(buf),
            Self::Curve(curve) => curve.load_from_buf(buf),
            Self::Otf(otf) => otf.load_from_buf(buf),
            Self::Layer(layer) => layer.load_from_buf(buf),
            Self::GameObject(gao) => gao.load_from_buf(buf),
            Self::Feu(feu) => feu.load_from_buf(buf),
            Self::ConstList(list) => list.load_from_buf(buf),
            Self::Dbk(dbk) => dbk.load_from_buf(buf),
            Self::MeshData(msd) => msd.load_from_buf(buf),
            Self::MeshMetadata(msh) => msh.load_from_buf(buf),
            Self::TextureData(txd) => txd.load_from_buf(buf),
            Self::TextureMetadata(tga) => tga.load_from_buf(buf),
            Self::SoundBank(snk) => snk.load_from_buf(buf),
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
            Self::Dbk(dbk) => dbk.unload(),
            Self::MeshData(msd) => msd.unload(),
            Self::MeshMetadata(msh) => msh.unload(),
            Self::TextureData(txd) => txd.unload(),
            Self::TextureMetadata(tga) => tga.unload(),
            Self::SoundBank(snk) => snk.unload(),
            Self::NoImpl => { }
        }
    }
}