
use std::collections::HashMap;
use serde::Deserialize;


#[derive(Deserialize, Debug)]
pub struct ExportConfig {
    pub capture_ids: HashMap<String, CaptureWay>,
    pub spawn_zone_ids: HashMap<String, SpawnZoneWay>,
    pub capture_visual_scripts: HashMap<String, CaptureVisualScript>
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