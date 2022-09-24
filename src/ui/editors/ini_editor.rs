use super::*;
use crate::objects::ini::*;

pub struct IniEditor {

}

impl Editor for IniEditor {
    fn draw(obj: &mut YetiObject, ui: &mut egui::Ui, ctx: &egui::Context) {
        if let ObjectArchetype::Ini(ini) = &obj.archetype {
            for kv in ini.entries.iter() {
                ui.horizontal(|ui| {
                    ui.label(kv.0);
                    ui.label("-");
                    match kv.1 {
                        IniEntry::Int(value) => { ui.label(format!("{:#010X}", value)); },
                        IniEntry::AssetKey(value) => { ui.label(format!("{:#010X}", value)); },
                        _ => { }
                    }
                });
            }
        }
    }
}