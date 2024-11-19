use super::*;
use crate::objects::ObjectArchetype;

#[derive(Default)]
pub struct GameobjectEditor {
    counter: u32
}

impl EditorImpl for GameobjectEditor {
    fn draw(&mut self, obj: &mut YetiObject, ui: &mut egui::Ui, _ectx: &mut EditorContext) {
        self.counter += 1;
        ui.label(format!("counter: {}", &self.counter));

        if let ObjectArchetype::GameObject(gao) = &obj.archetype {
            ui.label(format!("zero: {:#010X}", gao.zero));
            ui.label(format!("id flags: {:?}", gao.identity_flags));
            ui.label(format!("str flags: {}", gao.streaming_flags));
            ui.label(format!("flag a: {:#04X}", gao.flag_a));
            ui.label(format!("flag b: {:#04X}", gao.flag_b));
            ui.label(format!("flag c: {:#04X}", gao.flag_c));
            let (scale, rot, pos) = gao.matrix.to_scale_rotation_translation();
            ui.label(format!("pos: {}", pos));
            ui.label(format!("rot: {}", rot));
            ui.label(format!("scl: {}", scale));
        }
    }
}