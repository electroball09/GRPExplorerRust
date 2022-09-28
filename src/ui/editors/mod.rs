mod script_editor;
mod blank_editor;
mod ini_editor;
mod curve_editor;
mod otf_editor;
mod layer_editor;
mod gao_editor;

use std::{rc::Rc, cell::RefCell, ops::{Deref, DerefMut}};

pub use script_editor::*;
pub use blank_editor::*;
pub use ini_editor::*;
pub use curve_editor::*;
pub use otf_editor::*;
pub use layer_editor::*;
pub use gao_editor::*;
use crate::{objects::{ObjectArchetype, YetiObject}, bigfile::metadata::ObjectType};

pub trait Editor {
    fn draw(obj: &mut YetiObject, ui: &mut egui::Ui, ctx: &egui::Context);
}

pub fn draw_editor_for_type(obj_type: &ObjectType, obj: &mut YetiObject, ui: &mut egui::Ui, ctx: &egui::Context) {
    match obj_type {
        ObjectType::zc_ => ScriptEditor::draw(obj, ui, ctx),
        ObjectType::ini => IniEditor::draw(obj, ui, ctx),
        ObjectType::cur => CurveEditor::draw(obj, ui, ctx),
        ObjectType::otf => CurveEditor::draw(obj, ui, ctx),
        ObjectType::lay => LayerEditor::draw(obj, ui, ctx),
        ObjectType::gao => GameobjectEditor::draw(obj, ui, ctx),
        _ => BlankEditor::draw(obj, ui, ctx)
    }
}