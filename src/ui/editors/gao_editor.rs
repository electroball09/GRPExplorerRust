
use glam::{Mat4, Vec3, Quat};

use crate::objects::ObjectArchetype;

use super::{EditorImpl, EditorResponse};

pub struct GameobjectEditor;

impl EditorImpl for GameobjectEditor {
    fn draw(obj: &mut crate::objects::YetiObject, ui: &mut egui::Ui, ctx: &egui::Context) -> EditorResponse {
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

        EditorResponse::default()
    }
}