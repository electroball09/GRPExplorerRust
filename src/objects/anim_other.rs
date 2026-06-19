use std::io::{Cursor, Read};

use byteorder::{LittleEndian, ReadBytesExt};
use strum_macros::{Display};

use crate::objects::ArchetypeImpl;

#[derive(Default, Debug, Display)]
pub enum AnimEventData {
    #[default] None,
    Type01([u8; 30]),
    Type02([u8; 18]),
    Type03([u8; 18]),
}

#[derive(Default, Debug)]
pub struct AnimEvent {
    pub flags: u8,
    pub data: AnimEventData,
}

#[derive(Default, Debug)]
pub struct AnimEventContainer {
    pub version: u32,
    pub events: Vec<AnimEvent>,
}

impl ArchetypeImpl for AnimEventContainer {
    fn load_from_buf(&mut self, buf: &[u8]) -> Result<(), crate::bigfile::YetiIOError> {
        let mut cursor = Cursor::new(buf);

        {
            //signature at top of file: ' aev'
            let mut sig: [u8; 4] = [0; 4];
            cursor.read_exact(&mut sig)?;
            if !(sig[0] == 0x2E && sig[1] == b'a' && sig[2] == b'e' && sig[3] == b'v') {
                log::warn!("weird signature \'{}\'", String::from_utf8_lossy(&sig));
            }
        }

        self.version = cursor.read_u32::<LittleEndian>()?;

        let num_events = cursor.read_u32::<LittleEndian>()?;
        let mut events = Vec::with_capacity(num_events as usize);
        for _ in 0..num_events {
            let data_type = cursor.read_u8()?;
            let flags = cursor.read_u8()?;
            let event_data = match data_type {
                0x01 => {
                    let mut data = [0; 30];
                    cursor.read_exact(&mut data)?;
                    AnimEventData::Type01(data)
                },
                0x02 => {
                    let mut data = [0; 18];
                    cursor.read_exact(&mut data)?;
                    AnimEventData::Type02(data)
                },
                0x03 => {
                    let mut data = [0; 18];
                    cursor.read_exact(&mut data)?;
                    AnimEventData::Type03(data)
                },
                _ => {
                    return Err(format!("invalid event data type: {:#04X}", data_type).into());
                }
            };
            events.push(AnimEvent {
                flags,
                data: event_data
            });
        }
        self.events = events;

        Ok(())
    }

    fn unload(&mut self) {
        
    }
}

#[derive(Default, Debug)]
pub struct ListActionBank {
    pub version: u32,
    pub num_actions: u16
}

impl ArchetypeImpl for ListActionBank {
    fn load_from_buf(&mut self, buf: &[u8]) -> Result<(), crate::bigfile::YetiIOError> {
        let mut cursor = Cursor::new(&buf);

        self.version = cursor.read_u32::<LittleEndian>()?;
        if self.version > 2 {
            log::warn!("weird list action bank version: {}", self.version);
        }
        self.num_actions = cursor.read_u16::<LittleEndian>()?;

        Ok(())
    }

    fn unload(&mut self) {

    }
}

#[derive(Default, Debug)]
pub struct ActionBank {
    pub version: u32,
    pub unk_dat01: [u8; 10]
}

impl ArchetypeImpl for ActionBank {
    fn load_from_buf(&mut self, buf: &[u8]) -> Result<(), crate::bigfile::YetiIOError> {
        let mut cursor = Cursor::new(&buf);

        self.version = cursor.read_u32::<LittleEndian>()?;
        if self.version > 2 {
            log::warn!("weird action bank version: {}", self.version);
        }
        cursor.read_exact(&mut self.unk_dat01)?;

        Ok(())
    }

    fn unload(&mut self) {
        
    }
}

#[derive(Default, Debug)]
pub enum ActionType {
    #[default] None,
    Type01,
    Type02([f32; 3])
}

#[derive(Default, Debug)]
pub struct Action {
    pub action_type: ActionType,
    pub unk_01: u16,
}

impl ArchetypeImpl for Action {
    fn load_from_buf(&mut self, buf: &[u8]) -> Result<(), crate::bigfile::YetiIOError> {
        let mut cursor = Cursor::new(buf);

        let action_type = cursor.read_u32::<LittleEndian>()?;
        self.unk_01 = cursor.read_u16::<LittleEndian>()?;

        self.action_type = match action_type {
            0x01 => ActionType::Type01,
            0x02 => {
                let mut floats = [0.0f32; 3];
                floats[0] = cursor.read_f32::<LittleEndian>()?;
                floats[1] = cursor.read_f32::<LittleEndian>()?;
                floats[2] = cursor.read_f32::<LittleEndian>()?;
                ActionType::Type02(floats)
            },
            _ => {
                return Err(format!("weird action type {}", action_type).into());
            }
        };

        Ok(())
    }

    fn unload(&mut self) {
        
    }
}