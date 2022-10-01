use std::fs::File;

use crate::objects::ObjectArchetype;
use std::io::Write;
use super::{Editor, EditorResponse};

pub struct FeuEditor;

impl Editor for FeuEditor {
    fn draw(obj: &mut crate::objects::YetiObject, ui: &mut egui::Ui, ctx: &egui::Context) -> EditorResponse {
        if let ObjectArchetype::Feu(feu) = &obj.archetype {
            if ui.button("Extract to SWF...").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                    let mut path = String::from(path.to_str().unwrap());
                    path += &format!("/{}.swf", obj.get_name());

                    println!("extracting feu to {}", path);

                    if let Ok(mut file) = File::create(path) {
                        file.write(&[b'F', b'W', b'S']).unwrap();
                        file.write(&feu.feu_data[3..]).unwrap();
                    }
                }
            }
    
            ui.label(format!("unk_01: {}", feu.unk_01));
            ui.label(format!("unk_02: {}", feu.unk_02));
            ui.add_space(5.0);
            ui.label(format!("fire refs: {}", feu.feu_refs.len()));
            egui::ScrollArea::new([false, true]).auto_shrink([false, true]).max_height(350.0).show(ui,|ui| {
                for fref in feu.feu_refs.iter() {
                    ui.label(fref);
                }
            });
            ui.add_space(5.0);
            ui.label(format!("data len: {}", feu.feu_data.len()));
        }

        EditorResponse::default()
    }
}