use crate::{bigfile::{Bigfile, metadata::ObjectType}, objects::ObjectArchetype};
use std::{fs::*, collections::HashSet, io::Write};
use super::super::BfRef;
use log::*;

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
    fn draw(&mut self, ui: &mut egui::Ui, _ctx: &egui::Context) {
        if ui.button("Export Shader Node IDs").clicked() {
            let bf = self.bigfile.clone().unwrap();
            let mut bf = bf.as_ref().borrow_mut();
            export_shader_node_ids(&mut bf);
        }
        if ui.button("Export Zones").clicked() {
            let bf = self.bigfile.clone().unwrap();
            let mut bf = bf.as_ref().borrow_mut();
            export_zones(&mut bf);
        }
    }

    fn set_bigfile(&mut self, bf: crate::ui::BfRef) {
        self.bigfile = bf;
    }
}

fn make_path(file: &str) -> std::io::Result<String> {
    let mut path = String::from(std::env::current_dir().unwrap().to_str().unwrap());
    path += "\\tool_output\\";

    if let Err(_) = read_dir(&path) {
        create_dir(&path)?;
    }

    Ok(path + file)
}

fn export_zones(bf: &mut Bigfile) {
    let path = match make_path("zones.txt") {
        Ok(p) => p,
        Err(e) => {
            error!("{}", e);
            return;
        }
    };

    info!("exporting zones to {}", path);

    if let Ok(mut file) = File::create(path) {
        let keys: Vec<u32> = bf.file_table.iter()
            .filter(|ent| ent.1.object_type == ObjectType::zon)
            .map(|ent| *ent.0)
            .collect();
    
        for key in &keys {
            if let Ok(()) = bf.load_file(*key) {
                if let ObjectArchetype::Zone(zon) = &bf.object_table[&key].archetype {
                    writeln!(file, "{:#010X} - {} - {:?}", key, &bf.file_table[&key].get_name_ext(), zon).unwrap();
                }
            }
            bf.unload_file(*key).unwrap();
        }
    }
}

fn export_shader_node_ids(bf: &mut Bigfile) {
    let path = match make_path("shader_node_ids.txt") {
        Ok(p) => p,
        Err(e) => {
            error!("{}", e);
            return;
        }
    };

    info!("exporting shader node ids to {}", path);

    let shd_keys: Vec<u32> = bf.file_table.iter()
        .filter(|ent| ent.1.object_type == ObjectType::shd)
        .map(|ent| *ent.0)
        .collect();
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
            error!("{}", err.to_string());
        }
    };
}