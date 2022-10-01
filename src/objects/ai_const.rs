use std::{io::{Cursor}, cmp::Ordering, fmt::{Display, Debug}};
use xml::reader::{Error, ErrorKind, EventReader, XmlEvent};
use byteorder::{ReadBytesExt, LittleEndian};
use glam::*;

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

    pub fn contains_node(&self, name: &str) -> bool {
        match 
        self.nodes.iter().find(|n| name.cmp(&n.name) == Ordering::Equal) {
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
            Self::Int(n, i) => write!(f, "{}: {}", n, i),
            Self::Float(n, fl) => write!(f, "{}: {}", n, fl),
            Self::Vec(n, v) => write!(f, "{}: {}", n, v)
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

impl AIConstList {
    pub fn load_from_buf(&mut self, buf: &[u8]) -> Result<(), String> {
        let cursor = Cursor::new(buf);
        let reader = EventReader::new(cursor);
        let mut state = ReaderState::Begin;
        let mut values = &mut self.root_node.values;
        let mut val_name = String::new();
        for ev in reader {
            match ev {
                Ok(XmlEvent::StartElement { name: _, attributes, .. }) => {
                    match state {
                        ReaderState::Begin => state = ReaderState::NextConst,
                        ReaderState::NextConst => {
                            let name = &attributes[0].value;
                            //dbg!(&name);
                            let typ = &attributes[1].value;
                            let mut names: Vec<&str> = name.split('\\').collect();
                            val_name = String::from(names.pop().unwrap());
                            let mut node = &mut self.root_node;
                            for n in names.iter() {
                                if !node.contains_node(*n) {
                                    node.nodes.push(ConstTreeNode { name: String::from(*n), ..Default::default()});
                                }
                                node = node.get_sub_node_from_name(*n).unwrap();
                            }
                            values = &mut node.values;
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
                    //dbg!(&value);
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
                        values.push(val);
                        state = ReaderState::NextConst;
                        val_name = String::new();
                    }
                },
                Err(error) => {
                    println!("{:?}", error);
                },
                _ => continue
            }
        };

        Ok(())
    }

    pub fn unload(&mut self) {
        self.root_node = ConstTreeNode::default();
    }
}