use super::*;
use crate::objects::ini::*;

pub struct IniEditor;

impl EditorImpl for IniEditor {
    fn draw(&mut self, obj: &mut YetiObject, ui: &mut egui::Ui, ectx: &mut EditorContext) {
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
                                return Some(*value);
                            }
                            None
                        },
                        _ => { 
                            ui.label("invalid entry type");
                            None
                        }
                    }
                }).inner {
                    ectx.respond(EditorResponse::OpenNewTab(v));
                };
            }
        }
    }
}