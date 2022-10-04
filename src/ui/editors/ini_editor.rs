use super::*;
use crate::objects::ini::*;

pub struct IniEditor;

impl Editor for IniEditor {
    fn draw(obj: &mut YetiObject, ui: &mut egui::Ui, ctx: &egui::Context) -> EditorResponse {
        if let ObjectArchetype::Ini(ini) = &obj.archetype {
            for value in ini.entries.iter() {
                if let Some(v) = ui.horizontal(|ui| {
                    match value {
                        IniEntry::Int(key, value) => { 
                            ui.label(format!("{} - {:#010X}", key, value));
                            None
                        },
                        IniEntry::AssetKey(key, value) => {
                            ui.label(format!("{} -", key));
                            if ui.selectable_label(false, format!("{:#010X}", value)).clicked() {
                                return Some(vec![*value]);
                            }
                            None
                        },
                        _ => { 
                            ui.label("invalid entry type");
                            None
                        }
                    }
                }).inner {
                    return EditorResponse {
                        open_new_tab: Some(v),
                        ..Default::default()
                    }
                };
            }
        }

        EditorResponse::default()
    }
}