use std::clone;

use crate::objects::curve::{CurveType, CurvePoint};

use super::*;

pub struct CurveEditor {

}

impl CurveEditor {
    fn curve_line(point: &CurvePoint, ui: &mut egui::Ui)
    {
        ui.horizontal(|ui| {
            ui.label(format!("flags: {:#04X}  x: {}  y: {}  in: {}  out: {}", point.flags, point.x, point.y, point.in_tangent, point.out_tangent));
        });
    }

    fn draw_constant_curve(curve: &mut CurveType, ui: &mut egui::Ui, ctx: &egui::Context) {
        if let CurveType::Constant(curve) = curve {
            ui.label("constant curve");
            Self::curve_line(&curve.point, ui);
        }
    }
    
    fn draw_simple_curve(curve: &mut CurveType, ui: &mut egui::Ui, ctx: &egui::Context) {
        if let CurveType::Simple(curve) = curve {
            ui.label("simple curve");
            for point in curve.points.iter() {
                Self::curve_line(&point, ui);
            }
        }
    }
    
    fn draw_full_curve(curve: &mut CurveType, ui: &mut egui::Ui, ctx: &egui::Context) {
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
    fn draw(obj: &mut YetiObject, ui: &mut egui::Ui, ctx: &egui::Context) -> EditorResponse {
        ui.vertical(|ui| {
            if let ObjectArchetype::Curve(curve) = &mut obj.archetype {
                match &mut curve.curve {
                    CurveType::Constant(_cv) => Self::draw_constant_curve(&mut curve.curve, ui, ctx),
                    CurveType::Simple(_cv) => Self::draw_simple_curve(&mut curve.curve, ui, ctx),
                    CurveType::Full(_cv) => Self::draw_full_curve(&mut curve.curve, ui, ctx),
                    CurveType::Invalid => {
                        ui.label("invalid curve type");
                    }
                }
            }
        });

        EditorResponse::default()
    }
}