mod yeti_script; pub use yeti_script::*;
mod ini;         pub use ini::*;
mod curve;       pub use curve::*;
mod otf;         pub use otf::*;
mod layer;       pub use layer::*;
mod gameobject;  pub use gameobject::*;
mod feu;         pub use feu::*;
mod ai_const;    pub use ai_const::*;
mod dbk;         pub use dbk::*;
mod dbr;         pub use dbr::*;
mod epl;         pub use epl::*;
mod meshes;      pub use meshes::*;
mod texture;     pub use texture::*;
mod sound;       pub use sound::*;
mod material;    pub use material::*;
mod shader;      pub use shader::*;
mod skeleton;    pub use skeleton::*;
mod eps;         pub use eps::*;
mod zone;        pub use zone::*;
mod dtb;         pub use dtb::*;
mod vxc;         pub use vxc::*;
mod vxt;         pub use vxt::*;

mod load_error; pub use load_error::*;

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
    TextureMetadata(TextureMetadataObject),
    TextureData(TextureData),
    SoundBank(SoundBank),
    ShaderGraph(VisualShader),
    Skeleton(Skeleton),
    EditableParamStruct(EditableParamStruct),
    Zone(Zone),
    EditableParamsList(EditableParamsList),
    Dbr(Dbr),
    DataTable(DataTable),
    VertexColors(VertexColors),
    Vxt(Vxt),
    GraphicObjectTable(GraphicObjectTable),
}

impl ObjectArchetype {
    fn get_impl(&mut self) -> Option<&mut dyn ArchetypeImpl> {
        let a: Option<&mut dyn ArchetypeImpl> = match self {
            Self::Script                (ref mut arch) => Some(arch),
            Self::Ini                   (ref mut arch) => Some(arch),
            Self::Curve                 (ref mut arch) => Some(arch),
            Self::Otf                   (ref mut arch) => Some(arch),
            Self::Layer                 (ref mut arch) => Some(arch),
            Self::GameObject            (ref mut arch) => Some(arch),
            Self::Feu                   (ref mut arch) => Some(arch),
            Self::ConstList             (ref mut arch) => Some(arch),
            Self::Dbk                   (ref mut arch) => Some(arch),
            Self::MeshData              (ref mut arch) => Some(arch),
            Self::MeshMetadata          (ref mut arch) => Some(arch),
            Self::TextureData           (ref mut arch) => Some(arch),
            Self::TextureMetadata       (ref mut arch) => Some(arch),
            Self::SoundBank             (ref mut arch) => Some(arch),
            Self::Skeleton              (ref mut arch) => Some(arch),
            Self::EditableParamStruct   (ref mut arch) => Some(arch),
            Self::EditableParamsList    (ref mut arch) => Some(arch),
            Self::Zone                  (ref mut arch) => Some(arch),
            Self::Dbr                   (ref mut arch) => Some(arch),
            Self::DataTable             (ref mut arch) => Some(arch),
            Self::VertexColors          (ref mut arch) => Some(arch),
            Self::Vxt                   (ref mut arch) => Some(arch),
            Self::ShaderGraph           (ref mut arch) => Some(arch),
            Self::GraphicObjectTable    (ref mut arch) => Some(arch),
            Self::NoImpl => None
        };

        return a;
    }

    pub fn load_from_buf(&mut self, buf: &[u8]) -> Result<(), LoadError> {
        if let Some(arch) = self.get_impl() {
            return arch.load_from_buf(buf);
        }
        Ok(())
    }

    pub fn unload(&mut self) {
        if let Some(arch) = self.get_impl() {
            arch.unload();
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
            ObjectType::zc  => ObjectArchetype::Script(YetiScript::default()),
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
            ObjectType::tga => ObjectArchetype::TextureMetadata(TextureMetadataObject::default()),
            ObjectType::txd => ObjectArchetype::TextureData(TextureData::default()),
            ObjectType::snk => ObjectArchetype::SoundBank(SoundBank::default()),
            ObjectType::shd => ObjectArchetype::ShaderGraph(VisualShader::default()),
            ObjectType::ske => ObjectArchetype::Skeleton(Skeleton::default()),
            ObjectType::eps => ObjectArchetype::EditableParamStruct(EditableParamStruct::default()),
            ObjectType::zon => ObjectArchetype::Zone(Zone::default()),
            ObjectType::epl => ObjectArchetype::EditableParamsList(EditableParamsList::default()),
            ObjectType::dbr => ObjectArchetype::Dbr(Dbr::default()),
            ObjectType::dtb => ObjectArchetype::DataTable(DataTable::default()),
            ObjectType::vxc => ObjectArchetype::VertexColors(VertexColors::default()),
            ObjectType::vxt => ObjectArchetype::Vxt(Vxt::default()),
            ObjectType::got => ObjectArchetype::GraphicObjectTable(GraphicObjectTable::default()),
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

    pub fn add_ref(&mut self) {
        self.load_refs += 1;
    }

    pub fn load_from_buf(&mut self, buf: &[u8]) -> Result<(), LoadError> {
        let (refs, buf) = crate::bigfile::io::parse_and_remove_refs(buf);
        self.references = refs;

        if let Err(mut error) = self.archetype.load_from_buf(buf) {
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

pub trait ArchetypeImpl {
    fn load_from_buf(&mut self, buf: &[u8]) -> Result<(), LoadError>;
    fn unload(&mut self);
}

