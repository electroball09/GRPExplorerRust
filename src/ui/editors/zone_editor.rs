use super::*;
use crate::objects::zone::*;

pub struct ZoneEditor;

impl EditorImpl for ZoneEditor {
    fn draw(obj: &mut YetiObject, ui: &mut egui::Ui, ctx: &egui::Context) -> EditorResponse {
        if let ObjectArchetype::Zone(zone) = &obj.archetype {
            ui.label(format!("unk_01: {:#04X}", zone.unk_01));
            ui.label(format!("unk_02: {:#04X}", zone.unk_02));
            ui.label(format!("{:?}", zone.zone_type));
            ui.label(format!("unk_04: {:#04X}", zone.unk_04));
            ui.label(format!("unk_05: {:#04X}", zone.unk_05));
            ui.label(format!("unk_06: {:#04X}", zone.unk_06));
        }

        EditorResponse::default()
    }
}