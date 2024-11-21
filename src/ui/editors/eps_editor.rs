use super::*;

pub struct EditableParamStructEditor;

impl EditorImpl for EditableParamStructEditor {
    fn draw(&mut self, key: u32, ui: &mut egui::Ui, ectx: &mut EditorContext) {
        if let ObjectArchetype::EditableParamStruct(eps) = &ectx.bf.object_table.get(&key).unwrap().archetype {
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
    }
}

pub struct EditableParamsListEditor;

impl EditorImpl for EditableParamsListEditor {
    fn draw(&mut self, key: u32, ui: &mut egui::Ui, ectx: &mut EditorContext) {
        let obj = &ectx.bf.object_table.get(&key).unwrap();
        if let ObjectArchetype::EditableParamsList(epl) = &obj.archetype {
            ui.label(format!("num: {}", epl.names_list.len()));
            let mut i = 0;
            for name in &epl.names_list {
                ui.label(format!("{} - {:#010X}", &name, obj.references[i]));
                i += 1;
            }
        }
    }
}