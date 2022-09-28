pub mod metadata;
pub mod io;

use std::collections::HashMap;
use id_tree::*;
use id_tree::InsertBehavior::*;

use metadata::*;
use io::*;

use crate::objects::YetiObject;
use crate::objects::get_archetype_for_type;

pub fn obj_type_to_name(obj_type: &ObjectType) -> Option<&str> {
    match obj_type {
        ObjectType::ini => Some("Yeti INI"),
        ObjectType::wor => Some("World"),
        ObjectType::gol => Some("World Game Object List"),
        ObjectType::wal => Some("World Way List"),
        ObjectType::lay => Some("World Layer"),
        ObjectType::gao => Some("Game Object"),
        ObjectType::way => Some("Way (?)"),
        ObjectType::cur => Some("Curve"),
        ObjectType::wel => Some("Way External Link"),
        ObjectType::seq => Some("Sequence"),
        ObjectType::got => Some("Graphic Object Table"),
        ObjectType::msh => Some("Mesh Metadata"),
        ObjectType::vxc => Some("Vertex Cache (?)"),
        ObjectType::mat => Some("Material"),
        ObjectType::sha => Some("Shader"),
        ObjectType::tga => Some("Texture Metadata"),
        ObjectType::ske => Some("Skeleton"),
        ObjectType::shd => Some("Visual Shader"),
        ObjectType::dst => Some("DustFX"),
        ObjectType::cub => Some("Cubemap"),
        ObjectType::zc_ => Some("AI Script"),
        ObjectType::acb => Some("Action Bank"),
        ObjectType::act => Some("Action"),
        ObjectType::ani => Some("Animation"),
        ObjectType::aev => Some("Animation Event"),
        ObjectType::snk => Some("Sound Bank"),
        ObjectType::end => Some("Enumerable Descriptor"),
        ObjectType::sam => Some("Sound Ambience"),
        ObjectType::sin => Some("Sound INI"),
        ObjectType::smx => Some("Sound Mix"),
        ObjectType::svs => Some("Sound Volumetric Object"),
        ObjectType::ai_ => Some("AI Model"),
        ObjectType::aiv => Some("AI Variable"),
        ObjectType::zon => Some("Zone"),
        ObjectType::col => Some("Collision ???"),
        ObjectType::cot => Some("Collision Object Table"),
        ObjectType::gml => Some("Game Material List"),
        ObjectType::gmt => Some("Game Material"),
        ObjectType::ccm => Some("Cooked Collision Mesh ???"),
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
        ObjectType::syw => Some("Synapse World ???"),
        ObjectType::txd => Some("Texture Data"),
        ObjectType::fbk => Some("Fire Bank"),
        ObjectType::eps => Some("AI Editable Param Struct"),
        ObjectType::epl => Some("AI Editable Param List"),
        ObjectType::swl => Some("Synapse World List ???"),
        ObjectType::dtb => Some("Data Table"),
        ObjectType::otf => Some("OTF Font"),
        ObjectType::ttf => Some("TTF Font"),
        _ => None
    }
}

