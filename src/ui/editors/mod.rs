mod script_editor; use script_editor::*;
mod blank_editor; use blank_editor::*;
mod ini_editor; use ini_editor::*;
mod curve_editor; use curve_editor::*;
mod otf_editor; use otf_editor::*;
mod layer_editor; use layer_editor::*;
mod gao_editor; use gao_editor::*;
mod feu_editor; use feu_editor::*;
mod ai_const_editor; use ai_const_editor::*;
mod dbk_editor; use dbk_editor::*;
mod meshes_editor; use meshes_editor::*;
mod texture_editor; use texture_editor::*;
mod sound_editor; use sound_editor::*;
mod shadergraph_editor; use shadergraph_editor::*;
mod skeleton_editor; use skeleton_editor::*;
mod eps_editor; use eps_editor::*;
mod zone_editor; use zone_editor::*;

use crate::{objects::{ObjectArchetype, YetiObject}, bigfile::{metadata::ObjectType, Bigfile}};

trait EditorImpl {
    fn draw(obj: &mut YetiObject, ui: &mut egui::Ui, ctx: &egui::Context) -> EditorResponse;
}

pub enum EditorResponse {
    None,
    OpenNewTabs(Vec<u32>),
    PerformAction(u32, Box<dyn FnOnce(u32, &mut Bigfile) -> ()>)
}

impl Default for EditorResponse {
    fn default() -> Self {
        EditorResponse::None
    }
}

pub fn draw_editor_for_type(obj_type: &ObjectType, obj: &mut YetiObject, ui: &mut egui::Ui, ctx: &egui::Context) -> EditorResponse {
    if let Some(err) = &obj.load_error {
        ui.label(format!("ERR: {}", err));
        return EditorResponse::default();
    }
    
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
        ObjectType::tga => TextureMetadataEditor::draw(obj, ui, ctx),
        ObjectType::txd => TextureDataEditor::draw(obj, ui, ctx),
        ObjectType::snk => SnkEditor::draw(obj, ui, ctx),
        ObjectType::shd => ShaderGraphEditor::draw(obj, ui, ctx),
        ObjectType::ske => SkeletonEditor::draw(obj, ui, ctx),
        ObjectType::eps => EditableParamStructEditor::draw(obj, ui, ctx),
        ObjectType::zon => ZoneEditor::draw(obj, ui, ctx),
        ObjectType::dbr => DbrEditor::draw(obj, ui, ctx),
        ObjectType::epl => EditableParamsListEditor::draw(obj, ui, ctx),
        _ => BlankEditor::draw(obj, ui, ctx)
    }
}