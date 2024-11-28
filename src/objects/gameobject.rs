use std::io::Cursor;

use byteorder::{ReadBytesExt, LittleEndian};
use glam::*;
use bitflags::bitflags;
use super::{ArchetypeImpl, LoadError};
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
}

bitflags! {
    #[derive(Default)]
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

impl ArchetypeImpl for GameObject {
    fn load_from_buf(&mut self, buf: &[u8]) -> Result<(), LoadError> {
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
    fn load_from_buf(&mut self, _buf: &[u8]) -> Result<(), LoadError> {
        Ok(())
    }

    fn unload(&mut self) {
        *self = Default::default();
    }
}