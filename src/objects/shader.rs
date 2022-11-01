use std::io::{Read, Seek, Cursor, SeekFrom};
use byteorder::{ReadBytesExt, LittleEndian};
use super::{ArchetypeImpl, LoadError};

#[derive(Default)]
pub struct VisualShader {
    pub flags: u32,
    pub graphs: Vec<ShaderGraph>,
}

pub struct ShaderGraph {
    pub unk_01: u32,
    pub unk_02: u32,
    pub unk_03: u32,
    pub unk_04: u32,
    pub num_nodes: u32,
    pub unk_06: u32,
    pub nodes: Vec<ShaderNode>,
}

pub struct ShaderNode {
    id: String,
    pub i1: u32,
    pub i2: u32
}

impl ShaderNode {
    pub fn get_id(&self) -> &str {
        &self.id
    }
}

impl ArchetypeImpl for VisualShader {
    fn load_from_buf(&mut self, buf: &[u8]) -> Result<(), LoadError> {
        let mut cursor = Cursor::new(buf);

        self.flags = cursor.read_u32::<LittleEndian>()?;

        cursor.seek(std::io::SeekFrom::Current(64))?;
        loop {
            self.graphs.push(Self::read_graph(&mut cursor)?);
            if !Self::seek_to_next_node(&mut cursor) {
                break;
            }
            cursor.seek(SeekFrom::Current(-24))?;
        }

        Ok(())
    }

    fn unload(&mut self) {
        self.graphs.clear();
        self.graphs.shrink_to(0);
    }
}

impl VisualShader {
    fn read_graph<T: Seek + Read>(rdr: &mut T) -> Result<ShaderGraph, LoadError> {
        let mut graph = ShaderGraph {
            unk_01: rdr.read_u32::<LittleEndian>()?,
            unk_02: rdr.read_u32::<LittleEndian>()?,
            unk_03: rdr.read_u32::<LittleEndian>()?,
            unk_04: rdr.read_u32::<LittleEndian>()?,
            num_nodes: rdr.read_u32::<LittleEndian>()?,
            unk_06: rdr.read_u32::<LittleEndian>()?,
            nodes: Vec::new()
        };

        let mut nodes: Vec<ShaderNode> = Vec::with_capacity(graph.num_nodes as usize);
        let mut i = 0;
        while i < graph.num_nodes {
            nodes.push(Self::read_node(rdr)?);
            Self::seek_to_next_node(rdr);
            i += 1;
        };
        graph.nodes = nodes;

        Ok(graph)
    }

    fn read_node_id(rdr: &mut impl Read) -> Result<String, LoadError> {
        let len = rdr.read_u32::<LittleEndian>()? as usize;

        let mut buf: [u8; 256] = [0; 256];
        rdr.read(&mut buf[..len])?;

        let id = String::from_utf8(buf[..len].to_vec())?;
        Ok(id)
    }

    fn read_node(rdr: &mut impl Read) -> Result<ShaderNode, LoadError> {
        let id = Self::read_node_id(rdr)?;

        Ok(ShaderNode {
            id,
            i1: rdr.read_u32::<LittleEndian>()?,
            i2: rdr.read_u32::<LittleEndian>()?
        })
    }

    fn seek_to_next_node<T: Seek + Read>(rdr: &mut T) -> bool {
        fn check_esid(buf: &[u8]) -> bool {
            buf[0] == b'e' &&
            buf[1] == b'S' &&
            buf[2] == b'I' &&
            buf[3] == b'D'
        }

        let mut buf: [u8; 4] = [0; 4];
        let mut num: usize = 4;
        while num == 4 {
            match rdr.read(&mut buf) {
                Ok(n) => {
                    if check_esid(&buf) {
                        rdr.seek(SeekFrom::Current(-8)).expect("how is this even possible");
                        return true;
                    }
                    num = n;
                    rdr.seek(SeekFrom::Current(-3)).expect("how is this even possible");
                },
                Err(_err) => {
                    return false;
                }
            }
        };

        false
    }
}