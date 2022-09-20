pub mod metadata;
pub mod io;

use std::collections::HashMap;
use core::iter::Map;
use id_tree::*;
use id_tree::InsertBehavior::*;

use metadata::*;
use io::*;

use crate::util::IndAcc;

#[derive(Debug)]
pub struct Bigfile<T: BigfileIO> {
    pub segment_header: SegmentHeader,
    pub bigfile_header: BigfileHeader,
    pub file_table: HashMap<u32,FileEntry>,
    pub folder_table: HashMap<u16, FolderEntry>,
    pub io: T,
}

impl<T> Bigfile<T> where T: BigfileIO {
    pub fn new(path: &str) -> Result<Bigfile<T>, String> {
        let io = match T::create_from_path(path) {
            Ok(io) => io,
            Err(error) => return Err(error.to_string())
        };
        
        let bigfile = Bigfile {
            io,
            segment_header: SegmentHeader::default(),
            bigfile_header: BigfileHeader::default(),
            file_table: HashMap::new(),
            folder_table: HashMap::new(),
        };

        Ok(bigfile)
    }

    pub fn load_metadata(&mut self) -> Result<(), String> {
        self.segment_header = match self.io.load_segment_header() {
            Ok(header) => header,
            Err(error) => return Err(String::from(error))
        };
        self.bigfile_header = self.io.load_bigfile_header(&self.segment_header)?;
        self.file_table = self.io.load_file_table(&self.segment_header, &self.bigfile_header)?;
        self.folder_table = self.io.load_folder_table(&self.segment_header, &self.bigfile_header)?;
        BigfileTree::build(&self);
        // dbg!(&self.folder_table[&37]);
        // println!("{}", &self.folder_table[&37].get_name());
        Ok(())
    }
}

pub struct BigfileTree<'a, T: BigfileIO> {
    pub bigfile: &'a Bigfile<T>,
    pub tree: Tree<IndAcc<'a, HashMap<u16, FolderEntry>, &'a u16>>
}

impl <'a, T: BigfileIO> BigfileTree<'a, T> {
    pub fn build(bigfile: &'a Bigfile<T>) -> Self {
        let mut tree = TreeBuilder::new().with_node_capacity((bigfile.bigfile_header.num_folders + 1) as usize).build();

        println!("building tree");

        let root_id = tree.insert(Node::new(IndAcc::new(&bigfile.folder_table, &0)), AsRoot).unwrap();

        let mut map: HashMap<&u16, (NodeId, &u16)> = HashMap::with_capacity(bigfile.bigfile_header.num_folders as usize);
        for kv in bigfile.folder_table.iter() {
            map.insert(kv.0, (tree.insert(Node::new(IndAcc::new(&bigfile.folder_table, &kv.0)), UnderNode(&root_id)).unwrap(), &kv.1.parent_folder));
        }

        println!("organizing tree");

        for kv in map.iter() {
            let mut node = &root_id;
            if kv.1.1 != &65535 {
                node = &map[kv.1.1].0;
            }
            tree.move_node(&kv.1.0, MoveBehavior::ToParent(node)).unwrap();
        }

        BigfileTree {
            bigfile,
            tree
        }
    }
}