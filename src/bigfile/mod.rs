pub mod metadata;
pub mod io;
pub mod loader;
pub mod util;

use log::*;

use std::collections::HashMap;

use metadata::*;
use io::*;

use crate::objects::{YetiObject, LoadError};

pub fn obj_type_to_name(obj_type: &ObjectType) -> Option<&str> {
    match obj_type {
        ObjectType::ini => Some("Yeti INI"),
        ObjectType::wor => Some("World"),
        ObjectType::woc => Some("World Engine Config"),
        ObjectType::gol => Some("World Game Object List"),
        ObjectType::wal => Some("World Way List"),
        ObjectType::lay => Some("World Layer"),
        ObjectType::pco => Some("Precomputed Occlusion"),
        ObjectType::gao => Some("Game Object"),
        ObjectType::way => Some("Way ?"),
        ObjectType::cur => Some("Curve"),
        ObjectType::wel => Some("Way External Link"),
        ObjectType::seq => Some("Sequence"),
        ObjectType::got => Some("Graphic Object Table"),
        ObjectType::msh => Some("Mesh Metadata"),
        ObjectType::vxc => Some("Vertex Colors ?"),
        ObjectType::mat => Some("Material"),
        ObjectType::sha => Some("Shader"),
        ObjectType::tga => Some("Texture Metadata"),
        ObjectType::ske => Some("Skeleton"),
        ObjectType::shd => Some("Visual Shader"),
        ObjectType::dst => Some("DustFX"),
        ObjectType::cub => Some("Cubemap"),
        ObjectType::zc => Some("AI Script"),
        ObjectType::acb => Some("Action Bank"),
        ObjectType::act => Some("Action"),
        ObjectType::ani => Some("Animation"),
        ObjectType::aev => Some("Animation Event"),
        ObjectType::end => Some("Enumerable Descriptor"),
        ObjectType::snk => Some("Sound Bank"),
        ObjectType::sam => Some("Sound Ambience"),
        ObjectType::sin => Some("Sound INI"),
        ObjectType::smx => Some("Sound Mix"),
        ObjectType::svs => Some("Sound Volumetric Object"),
        ObjectType::ai => Some("AI Model"),
        ObjectType::aiv => Some("AI Variable"),
        ObjectType::zon => Some("Zone"),
        ObjectType::col => Some("Collision List"),
        ObjectType::cot => Some("Collision Object Table"),
        ObjectType::ccm => Some("Convex Collision Mesh"),
        ObjectType::gml => Some("Game Material List"),
        ObjectType::gmt => Some("Game Material"),
        ObjectType::dbk => Some("Dynamic Bank"),
        ObjectType::dbl => Some("Dynamic Bank List"),
        ObjectType::dbr => Some("Dynamic Bank Reference List"),
        ObjectType::wil => Some("World Include List"),
        ObjectType::lab => Some("List Action Bank"),
        ObjectType::feu => Some("Fire Package"),
        ObjectType::ffd => Some("Fire Font Package"),
        ObjectType::top => Some("World Topography"),
        ObjectType::msd => Some("Mesh Data"),
        ObjectType::nav => Some("World Nav Data"),
        ObjectType::cst => Some("AI Const List"),
        ObjectType::syw => Some("Synapse World ??"),
        ObjectType::swl => Some("Synapse World List ??"),
        ObjectType::txd => Some("Texture Data"),
        ObjectType::fbk => Some("Fire Bank"),
        ObjectType::eps => Some("AI Editable Param Struct"),
        ObjectType::epl => Some("AI Editable Param List"),
        ObjectType::dtb => Some("Data Table"),
        ObjectType::otf => Some("OTF Font"),
        ObjectType::ttf => Some("TTF Font"),
        ObjectType::zar => Some("Zone Array ?"),
        _ => None
    }
}

pub struct Bigfile {
    pub segment_header: SegmentHeader,
    pub bigfile_header: BigfileHeader,
    pub file_table: HashMap<YKey, FileEntry>,
    pub object_table: HashMap<YKey, YetiObject>,
    pub folder_table: HashMap<u16, FolderEntry>,
    pub io: Box<dyn BigfileIO>,
    pub file_list_map: HashMap<u16, Box<Vec<YKey>>>,
}

impl Bigfile {
    pub fn new<T: BigfileIO + 'static>(path: String) -> Result<Bigfile, String> {
        let path = String::from(path);
        let io = match T::create_from_path(&path) {
            Ok(io) => io,
            Err(error) => return Err(error.to_string())
        };
        
