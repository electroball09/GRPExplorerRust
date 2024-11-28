use super::*;
use crate::objects::ObjectArchetype;

#[derive(Default)]
pub struct GameobjectEditor {
    
}

impl EditorImpl for GameobjectEditor {
    fn draw(&mut self, key: u32, ui: &mut egui::Ui, ectx: &mut EditorContext) {
        if ui.button("Export to .glb...").clicked() {
            ectx.respond(EditorResponse::GltfExport(key));
        }

        if let ObjectArchetype::GameObject(gao) = &ectx.bf.object_table.get(&key).unwrap().archetype {
            ui.label(format!("zero: {:#010X}", gao.zero));
            ui.label(format!("id flags: {:?}", gao.identity_flags));
            ui.label(format!("str flags: {}", gao.streaming_flags));
            ui.label(format!("flag a: {:#04X}", gao.flag_a));
            ui.label(format!("flag b: {:#04X}", gao.flag_b));
            ui.label(format!("flag c: {:#04X}", gao.flag_c));
            ui.label(format!("pos: {}", gao.position()));
            ui.label(format!("rot: {}", gao.rotation()));
            ui.label(format!("scl: {}", gao.scale()));
        }
    }
}

#[derive(Default)]
pub struct GraphicObjectTableEditor {

}

impl EditorImpl for GraphicObjectTableEditor {
    fn draw(&mut self, key: u32, ui: &mut egui::Ui, ectx: &mut EditorContext) {
        if ui.button("Export to .glb...").clicked() {
            ectx.respond(EditorResponse::GltfExport(key));
        }
    }
}