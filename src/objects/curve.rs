use std::io::{Cursor};

use byteorder::{ReadBytesExt, LittleEndian};
use num::FromPrimitive;
use super::{ArchetypeImpl, LoadError};

#[derive(Default)]
pub struct YetiCurve {
    pub curve: CurveType,
}

pub struct CurvePoint {
    pub flags: u8,
    pub x: f32,
    pub y: f32,
    pub in_tangent: f32,
    pub out_tangent: f32
}

#[derive(Default)]
pub enum CurveType {
    #[default]
    Invalid,
    Constant(ConstantCurve),
    Simple(SimpleCurve),
    Full(FullCurve)
}

pub struct ConstantCurve {
    pub point: CurvePoint
}

pub struct SimpleCurve {
    pub points: Vec<CurvePoint>
}

pub struct FullCurve {
    pub flags: u8,
    pub points: Vec<CurvePoint>
}

impl ArchetypeImpl for YetiCurve {
    fn load_from_buf(&mut self, buf: &[u8]) -> Result<(), LoadError> {
        let mut cursor = Cursor::new(buf);
        let curve_type = FromPrimitive::from_i32(cursor.read_i32::<LittleEndian>().unwrap()).unwrap();
        
        let curve = match curve_type {
            0 => Self::load_constant_curve(&mut cursor),
            2 => Self::load_simple_curve(&mut cursor),
            4 => Self::load_full_curve(&mut cursor),
            _ => {
                Err(format!("Invalid curve type: {}", curve_type).into())
            }
        };

        match curve {
            Ok(cur) => {
                self.curve = cur;
                Ok(())
            },
            Err(error) => {
                self.curve = CurveType::Invalid;
                Err(error)
            }
        }
    }

    fn unload(&mut self) {
        self.curve = CurveType::Invalid;
    }
}

impl YetiCurve {
    fn load_constant_curve(buf: &mut Cursor<&[u8]>) -> Result<CurveType, LoadError> {
        let y = buf.read_f32::<LittleEndian>()?;
        let point = CurvePoint {
            flags: 0,
            x: 0.0,
            y,
            in_tangent: 0.0,
            out_tangent: 0.0
        };
        Ok(CurveType::Constant(ConstantCurve { point }))
    }

    fn load_simple_curve(buf: &mut Cursor<&[u8]>) -> Result<CurveType, LoadError> {
        let count = buf.read_u16::<LittleEndian>()?;
        let mut v: Vec<CurvePoint> = Vec::new();

        let mut i = 0;
        while i < count {
            let point = CurvePoint {
                flags: 0,
                x: buf.read_f32::<LittleEndian>()?,
                y: buf.read_f32::<LittleEndian>()?,
                in_tangent: 0.0,
                out_tangent: 0.0
            };

            v.push(point);
            i += 1;
        }

        Ok(CurveType::Simple(SimpleCurve { points: v }))
    }

    fn load_full_curve(buf: &mut Cursor<&[u8]>) -> Result<CurveType, LoadError> {
        let count = buf.read_u16::<LittleEndian>()?;
        let mut v: Vec<CurvePoint> = Vec::new();

        let flags = buf.read_u8()?;

        let mut i = 0;
        while i < count {
            let point = CurvePoint {
                flags: buf.read_u8()?,
                x: buf.read_f32::<LittleEndian>()?,
                y: buf.read_f32::<LittleEndian>()?,
                in_tangent: buf.read_f32::<LittleEndian>()?,
                out_tangent: buf.read_f32::<LittleEndian>()?
            };

            v.push(point);
            i += 1;
        }

        Ok(CurveType::Full(FullCurve {
            points: v,
            flags
        }))
    }
}