        let bigfile = Bigfile {
            io: Box::new(io),
            segment_header: SegmentHeader::default(),
            bigfile_header: BigfileHeader::default(),
            file_table: HashMap::new(),
            object_table: HashMap::new(),
            folder_table: HashMap::new(),
            file_list_map: HashMap::new()
        };

        Ok(bigfile)
    }

    pub fn load_metadata(&mut self) -> Result<(), LoadError> {
        info!("loading metadata");
        self.segment_header = self.io.read_segment_header()?;
        self.bigfile_header = self.io.read_bigfile_header(&self.segment_header)?;
        self.file_table = self.io.read_file_table(&self.segment_header, &self.bigfile_header)?;
        self.object_table = self.build_archetype_table()?;
        self.folder_table = self.io.read_folder_table(&self.segment_header, &self.bigfile_header)?;
        self.build_file_tree()?;
        info!("all metadata loaded");
        Ok(())
    }

    pub fn load_file(&mut self, key: YKey) -> Result<bool, LoadError> {
        if let Some(file) = self.file_table.get(&key.into()) {
            let obj = self.object_table.get_mut(&key.into()).unwrap();
            if obj.is_loaded() { 
                obj.add_ref();
                return Ok(false); 
            }
            
            let bytes = self.io.read_file(&self.segment_header, &self.bigfile_header, file)?;
    
            obj.load_from_buf(&bytes)?;

            return Ok(true)
        } else {
            Err("file not found!".into())
        }
    }

    pub fn unload_file(&mut self, key: YKey) -> Result<(), String> {
        if let Some(obj) = self.object_table.get_mut(&key.into()) {
            obj.unload();
            Ok(())
        } else {
            Err("file not found!".into())
        }
    }

    pub fn is_key_valid(&self, key: YKey) -> bool {
        if let Some(file) = self.file_table.get(&key.into()) {
            if file.offset != 0xFFFFFFFF {
                return true;
            }
        }

        return false;
    }

    pub fn extract_file_to_path(&mut self, path: &str, key: YKey) -> Result<(), String> {
        let mut file = match std::fs::File::create(path) {
            Ok(file) => file,
            Err(error) => return Err(error.to_string())
        };

        let bytes = self.io.read_file(&self.segment_header, &self.bigfile_header, &self.file_table[&key.into()])?;

        let mut buf: [u8; 4] = [0; 4];
        buf.copy_from_slice(&bytes[..4]);
        let num_refs = i32::from_le_bytes(buf);

        let refs_len = 4 + (num_refs as usize) * 4;

        if let Err(error) = std::io::Write::write(&mut file, &bytes[refs_len..]) {
            return Err(error.to_string());
        }

        Ok(())
    }

    pub fn get_full_directory(&self, folder: u16) -> String {
        let mut dir = String::new();

        let mut dirs: Vec<u16> = Vec::new();
        dirs.push(folder);
        let mut parent = self.folder_table[&folder].parent_folder;
        while parent != 0xFFFF {
            dirs.push(parent);
            parent = self.folder_table[&parent].parent_folder;
        };

        for folder in dirs.iter().rev() {
            dir += &String::from(self.folder_table[&folder].get_name());
            dir += "/";
        }

        dir
    }

    pub fn log_loaded_objects(&self) {
        for obj in self.object_table.values() {
            if obj.is_loaded() {
                info!("{} {:#010X}", obj.get_name(), obj.get_key());
            }
        }
    }

    fn build_archetype_table(&mut self) -> Result<HashMap<YKey, YetiObject>, String> {
        let mut table = HashMap::<YKey, YetiObject>::new();
        for kv in self.file_table.iter() {
            table.insert(*kv.0, YetiObject::from_file_entry(&self.file_table[kv.0]));
        };
        Ok(table)
    }

    fn build_file_tree(&mut self) -> Result<(), String> {
        info!("building file lists");
        let mut file_list_map: HashMap<u16, Box<Vec<YKey>>> = HashMap::new();
        for file in self.file_table.values() {
            file_list_map.entry(file.parent_folder).or_insert(Box::new(Vec::with_capacity(1))).push(file.key);
        }

        info!("sorting file lists");
        for kv in file_list_map.iter_mut() {
            kv.1.sort_by(|a, b| self.file_table[a].get_name().cmp(self.file_table[b].get_name()));
        }

        self.file_list_map = file_list_map;

        Ok(())
    }

}