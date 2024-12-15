use byteorder::{LittleEndian, ReadBytesExt};
use log::*;
use core::str;
use std::{cmp::Ordering, fmt::Display, io::Cursor};
use xml::reader::{EventReader, XmlEvent};
use glam::*;
use super::{ArchetypeImpl, YetiIOError};

pub struct AIConstList {
    pub root_node: ConstTreeNode,
}

#[derive(Default)]
pub struct ConstTreeNode {
    name: String,
    pub nodes: Vec<ConstTreeNode>,
    pub values: Vec<ConstValue>
}

impl ConstTreeNode {
    pub fn get_sub_node_from_name(&mut self, name: &str) -> Option<&mut ConstTreeNode> {
        self.nodes.iter_mut().find(|n| name.cmp(&n.name) == Ordering::Equal)
    }

    pub fn get_or_insert_node(&mut self, name: &str) -> &mut ConstTreeNode {
        if !self.contains_node(name) {
            self.nodes.push(ConstTreeNode {
                name: name.to_string(),
                nodes: Vec::new(),
                values: Vec::new()
            })
        }

        return self.get_sub_node_from_name(name).unwrap();
    }

    pub fn contains_node(&self, name: &str) -> bool {
        match self.nodes.iter().find(|n| name.cmp(&n.name) == Ordering::Equal) {
            None => false,
            Some(_) => true
        }
    }



    pub fn get_name(&self) -> &str {
        &self.name
    }
}

pub enum ConstValue {
    Int(String, i32),
    Float(String, f32),
    Vec(String, Vec3)
}

impl Display for ConstValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Int(n, i) => write!(f, "INT - {}: {}", n, i),
            Self::Float(n, fl) => write!(f, "FLT - {}: {}", n, fl),
            Self::Vec(n, v) => write!(f, "VEC - {}: {}", n, v)
        }
    }
}

impl ConstValue {
    pub fn get_name(&self) -> &String {
        match self {
            ConstValue::Int(str, _) => &str,
            ConstValue::Float(str, _) => &str,
            ConstValue::Vec(str, _) => &str,
        }
    }

    pub fn with_name(&self, name: &str) -> Self {
        match self {
            ConstValue::Int(_, val) => ConstValue::Int(name.to_string(), *val),
            ConstValue::Float(_, val) => ConstValue::Float(name.to_string(), *val),
            ConstValue::Vec(_, val) => ConstValue::Vec(name.to_string(), *val)
        }
    }
}

impl Default for AIConstList {
    fn default() -> Self {
        Self {
            root_node: ConstTreeNode::default()
        }
    }
}

impl AIConstList {
    fn load_xml_consts(buf: &[u8]) -> Result<Vec<ConstValue>, YetiIOError> {
        let mut v = Vec::new();

        enum ReaderState {
            Begin,
            NextConst,
            ReadInt,
            ReadFloat,
            ReadVec,
            ReadVecX,
            ReadVecY(f32),
            ReadVecZ(f32, f32)
        }
        
        let cursor = Cursor::new(buf);
        let reader = EventReader::new(cursor);
        let mut state = ReaderState::Begin;
        let mut val_name = String::new();

        for ev in reader {
            match ev {
                Ok(XmlEvent::StartElement { name: _, attributes, .. }) => {
                    match state {
                        ReaderState::Begin => state = ReaderState::NextConst,
                        ReaderState::NextConst => {
                            val_name = attributes[0].value.clone();
                            let typ = &attributes[1].value;
                            state = match typ.as_str() {
                                "INT" => ReaderState::ReadInt,
                                "FLT" => ReaderState::ReadFloat,
                                "VEC" => ReaderState::ReadVec,
                                _ => ReaderState::NextConst
                            }
                        },
                        ReaderState::ReadVec => {
                            state = ReaderState::ReadVecX;
                        }
                        _ => continue
                    }
                },
                Ok(XmlEvent::Characters(value)) => {
                    let val = match state {
                        ReaderState::ReadInt => {
                            Some(ConstValue::Int(val_name.clone(), value.parse::<i32>().unwrap()))
                        },
                        ReaderState::ReadFloat => {
                            Some(ConstValue::Float(val_name.clone(), value.parse::<f32>().unwrap()))
                        },
                        ReaderState::ReadVecX => {
                            state = ReaderState::ReadVecY(value.parse::<f32>().unwrap());
                            None
                        },
                        ReaderState::ReadVecY(x) => {
                            state = ReaderState::ReadVecZ(x, value.parse::<f32>().unwrap());
                            None
                        },
                        ReaderState::ReadVecZ(x, y) => {
                            Some(ConstValue::Vec(val_name.clone(), Vec3::new(x, y, value.parse::<f32>().unwrap())))
                        },
                        _ => continue
                    };
                    if let Some(val) = val {
                        v.push(val);
                        state = ReaderState::NextConst;
                        val_name = String::new();
                    }
                },
                Err(error) => {
                    error!("{:?}", error);
                    return Err(error.into());
                },
                _ => continue
            }
        };

        Ok(v)
    }

    fn load_binary_consts(buf: &[u8]) -> Result<Vec<ConstValue>, YetiIOError> {
        let mut v = Vec::new();

        let mut cursor = Cursor::new(buf);
        let num = {
            cursor.read_u64::<LittleEndian>()?; //skip first 64 bytes
            cursor.read_u32::<LittleEndian>()?
        };

        for _ in 0..num {
            let typ = cursor.read_u8()?;
            let name = crate::util::read_nul_term_string(&mut cursor)?;

            let cst = match typ {
                1 => ConstValue::Int(name, cursor.read_i32::<LittleEndian>()?),
                2 => ConstValue::Float(name, cursor.read_f32::<LittleEndian>()?),
                3 => ConstValue::Vec(name, Vec3::new(
                    cursor.read_f32::<LittleEndian>()?,
                    cursor.read_f32::<LittleEndian>()?,
                    cursor.read_f32::<LittleEndian>()?
                )),
                v => return Err(format!("unknown ai const type {}", v).into())
            };

            v.push(cst);
        }

        Ok(v)
    }
}

impl ArchetypeImpl for AIConstList {
    fn load_from_buf(&mut self, buf: &[u8]) -> Result<(), YetiIOError> {

        let consts = match (&buf[0..8]).read_u64::<LittleEndian>()? {
            8391171954870665532 => Self::load_xml_consts(buf)?,
            _ => Self::load_binary_consts(buf)?
        };

        for cst in consts {
            let mut names: Vec<&str> = cst.get_name().split("\\").collect();
            let val_name = names.pop().unwrap().to_string();
            let mut node = &mut self.root_node;
            for n in names.iter() {
                node = node.get_or_insert_node(*n);
            }
            node.values.push(cst.with_name(&val_name));
        }

        Ok(())
    }

    fn unload(&mut self) {
        *self = Default::default();
    }
}