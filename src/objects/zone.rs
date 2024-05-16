use std::io::Cursor;
use byteorder::{LittleEndian, ReadBytesExt};
use log::*;
use super::{ArchetypeImpl, LoadError};

#[derive(Debug, Default, Clone, Copy)]
pub struct Zone {
    pub unk_01: u8,
    pub unk_02: u8,
    pub zone_type: ZoneType,
    pub unk_04: u8,
    pub unk_05: u8,
    pub unk_06: u8
}

#[derive(Debug, Default, Clone, Copy)]
pub enum ZoneType {
    #[default]
    Point,
    Ray,
    Sphere(ZoneSphere),
    AABox,
    OBox,
    Capsule
}

pub trait ZoneTypeTrait {
    fn load_from_buf(&mut self, cursor: &mut Cursor<&[u8]>) -> Result<(), LoadError>;
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct ZoneSphere {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub radius: f32,
    pub unk_01: f32
}

impl ZoneTypeTrait for ZoneSphere {
    fn load_from_buf(&mut self, cursor: &mut Cursor<&[u8]>) -> Result<(), LoadError> {
        self.x = cursor.read_f32::<LittleEndian>()?;
        self.y = cursor.read_f32::<LittleEndian>()?;
        self.z = cursor.read_f32::<LittleEndian>()?;
        self.radius = cursor.read_f32::<LittleEndian>()?;
        self.unk_01 = cursor.read_f32::<LittleEndian>()?;

        Ok(())
    }
}

#[derive(FromPrimitive, ToPrimitive, Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum ZoneModType {
    #[default]
    Invalid = 0,
    Ambient = 1,
    Fog = 2,
    PostFx = 3,
    SunLight = 4,
    Sector = 5,
    CharacterDiffuseMultiplier = 6
}

impl ArchetypeImpl for Zone {
    fn load_from_buf(&mut self, buf: &[u8]) -> Result<(), LoadError> {
        let mut cursor = Cursor::new(buf);
        self.unk_01 = cursor.read_u8()?;
        self.unk_02 = cursor.read_u8()?;
        self.zone_type = get_zone(cursor.read_u8()?);
        self.unk_04 = cursor.read_u8()?;
        self.unk_05 = cursor.read_u8()?;
        self.unk_06 = cursor.read_u8()?;

        match &mut self.zone_type {
            ZoneType::Sphere(sphere) => sphere.load_from_buf(&mut cursor)?,
            _ => ()
        }
        
        Ok(())
    }

    fn unload(&mut self) {

    }
}

impl Zone {
    pub fn get_zone_type_id(&self) -> u8 {
        match self.zone_type {
            ZoneType::Point => 0,
            ZoneType::Ray => 1,
            ZoneType::Sphere(_) => 2,
            ZoneType::AABox => 3,
            ZoneType::OBox => 4,
            ZoneType::Capsule => 5
        }
    }
}

fn get_zone(byte: u8) -> ZoneType {
    match byte {
        0 => ZoneType::Point,
        1 => ZoneType::Ray,
        2 => ZoneType::Sphere(ZoneSphere::default()),
        3 => ZoneType::AABox,
        4 => ZoneType::OBox,
        5 => ZoneType::Capsule,
        _ => {
            error!("invalid zone type! {:#04X}", byte);
            ZoneType::Point
        }
    }
}