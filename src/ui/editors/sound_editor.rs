use super::*;
use crate::objects::ObjectArchetype;

pub struct SnkEditor;

impl EditorImpl for SnkEditor {
    fn draw(&mut self, obj: &mut YetiObject, ui: &mut egui::Ui, _ectx: &mut EditorContext) {
        if let ObjectArchetype::SoundBank(snk) = &obj.archetype {
            ui.collapsing(format!("num nums: {}", snk.numbers.len()), |ui| {
                for n in &snk.numbers {
                    ui.label(format!("{:#010X}", *n));
                }
            });

            ui.label(&snk.name);

            let mut n: u64 = 0;
            for ent in &snk.entries {
                if ent.m_offset != 0xFFFFFFFF {
                    n += ent.m_offset as u64;
                }
            }
            ui.label(format!("predicted size: {} kb", n as f64 / 1024.0));

            ui.collapsing(format!("num entries: {}", snk.entries.len()), |ui| {
                egui::ScrollArea::new([false, true]).auto_shrink([false, false]).show(ui, |ui| {
                    for ent in &snk.entries {
                        ui.collapsing(format!("{:#04X} {}", ent.id, &ent.name), |ui| {
                            ui.label(format!("id: {:#04X}", ent.id));
                            ui.label(format!("offset?: {}", ent.m_offset));
                            ui.label(format!("size?: {}", ent.m_len));
                            ui.label(format!("idk?: {}", ent.m_idk));
                        });
                    }
                })
            });
        }
    }
}