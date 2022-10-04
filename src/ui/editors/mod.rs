mod script_editor;
mod blank_editor;
mod ini_editor;
mod curve_editor;
mod otf_editor;
mod layer_editor;
mod gao_editor;
mod feu_editor;
mod ai_const_editor;
mod dbk_editor;
mod meshes_editor;

use script_editor::*;
use blank_editor::*;
use ini_editor::*;
use curve_editor::*;
use otf_editor::*;
use layer_editor::*;
use gao_editor::*;
use feu_editor::*;
use ai_const_editor::*;
use dbk_editor::*;
use meshes_editor::*;
use crate::{objects::{ObjectArchetype, YetiObject}, bigfile::{metadata::ObjectType, Bigfile}};

pub trait Editor {
    fn draw(obj: &mut YetiObject, ui: &mut egui::Ui, ctx: &egui::Context) -> EditorResponse;
}

pub trait PerformEditorAction {
    fn do_action(bf: &Bigfile);
}

pub struct EditorResponse {
    pub open_new_tab: Option<Vec<u32>>,
    pub perform_action: Option<Box<dyn FnOnce(&Bigfile) -> ()>>,
}

impl Default for EditorResponse {
    fn default() -> Self {
        EditorResponse {
            open_new_tab: None,
            perform_action: None,
        }
    }
}

pub fn draw_editor_for_type(obj_type: &ObjectType, obj: &mut YetiObject, ui: &mut egui::Ui, ctx: &egui::Context) -> EditorResponse {
    match obj_type {
        ObjectType::zc_ => ScriptEditor::draw(obj, ui, ctx),
        ObjectType::ini => IniEditor::draw(obj, ui, ctx),
        ObjectType::cur => CurveEditor::draw(obj, ui, ctx),
        ObjectType::otf => OtfEditor::draw(obj, ui, ctx),
        ObjectType::lay => LayerEditor::draw(obj, ui, ctx),
        ObjectType::gao => GameobjectEditor::draw(obj, ui, ctx),
        ObjectType::feu => FeuEditor::draw(obj, ui, ctx),
        ObjectType::cst => AIConstEditor::draw(obj, ui, ctx),
        ObjectType::dbk => DbkEditor::draw(obj, ui, ctx),
        ObjectType::msd => MeshDataEditor::draw(obj, ui, ctx),
        _ => BlankEditor::draw(obj, ui, ctx)
    }
}