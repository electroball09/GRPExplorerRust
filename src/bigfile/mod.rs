pub mod metadata;
pub mod io;

use std::collections::HashMap;
use id_tree::*;
use id_tree::InsertBehavior::*;

use metadata::*;
use io::*;

use crate::objects::YetiObject;
use crate::objects::get_archetype_for_type;

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
        
        let v = self.io.read_file(&self.segment_header, &self.bigfile_header, file).unwrap();

        let mut buf: [u8; 4] = [0; 4];
        buf.copy_from_slice(&v[..4]);
        let num_refs = i32::from_le_bytes(buf);
        println!("num refs {}", &num_refs);

        obj.archetype.load_from_buf(&v[(4 + (num_refs as usize) * 4)..])
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

    pub fn print_tree(&self) {
        let root_id = self.tree.root_node_id().unwrap();

        fn recurse(bf: &Bigfile, node_id: &NodeId, indentation: &String) {
            let node = bf.tree.get(node_id).unwrap();
            println!("{}{}", indentation, bf.folder_table[node.data()].get_name());
            let mut new_ind = indentation.clone();
            new_ind += &"-";
            for child in node.children().iter() {
                recurse(bf, child, &new_ind);
            }
        }

        recurse(self, root_id, &String::from(""));
    }

    pub fn folder_idx_to_node(&self, idx: &u16) -> &Node<u16> {
        self.tree.get(&self.node_id_map[idx].0).unwrap()
    }

    pub fn node_id_to_folder(&self, node_id: &NodeId) -> &FolderEntry {
        &self.folder_table[self.tree.get(node_id).unwrap().data()]
    }
}