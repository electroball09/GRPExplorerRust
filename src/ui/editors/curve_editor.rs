use crate::objects::{CurveType, CurvePoint};

use super::*;

pub struct CurveEditor;

impl CurveEditor {
    fn curve_line(point: &CurvePoint, ui: &mut egui::Ui)
    {
        ui.horizontal(|ui| {
            ui.label(format!("flags: {:#04X}  x: {}  y: {}  in: {}  out: {}", point.flags, point.x, point.y, point.in_tangent, point.out_tangent));
        });
    }

    fn draw_constant_curve(curve: &mut CurveType, ui: &mut egui::Ui) {
        if let CurveType::Constant(curve) = curve {
            ui.label("constant curve");
            Self::curve_line(&curve.point, ui);
        }
    }
    
    fn draw_simple_curve(curve: &mut CurveType, ui: &mut egui::Ui) {
        if let CurveType::Simple(curve) = curve {
            ui.label("simple curve");
            for point in curve.points.iter() {
                Self::curve_line(&point, ui);
            }
        }
    }
    
    fn draw_full_curve(curve: &mut CurveType, ui: &mut egui::Ui) {
        if let CurveType::Full(curve) = curve {
            ui.label("full curve");
            ui.label(format!("flags: {:#010X}", curve.flags));

            for point in curve.points.iter() {
                Self::curve_line(&point, ui);
            }
        }
    }
}

impl EditorImpl for CurveEditor {
    fn draw(&mut self, key: u32, ui: &mut egui::Ui, ectx: &mut EditorContext, _tctx: &EditorTabContext) {
        if let ObjectArchetype::Curve(ref mut curve) = &mut ectx.bf.object_table.get_mut(&key).unwrap().archetype {
            match &mut curve.curve {
                CurveType::Constant(_cv) => Self::draw_constant_curve(&mut curve.curve, ui),
                CurveType::Simple(_cv) => Self::draw_simple_curve(&mut curve.curve, ui),
                CurveType::Full(_cv) => Self::draw_full_curve(&mut curve.curve, ui),
                CurveType::Invalid => {
                    ui.label("invalid curve type");
                }
            }
        }
    }
}