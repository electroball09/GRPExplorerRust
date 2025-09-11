use eframe::egui;
use crate::metadata::YKey;
use super::*;

pub struct GltfExportWindow {
    asset_key: YKey,
    asset_name: String,
    options: GltfExportOptions,
    close_requested: bool,
    template: ExportTemplateType,
    edit_strings: OptionStrings,
    map_name: String,
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

#[derive(Debug, strum_macros::Display, strum::EnumIter, PartialEq, Clone, Copy)]
enum ExportTemplateType {
    Default,
    Blender,
    UE4,
    UE5,

    Custom
}

fn number_field(ui: &mut egui::Ui, label: &str, num: &mut f32, string: &mut String) {
    ui.horizontal(|ui| {
        ui.label(label);
        ui.number_field(num, string);
    });
}

impl GltfExportWindow {
    pub fn new(asset_key: YKey, asset_name: &str) -> Self {
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
            map_name: String::new(),
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

    fn is_valid(&self) -> bool {
        !self.map_name.is_empty()
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
                        ui.enum_selector(&mut self.template);
                    });

                    let (mut options, edit_enabled) = match self.template {
                        ExportTemplateType::Default => (GltfExportOptions::default(), false),
                        ExportTemplateType::Blender => (GltfExportOptions::blender(), false),
                        ExportTemplateType::UE4 => (GltfExportOptions::ue4(), false),
                        ExportTemplateType::UE5 => (GltfExportOptions::ue5(), false),
                        ExportTemplateType::Custom => (std::mem::take(&mut self.options), true)
                    };

                    if options != self.options {
                        Self::opt_to_strings(&options, &mut self.edit_strings);
                    }

                    ui.separator();

                    ui.add_enabled_ui(edit_enabled, |ui| {
                        ui.label("LIGHTS");
                        number_field(ui, "Dir. Light Multiplier"        , &mut options.directional_light_intensity_multiplier     , &mut self.edit_strings.directional_light_intensity_multiplier );
                        ui.checkbox(&mut options.invert_directional_lights, "Invert Dir. Lights");
                        number_field(ui, "Spot Light Multiplier"        , &mut options.spot_light_intentisy_multiplier            , &mut self.edit_strings.spot_light_intentisy_multiplier        );
                        number_field(ui, "Spot Light Range Multiplier"  , &mut options.spot_light_range_multiplier                , &mut self.edit_strings.spot_light_range_multiplier            );
                        ui.checkbox(&mut options.invert_spot_lights, "Invert Spot Lights");
                        number_field(ui, "Point Light Multiplier"       , &mut options.point_light_intensity_multiplier           , &mut self.edit_strings.point_light_intensity_multiplier       );
                        number_field(ui, "Point Light Range Multiplier" , &mut options.point_light_range_multiplier               , &mut self.edit_strings.point_light_range_multiplier           );
                        number_field(ui, "Skybox Brighness Multiplier"  , &mut options.skybox_emissive_multiplier                 , &mut self.edit_strings.skybox_emissive_multiplier             );

                        ui.separator();

                        ui.label("EXPORT OPTIONS");
                        ui.checkbox(&mut options.export_collision, "Export Collision");
                        ui.checkbox(&mut options.export_empty_gaos, "Export Empty GAOs");
                        ui.horizontal_wrapped(|ui| {
                            ui.label("Way Export: ");
                            ui.enum_selector(&mut options.way_export_strategy);
                        });
                    });

                    ui.separator();

                    ui.label("Map Name");
                    ui.text_edit_singleline(&mut self.map_name);

                    ui.separator();

                    self.options = options;
                    ui.add_enabled_ui(self.is_valid(), |ui| {
                        if ui.button("Export...").clicked() {
                            self.options.map_name = self.map_name.clone();
                            crate::export::gltf_export(self.asset_key, bf, self.options.clone());
                            self.close_requested = true;
                        }
                    });
                });
            }
        );

        self.close_requested
    }
}