use super::*;

pub struct ZoneEditor;

impl EditorImpl for ZoneEditor {
    fn draw(&mut self, key: u32, ui: &mut egui::Ui, ectx: &mut EditorContext, _tctx: &EditorTabContext) {
        if let ObjectArchetype::Zone(zone) = &ectx.bf.object_table.get(&key).unwrap().archetype {
            ui.label(format!("unk_01: {:#04X}", zone.unk_01));
            ui.label(format!("unk_02: {:#04X}", zone.unk_02));
            ui.label(format!("{:?}", zone.zone_type));
            ui.label(format!("unk_04: {:#04X}", zone.unk_04));
            ui.label(format!("unk_05: {:#04X}", zone.unk_05));
            ui.label(format!("unk_06: {:#04X}", zone.unk_06));
        }
    }
}