use std::io::{Cursor, Read};

use byteorder::{ReadBytesExt, LittleEndian};
use glam::*;
use bitflags::bitflags;
use log::warn;
use super::{ArchetypeImpl, YetiIOError};
use crate::util::load_util::*;

#[derive(Default)]
pub struct GameObject {
    pub zero: u32,
    pub identity_flags: IdentityFlags,
    pub streaming_flags: u32,
    pub flag_a: u8,
    pub flag_b: u8,
    pub flag_c: u8,
    pub matrix: Mat4,
    pub light: Light,
}

bitflags! {
    #[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct IdentityFlags: u32 {
        const ENTITY_FLAG            = 1 << 0;
        const HAS_ATTACHMENTS        = 1 << 1;
        const INITIAL_POS            = 1 << 2;
        const COLLISION_A            = 1 << 3;
        const HAS_ANIM               = 1 << 4;
        const COLLISION_LIST_HANDLE  = 1 << 5;
        const DUSTFX                 = 1 << 6;
        const HAS_PHYSICS            = 1 << 7;
        const HAS_VISUAL             = 1 << 8;
        const HAS_AABOX              = 1 << 9;
        const GAM_STRUCT             = 1 << 10;
        const ZONE_ARRAY             = 1 << 11;
        const AI_STRUCT              = 1 << 12;
        const SEQUENCE               = 1 << 13;
        const SFX_STRUCT             = 1 << 14;
        const UNK_02                 = 1 << 15;
        const UNK_03                 = 1 << 16;
        const DUSTFX_STRUCT          = 1 << 17;
        const UNK_04                 = 1 << 18;
        const COLLISION_LIST_STRUCT  = 1 << 19;
        const UNK_05                 = 1 << 20;
        const UNK_06                 = 1 << 21;
        const UNK_07                 = 1 << 22;
        const SND_OBJECT             = 1 << 23;
        const UNK_08                 = 1 << 24;
        const GAME_GROUP             = 1 << 25;
        const GAME_HOOK              = 1 << 26;
        const UNK_09                 = 1 << 27;
        const SFX_OBJECT_STRUCT      = 1 << 28;
        const EDITABLE_BVOLUME       = 1 << 29;
        const ORIENTED_BOX           = 1 << 30;
        const SPAWNER_BANK           = 1 << 31;
    }
}

#[derive(Default, Debug)]
pub enum Light {
    #[default]
    None,
    Point(PointLightParams),
    Spot(SpotLightParams),
    Directional(DirectionalLightParams)
}

#[derive(Default, Debug)]
pub struct PointLightParams {
    pub color: Vec4,
    pub intensity: f32,
    pub range: f32
}

#[derive(Default, Debug)]
pub struct SpotLightParams {
    pub color: Vec4,
    pub intensity: f32,
    pub range: f32,
    pub inner_cone_angle: f32,
    pub outer_cone_angle: f32
}

#[derive(Default, Debug)]
pub struct DirectionalLightParams {
    pub color: Vec4,
    pub intensity: f32
}

impl GameObject {
    pub fn position(&self) -> Vec3 {
        let (_, _, pos) = self.matrix.to_scale_rotation_translation();
        pos
    }

    pub fn rotation(&self) -> Quat {
        let (_, rot, _) = self.matrix.to_scale_rotation_translation();
        rot
    }

    pub fn scale(&self) -> Vec3 {
        let (scl, _, _) = self.matrix.to_scale_rotation_translation();
        scl
    }
}

impl ArchetypeImpl for GameObject {
    fn load_from_buf(&mut self, buf: &[u8]) -> Result<(), YetiIOError> {
        let mut cursor = Cursor::new(buf);

        self.zero = cursor.read_u32::<LittleEndian>()?;
        if self.zero != 0{
            return Err("GameObject sanity check was not zero!".into());
        }
        self.identity_flags = IdentityFlags::from_bits(cursor.read_u32::<LittleEndian>()?).unwrap();
        self.streaming_flags = cursor.read_u32::<LittleEndian>()?;
        self.flag_a = cursor.read_u8()?;
        self.flag_b = cursor.read_u8()?;
        self.flag_c = cursor.read_u8()?;
        self.matrix = read_mat4(&mut cursor)?;

        if self.identity_flags.contains(IdentityFlags::SFX_STRUCT | IdentityFlags::HAS_VISUAL) {
            let lt = cursor.read_u8()?;
            let mut dat: [u8; 8] = [0; 8];
            cursor.read(&mut dat)?;
            match lt {
                1 => {
                    self.light = Light::Point(PointLightParams {
                        color: Vec4::new(
                            cursor.read_u8()? as f32 / 255.0,
                            cursor.read_u8()? as f32 / 255.0,
                            cursor.read_u8()? as f32 / 255.0,
                            cursor.read_u8()? as f32 / 255.0
                        ).zyxw(), // for some reason yeti light colors are bgra
                        intensity: cursor.read_f32::<LittleEndian>()?,
                        range: {
                            cursor.read_f32::<LittleEndian>()?;
                            cursor.read_f32::<LittleEndian>()?
                        }
                    });
                },
                2 => {
                    self.light = Light::Spot( SpotLightParams {
                        color: Vec4::new(
                            cursor.read_u8()? as f32 / 255.0,
                            cursor.read_u8()? as f32 / 255.0,
                            cursor.read_u8()? as f32 / 255.0,
                            cursor.read_u8()? as f32 / 255.0
                        ).zyxw(), // for some reason yeti light colors are bgra
                        intensity: cursor.read_f32::<LittleEndian>()?,
                        range: {
                            cursor.read_f32::<LittleEndian>()?;
                            cursor.read_f32::<LittleEndian>()?
                        },
                        inner_cone_angle: {
                            cursor.read_f32::<LittleEndian>()?;
                            cursor.read_f32::<LittleEndian>()?;
                            cursor.read_f32::<LittleEndian>()?
                        },
                        outer_cone_angle: cursor.read_f32::<LittleEndian>()?
                    });
                },
                3 => {
                    self.light = Light::Directional(DirectionalLightParams {
                        color: Vec4::new(
                            cursor.read_u8()? as f32 / 255.0,
                            cursor.read_u8()? as f32 / 255.0,
                            cursor.read_u8()? as f32 / 255.0,
                            cursor.read_u8()? as f32 / 255.0
                        ).zyxw(), // for some reason yeti light colors are bgra
                        intensity: cursor.read_f32::<LittleEndian>()?,
                    });
                },
                _ => {
                    self.light = Light::None;
                    //return Err(format!("weird light type?? {}", lt).into());
                    warn!("weird light type?? {}", lt);
                }
            }
        }

        Ok(())
    }

    fn unload(&mut self) {
        *self = Self {
            ..Default::default()
        }
    }
}

#[derive(Default)]
pub struct GraphicObjectTable {

}

impl ArchetypeImpl for GraphicObjectTable {
    fn load_from_buf(&mut self, _buf: &[u8]) -> Result<(), YetiIOError> {
        Ok(())
    }

    fn unload(&mut self) {
        *self = Default::default();
    }
}