pub struct Bigfile {
    pub segment_header: SegmentHeader,
    pub bigfile_header: BigfileHeader,
    pub file_table: HashMap<u32,FileEntry>,
    pub object_table: HashMap<u32, YetiObject>,
    pub folder_table: HashMap<u16, FolderEntry>,
    pub io: Box<dyn BigfileIO>,
    pub tree: Tree<u16>,
    pub file_list_map: HashMap<u16, Box<Vec<u32>>>,
    pub node_id_map: HashMap<u16, (NodeId, u16)>,
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
            tree: TreeBuilder::new().build(),
            file_list_map: HashMap::new(),
            node_id_map: HashMap::new()
        };

        Ok(bigfile)
    }

    pub fn load_metadata(&mut self) -> Result<(), String> {
        println!("loading metadata");
        self.segment_header = match self.io.read_segment_header() {
            Ok(header) => header,
            Err(error) => return Err(String::from(error))
        };
        self.bigfile_header = self.io.read_bigfile_header(&self.segment_header)?;
        self.file_table = self.io.read_file_table(&self.segment_header, &self.bigfile_header)?;
        self.object_table = self.build_archetype_table()?;
        self.folder_table = self.io.read_folder_table(&self.segment_header, &self.bigfile_header)?;
        self.build_file_tree()?;
        println!("all metadata loaded");
        Ok(())
    }

    pub fn load_file(&mut self, key: u32) -> Result<(), String> {
        let file = &self.file_table[&key];
        let obj = self.object_table.get_mut(&key).unwrap();
        if obj.is_loaded() { return Ok(()); }
        
        let bytes = self.io.read_file(&self.segment_header, &self.bigfile_header, file)?;

        let mut buf: [u8; 4] = [0; 4];
        buf.copy_from_slice(&bytes[..4]);
        let num_refs = i32::from_le_bytes(buf);
        let mut refs: Vec<u32> = Vec::new();
        if num_refs > 0 {
            let mut i: usize = 0;
            while i < num_refs as usize {
                let ind = 4 + i * 4;
                buf.copy_from_slice(&bytes[ind..ind + 4]);
                refs.push(u32::from_le_bytes(buf));
                i += 1;
            }
        }

        obj.references = refs;
        obj.load_from_buf(&bytes[(4 + (num_refs as usize) * 4)..])
    }

    pub fn unload_file(&mut self, key: u32) -> Result<(), String> {
        let obj = self.object_table.get_mut(&key).unwrap();
        obj.unload();
        Ok(())
    }

    pub fn extract_file_to_path(&mut self, path: &str, key: u32) -> Result<(), String> {
        let mut file = match std::fs::File::create(path) {
            Ok(file) => file,
            Err(error) => return Err(error.to_string())
        };

        let bytes = self.io.read_file(&self.segment_header, &self.bigfile_header, &self.file_table[&key])?;

        let mut buf: [u8; 4] = [0; 4];
        buf.copy_from_slice(&bytes[..4]);
        let num_refs = i32::from_le_bytes(buf);

        if let Err(error) = std::io::Write::write(&mut file, &bytes[(4 + (num_refs as usize) * 4)..]) {
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

    fn build_archetype_table(&mut self) -> Result<HashMap<u32, YetiObject>, String> {
        let mut table = HashMap::<u32, YetiObject>::new();
        for kv in self.file_table.iter() {
            table.insert(kv.0.clone(), get_archetype_for_type(&kv.1.object_type));
        };
        Ok(table)
    }

    fn build_file_tree(&mut self) -> Result<(), String> {
        //let mut tree = TreeBuilder::new().with_node_capacity(self.folder_table.len() + 1).build();

        println!("building file lists");
        let mut file_list_map: HashMap<u16, Box<Vec<u32>>> = HashMap::new();
        for file in self.file_table.values() {
            file_list_map.entry(file.parent_folder).or_insert(Box::new(Vec::with_capacity(1))).push(file.key);
        }

        println!("sorting file lists");
        for kv in file_list_map.iter_mut() {
            kv.1.sort_by(|a, b| self.file_table[a].get_name().cmp(self.file_table[b].get_name()));
        }

        // println!("building tree");
        // let root_id = tree.insert(Node::new(0), AsRoot).unwrap();
        // let mut node_id_map: HashMap<u16, (NodeId, u16)> = HashMap::with_capacity(self.folder_table.len());
        // for kv in self.folder_table.iter() {
        //     node_id_map.insert(kv.0.clone(), (tree.insert(Node::new(kv.0.clone()), UnderNode(&root_id)).unwrap(), kv.1.parent_folder));
        // }

        // println!("organizing tree");

        // for kv in node_id_map.iter() {
        //     let mut node = &root_id;
        //     if kv.1.1 != 65535 {
        //         node = &node_id_map[&kv.1.1].0;
        //     }
        //     tree.move_node(&kv.1.0, MoveBehavior::ToParent(node)).unwrap();
        // };

        //self.tree = tree;
        self.file_list_map = file_list_map;
        //self.node_id_map = node_id_map;

        Ok(())
    }

}