use crate::{objects::{ActionType, AnimEventData}, ui::{editors::EditorImpl, util::format_bytes_to_hex_wrapped}};
use super::*;


pub struct AnimEventEditor;

impl EditorImpl for AnimEventEditor {
    fn draw(&mut self, key: YKey, ui: &mut egui::Ui, ectx: &mut EditorContext, _tctx: &EditorTabContext) {
        let Some(aev) = ectx.bf.object_table.get(&key) else { return; };
        let ObjectArchetype::AnimEvent(aev) = &aev.archetype else { return; };

        ui.label(format!("num events: {}", aev.events.len()));
        for (idx, event) in aev.events.iter().enumerate() {
            ui.collapsing(format!("event index {} type {}", idx, event.data), |ui| {
                ui.label(format!("event type {}", event.data));
                ui.label(format!("flags {:08b}", event.flags));
                ui.label(match &event.data {
                    AnimEventData::None => String::from("None"),
                    AnimEventData::Type01(buf) => format_bytes_to_hex_wrapped(buf),
                    AnimEventData::Type02(buf) => format_bytes_to_hex_wrapped(buf),
                    AnimEventData::Type03(buf) => format_bytes_to_hex_wrapped(buf)
                })
            });
        }
    }
}

pub struct ListActionBankEditor;

impl EditorImpl for ListActionBankEditor {
    fn draw(&mut self, key: YKey, ui: &mut egui::Ui, ectx: &mut EditorContext, tctx: &EditorTabContext) {
        let ObjectArchetype::ListActionBank(lab) = &ectx.bf.object_table.get(&key).unwrap().archetype else { return; };

        ui.label(format!("version: {}", lab.version));
        ui.label(format!("num actions: {}", lab.num_actions));
    }
}

pub struct ActionBankEditor;

impl EditorImpl for ActionBankEditor {
    fn draw(&mut self, key: YKey, ui: &mut egui::Ui, ectx: &mut EditorContext, tctx: &EditorTabContext) {
        let ObjectArchetype::ActionBank(acb) = &ectx.bf.object_table.get(&key).unwrap().archetype else { return; };

        ui.label(format!("version: {}", acb.version));
        ui.label(format_bytes_to_hex_wrapped(&acb.unk_dat01));
    }
}

pub struct ActionEditor;

impl EditorImpl for ActionEditor {
    fn draw(&mut self, key: YKey, ui: &mut egui::Ui, ectx: &mut EditorContext, tctx: &EditorTabContext) {
        let ObjectArchetype::Action(act) = &ectx.bf.object_table.get(&key).unwrap().archetype else { return; };

        ui.label(format!("unk_01: {:#06X}", act.unk_01));
        match act.action_type {
            ActionType::None => { ui.label("none action type???"); },
            ActionType::Type01 => { ui.label("Type01"); },
            ActionType::Type02(data) => { 
                ui.label("Type02");
                ui.label(format!("f0: {}", data[0]));
                ui.label(format!("f1: {}", data[1]));
                ui.label(format!("f2: {}", data[2]));
            }
        }
    }
}