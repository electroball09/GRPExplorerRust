use std::{ops::Deref, rc::Rc, cell::RefCell};

use super::Editor;
use crate::objects::{*, yeti_script::YetiScript};

pub struct ScriptEditor {

}

impl ScriptEditor {

}

impl Editor for ScriptEditor {
    fn draw(obj: &mut YetiObject, ui: &mut egui::Ui, ctx: &egui::Context) {
        if let ObjectArchetype::Script(script) = &obj.archetype {
            let buf = script.buffer.clone();
            let mut script_string = String::from_utf8(buf).unwrap();
            ui.text_edit_multiline(&mut script_string);
        } else {
            ui.label(format!("wrong editor ???"));
        }
    }
}