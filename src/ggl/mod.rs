use std::{collections::HashMap, sync::Arc};

use crate::*;

fn load_sources(map: &mut HashMap<String, String>) {
    include!(concat!(env!("OUT_DIR"), "/shadergen.rs"))
}

struct ParsedShader {
    vert: Option<String>,
    frag: Option<String>
}

impl ParsedShader {
    pub fn from_source(source: &String) -> Self {
        let mut vert = "".to_string();
        let mut frag = "".to_string();

        enum Stage {
            None, Vert, Frag
        }

        let mut stage = Stage::None;
        
        for line in source.lines() {
            match line {
                "##vert" => stage = Stage::Vert,
                "##frag" => stage = Stage::Frag,
                _ => {
                    match stage {
                        Stage::Vert => vert = format!("{}\r\n{}", vert, line),
                        Stage::Frag => frag = format!("{}\r\n{}", vert, line),
                        _ => {
                            warn!("unrecognized identifier: {:?}", line)
                        }
                    }
                }
            }
        }

        let vert = match vert.len() {
            0 => None,
            _ => Some(vert)
        };

        let frag = match frag.len() {
            0 => None,
            _ => Some(frag)
        };

        ParsedShader {
            vert,
            frag
        }
    }
}

pub struct ShaderCache {
    parsed_shaders: HashMap<String, ParsedShader>,
}

impl ShaderCache {
    pub fn new() -> Self {
        let mut sources = HashMap::<String, String>::new();
        load_sources(&mut sources);

        let parsed_shaders: HashMap<String, ParsedShader> = sources.iter().map(|x| {
            info!("Parsed shader {}", &x.0);
            (x.0.clone(), ParsedShader::from_source(x.1))
        }).collect();

        Self {
            parsed_shaders,
        }
    }

    pub unsafe fn init(&mut self, gl: Arc<glow::Context>) {
        info!("Shader cache is initializing");


    }
}