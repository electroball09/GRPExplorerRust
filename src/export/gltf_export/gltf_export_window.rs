use strum::{EnumIter, IntoEnumIterator};

use super::*;

pub struct GltfExportWindow {
    asset_key: u32,
    asset_name: String,
    options: GltfExportOptions,
    close_requested: bool,
    template: ExportTemplateType,
    edit_strings: OptionStrings,
}

#[derive(Default)]
struct OptionStrings {
    pub directional_light_intensity_multiplier  : String,
    pub spot_light_intentisy_multiplier         : String,
    pub spot_light_range_multiplier             : String,
    pub point_light_intensity_multiplier        : String,
    pub point_light_range_multiplier            : String,
    pub skybox_emissive_multiplier              : String,
}

#[derive(Debug, strum_macros::Display, EnumIter, PartialEq, Clone, Copy)]
enum ExportTemplateType {
    Default,
    Blender,
    UE4,

    Custom
}

fn number_field(ui: &mut egui::Ui, label: &str, num: &mut f32, string: &mut String) {
    let rsp = ui.horizontal(|ui| {
        ui.label(label);
        ui.text_edit_singleline(string)
    }).inner;
    if rsp.lost_focus() {
        if let Ok(n) = string.parse::<f32>() {
            *num = n;
        }
        *string = format!("{}", num);
    }
}

impl GltfExportWindow {
    pub fn new(asset_key: u32, asset_name: &str) -> Self {
        Self {
            asset_key,
            asset_name: asset_name.into(),
            options: Default::default(),
            close_requested: false,
            template: ExportTemplateType::Default,
            edit_strings: {
                let mut strings = OptionStrings::default();
                Self::opt_to_strings(&GltfExportOptions::default(), &mut strings);
                strings
            },
        }
    }

    fn opt_to_strings(options: &GltfExportOptions, strings: &mut OptionStrings) {
        strings.directional_light_intensity_multiplier = format!("{}", options.directional_light_intensity_multiplier   );
        strings.spot_light_intentisy_multiplier        = format!("{}", options.spot_light_intentisy_multiplier          );
        strings.spot_light_range_multiplier            = format!("{}", options.spot_light_range_multiplier              );
        strings.point_light_intensity_multiplier       = format!("{}", options.point_light_intensity_multiplier         );
        strings.point_light_range_multiplier           = format!("{}", options.point_light_range_multiplier             );
        strings.skybox_emissive_multiplier             = format!("{}", options.skybox_emissive_multiplier               );
    }

    pub fn draw(&mut self, ctx: &egui::Context, bf: &Bigfile) -> bool {
        ctx.show_viewport_immediate(egui::ViewportId::from_hash_of(format!("gltf_export_{:#010X}", self.asset_key)),
            egui::ViewportBuilder::default()
            .with_title(format!("Export {} to glTF2.0", self.asset_name))
            .with_maximize_button(false)
            .with_minimize_button(false)
            .with_position([500.0, 200.0])
            .with_drag_and_drop(true)
            .with_inner_size([800.0, 500.0])
            .with_min_inner_size([400.0, 100.0]), 
            |ctx, _class| {
                ctx.input(|state| {
                    if state.viewport().close_requested() {
                        self.close_requested = true;
                    }
                });  

                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.label(format!("asset_key: {:#010X}", self.asset_key));
                    ui.label(format!("asset_name: {}", self.asset_name));

                    ui.separator();

                    ui.label(format!("Export Template: {}", self.template));

                    ui.horizontal_wrapped(|ui| {
                        for e in ExportTemplateType::iter() {
                            ui.radio_value(&mut self.template, e.clone(), e.to_string());
                        }
                    });

                    let (mut options, edit_enabled) = match self.template {
                        ExportTemplateType::Default => (GltfExportOptions::default(), false),
                        ExportTemplateType::Blender => (GltfExportOptions::blender(), false),
                        ExportTemplateType::UE4 => (GltfExportOptions::ue4(), false),
                        ExportTemplateType::Custom => (self.options, true)
                    };

                    if options != self.options {
                        Self::opt_to_strings(&options, &mut self.edit_strings);
                    }

                    ui.add_enabled_ui(edit_enabled, |ui| {
                        number_field(ui, "Dir. Light Multiplier"        , &mut options.directional_light_intensity_multiplier     , &mut self.edit_strings.directional_light_intensity_multiplier );
                        ui.checkbox(&mut options.invert_directional_lights, "Invert Dir. Lights");
                        number_field(ui, "Spot Light Multiplier"        , &mut options.spot_light_intentisy_multiplier            , &mut self.edit_strings.spot_light_intentisy_multiplier        );
                        number_field(ui, "Spot Light Range Multiplier"  , &mut options.spot_light_range_multiplier                , &mut self.edit_strings.spot_light_range_multiplier            );
                        ui.checkbox(&mut options.invert_spot_lights, "Invert Spot Lights");
                        number_field(ui, "Point Light Multiplier"       , &mut options.point_light_intensity_multiplier           , &mut self.edit_strings.point_light_intensity_multiplier       );
                        number_field(ui, "Point Light Range Multiplier" , &mut options.point_light_range_multiplier               , &mut self.edit_strings.point_light_range_multiplier           );
                        number_field(ui, "Skybox Brighness Multiplier"  , &mut options.skybox_emissive_multiplier                 , &mut self.edit_strings.skybox_emissive_multiplier             );
                        ui.checkbox(&mut options.export_collision, "Export Collision");
                    });

                    self.options = options;

                    ui.separator();
                    if ui.button("Export...").clicked() {
                        crate::export::gltf_export(self.asset_key, bf, self.options);
                        self.close_requested = true;
                    }
                });
            }
        );

        self.close_requested
    }
}