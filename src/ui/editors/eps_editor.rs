use super::*;
use crate::objects::eps::*;

pub struct EditableParamStructEditor;

impl EditorImpl for EditableParamStructEditor {
    fn draw(obj: &mut YetiObject, ui: &mut egui::Ui, _ctx: &egui::Context) -> EditorResponse {
        if let ObjectArchetype::EditableParamStruct(eps) = &obj.archetype {
            ui.label(format!("unk_01: {} / {:#010X}", eps.unk_01, eps.unk_01));
            ui.label(format!("data_len: {} / {:#010X}", eps.struct_data_len, eps.struct_data_len));
            ui.label(format!("num_entries: {}", eps.num_entries));
            for value in eps.entries.iter() {
                ui.collapsing(&value.name, |ui| {
                    ui.label(format!("unk_01: {:#04X}", value.unk_01));
                    ui.label(format!("offset: {} / {:#010X}", value.data_offset, value.data_offset));
                });
            }
        }

        EditorResponse::default()
    }
}

pub struct EditableParamsListEditor;

impl EditorImpl for EditableParamsListEditor {
    fn draw(obj: &mut YetiObject, ui: &mut egui::Ui, ctx: &egui::Context) -> EditorResponse {
        if let ObjectArchetype::EditableParamsList(epl) = &obj.archetype {
            ui.label(format!("num: {}", epl.names_list.len()));
            let mut i = 0;
            for name in &epl.names_list {
                ui.label(format!("{} - {:#010X}", &name, obj.references[i]));
                i += 1;
            }
        }

        EditorResponse::default()
    }
}