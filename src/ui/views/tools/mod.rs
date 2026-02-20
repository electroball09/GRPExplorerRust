use crate::{bigfile::{metadata::ObjectType, Bigfile}, metadata::YKey, objects::ObjectArchetype, ui::AppContext};
use std::{collections::HashSet, fs::{self, *}, io::{self, Write}};
use log::*;
use crate::egui as egui;

pub struct ToolsView {
    
}

impl ToolsView {
    pub fn new() -> Self {
        Self {
            
        }
    }
}

impl super::View for ToolsView {
    fn draw(&mut self, ui: &mut egui::Ui, mut app: AppContext) {
        if let Some(ref mut bf) = app.bigfile {
            if ui.button("Export Shader Node IDs").clicked() {
                export_shader_node_ids(bf);
            }
            if ui.button("Export Zones").clicked() {
                export_zones(bf);
            }
            if ui.button("Export node hierarchy for .glb file").clicked() {
                export_node_hierarchy();
            }
        }
    }
}

fn make_tool_output_file_path(file: &str) -> std::io::Result<String> {
    let mut path = String::from(std::env::current_dir().unwrap().to_str().unwrap());
    path += "\\tool_output\\";

    if let Err(_) = read_dir(&path) {
        create_dir(&path)?;
    }

    Ok(path + file)
}

fn export_node_hierarchy() {
    let path = match rfd::FileDialog::new().add_filter("glTF2.0", &["glb"]).pick_file() {
        Some(path) => { path },
        None => return
    };

    let output_name = make_tool_output_file_path(&(path.file_name().unwrap().to_str().unwrap().to_string() + ".txt")).unwrap();

    let mut output_file = fs::File::create(output_name).unwrap();

    run_export_hierarchy(path.to_str().unwrap(), &mut output_file);
}

fn print_tree(node: &gltf::Node, depth: i32, output_file: &mut File) {
    for _ in 0..(depth - 1) {
        write!(output_file, "  ").unwrap();
    }
    write!(output_file, " -").unwrap();
    write!(output_file, " Node {}", node.index()).unwrap();
    if let Some(v) = node.extras() {
        write!(output_file, " Extras {}", v.to_string()).unwrap();
    }
    writeln!(output_file).unwrap();

    for child in node.children() {
        print_tree(&child, depth + 1, output_file);
    }
}

fn run_export_hierarchy(path: &str, output_file: &mut File) {
    let file = fs::File::open(path).unwrap();
    let reader = io::BufReader::new(file);
    let gltf = gltf::Gltf::from_reader(reader).unwrap();
    for scene in gltf.scenes() {
        write!(output_file, "Scene {}", scene.index()).unwrap();
        writeln!(output_file).unwrap();
        for node in scene.nodes() {
            print_tree(&node, 1, output_file);
        }
    }
}

fn export_zones(bf: &mut Bigfile) {
    let path = match make_tool_output_file_path("zones.txt") {
        Ok(p) => p,
        Err(e) => {
            error!("{}", e);
            return;
        }
    };

    info!("exporting zones to {}", path);

    if let Ok(mut file) = File::create(path) {
        let keys: Vec<YKey> = bf.file_table.iter()
            .filter(|ent| ent.1.object_type == ObjectType::zon)
            .map(|ent| *ent.0)
            .collect();
    
        for key in &keys {
            if let Ok(_) = bf.load_file(*key) {
                if let ObjectArchetype::Zone(zon) = &bf.object_table[&key].archetype {
                    writeln!(file, "{:#010X} - {} - {:?}", key, &bf.file_table[&key].get_name_ext(), zon).unwrap();
                }
            }
            bf.unload_file(*key).unwrap();
        }
    }
}

fn export_shader_node_ids(bf: &mut Bigfile) {
    let path = match make_tool_output_file_path("shader_node_ids.txt") {
        Ok(p) => p,
        Err(e) => {
            error!("{}", e);
            return;
        }
    };

    info!("exporting shader node ids to {}", path);

    let shd_keys: Vec<YKey> = bf.file_table.iter()
        .filter(|ent| ent.1.object_type == ObjectType::shd)
        .map(|ent| *ent.0)
        .collect();
    let mut ids: HashSet<String> = HashSet::new();
    for key in &shd_keys {
        if let Ok(_) = bf.load_file(*key) {
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