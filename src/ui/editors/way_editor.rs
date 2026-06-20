use glam::Vec3;

use crate::ui::util::format_bytes_to_hex;

use super::*;

pub struct WayEditor;

impl EditorImpl for WayEditor {
    fn draw(&mut self, key: YKey, ui: &mut egui::Ui, ectx: &mut EditorContext, tctx: &EditorTabContext) {
        let Some(object) = &ectx.bf.object_table.get(&key) else { return; };
        let ObjectArchetype::Way(ref way) = object.archetype else { return; };

        let mut min = Vec3::INFINITY;
        let mut max = Vec3::NEG_INFINITY;
        
        for key in &object.references {
            let ObjectArchetype::GameObject(gao) = &ectx.bf.object_table.get(key).unwrap().archetype else { continue; };

            min = min.min(gao.position());
            max = max.max(gao.position());
        }

        ui.label(format!("bounds min: {}", min));
        ui.label(format!("bounds max: {}", max));

        ui.label(format_bytes_to_hex(&way.unk_dat01));
        ui.label(format!("num way gaos: {}", way.num_way_gaos));

        for (idx, data) in way.way_datas.iter().enumerate() {
            ui.label(format!("index {}: {}", idx, format_bytes_to_hex(data)));
        }
    }
}