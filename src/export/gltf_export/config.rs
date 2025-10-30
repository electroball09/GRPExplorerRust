
use std::collections::HashMap;
use serde::Deserialize;

use crate::bigfile::metadata::YKey;


#[derive(Deserialize, Debug)]
pub struct ExportConfig {
    pub capture_ids: HashMap<String, CaptureWay>,
    pub spawn_zone_ids: HashMap<String, SpawnZoneWay>,
    pub capture_visual_scripts: HashMap<String, CaptureVisualScript>,
    pub material_shader_type_ids: MaterialShaderTypeIDs,
}

#[derive(Deserialize, Debug)]
pub struct CaptureWay {
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct SpawnZoneWay {
    pub name: String,
    pub team: i32,
}

#[derive(Deserialize, Debug)]
pub struct CaptureVisualScript {
    pub for_point: String
}

#[derive(Deserialize, Debug)]
pub struct MaterialShaderTypeIDs {
    pub alphatest: Vec<String>,
    pub alphablend_emissive: Vec<String>,
    pub alphablend: Vec<String>,
    pub invcoloralpha: Vec<String>,
    pub coloralpha: Vec<String>,
    pub skybox: Vec<String>,
    pub submarine: Vec<String>,
    pub standard: Vec<String>,
}

impl MaterialShaderTypeIDs {
    pub fn has_alphatest_key(&self, mat: &YKey, shd: &YKey) -> bool {
        let v = &self.alphatest;
        v.contains(&mat.to_string()) || v.contains(&shd.to_string())
    }

    pub fn has_alphablend_emissive_key(&self, mat: &YKey, shd: &YKey) -> bool {
        let v = &self.alphablend_emissive;
        v.contains(&mat.to_string()) || v.contains(&shd.to_string())
    }

    pub fn has_alphablend(&self, mat: &YKey, shd: &YKey) -> bool {
        let v = &self.alphablend;
        v.contains(&mat.to_string()) || v.contains(&shd.to_string())
    }

    pub fn has_invcoloralpha(&self, mat: &YKey, shd: &YKey) -> bool {
        let v = &self.invcoloralpha;
        v.contains(&mat.to_string()) || v.contains(&shd.to_string())
    }

    pub fn has_coloralpha(&self, mat: &YKey, shd: &YKey) -> bool {
        let v = &self.coloralpha;
        v.contains(&mat.to_string()) || v.contains(&shd.to_string())
    }

    pub fn has_skybox(&self, mat: &YKey, shd: &YKey) -> bool {
        let v = &self.skybox;
        v.contains(&mat.to_string()) || v.contains(&shd.to_string())
    }

    pub fn has_submarine(&self, mat: &YKey, shd: &YKey) -> bool {
        let v = &self.submarine;
        v.contains(&mat.to_string()) || v.contains(&shd.to_string())
    }

    pub fn has_standard(&self, mat: &YKey, shd: &YKey) -> bool {
        let v = &self.standard;
        v.contains(&mat.to_string()) || v.contains(&shd.to_string())
    }
}