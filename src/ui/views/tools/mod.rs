use crate::{bigfile::{Bigfile, metadata::ObjectType}, objects::ObjectArchetype};
use std::{fs::File, collections::HashSet, io::Write};
use super::super::BfRef;

type ToolCoroutine = Box<dyn FnMut(&mut Bigfile, Vec<u32>, u32) -> bool>;

pub struct ToolsView {
    bigfile: BfRef,
}

impl ToolsView {
    pub fn new(bf: BfRef) -> Self {
        Self {
            bigfile: bf,
        }
    }
}

impl super::View for ToolsView {
    fn draw(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        if ui.button("Export Shader Node IDs").clicked() {
            let bf = self.bigfile.clone().unwrap();
            let mut bf = bf.as_ref().borrow_mut();
            export_shader_node_ids(&mut bf);
        }
    }

    fn set_bigfile(&mut self, bf: crate::ui::BfRef) {
        self.bigfile = bf;
    }

    fn settings_menu(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        
    }
}

pub fn export_shader_node_ids(bf: &mut Bigfile) {
    let mut path = String::from(std::env::current_dir().unwrap().to_str().unwrap());
    path += "\\shader_node_ids.txt";

    println!("exporting shader node ids to {}", path);

    let shd_keys: Vec<u32> = bf.file_table.iter().filter(|ent| ent.1.object_type == ObjectType::shd).map(|ent| *ent.0).collect();
    let mut ids: HashSet<String> = HashSet::new();
    for key in &shd_keys {
        if let Ok(()) = bf.load_file(*key) {
            if let ObjectArchetype::ShaderGraph(shd) = &bf.object_table[&key].archetype {
                for graph in &shd.graphs {
                    for node in &graph.nodes {
                        ids.insert(String::from(node.get_id()));
                    }
                }
            }
        }
        bf.unload_file(*key).unwrap();
    }

    match File::create(path) {
        Ok(mut file) => {
            let mut v = Vec::from_iter(ids);
            v.sort();

            let mut n: Vec<String> = Vec::with_capacity(shd_keys.len());
            for key in shd_keys {
                n.push(format!("{} {:#010X}", bf.file_table[&key].get_name(), key));
            }
            n.sort();

            for name in n {
                writeln!(file, "{}", name).unwrap();
            }
            
            for id in v {
                writeln!(file, "{}", id).unwrap();
            }
        },
        Err(err) => {
            println!("{}", err.to_string());
        }
    };
}