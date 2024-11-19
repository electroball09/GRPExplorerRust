use std::{collections::HashMap, sync::Arc};

use glow::HasContext;

use crate::*;

fn load_sources(map: &mut HashMap<String, String>) {
    include!(concat!(env!("OUT_DIR"), "/shadergen.rs"))
}

struct AppShader {
    name: String,
    vert: Option<String>,
    frag: Option<String>,
    program: Option<glow::NativeProgram>,
}

impl AppShader {
    pub unsafe fn compile_shader(&mut self, gl: &glow::Context) {
        info!("compiling shader {}", &self.name);

        let program = gl.create_program().expect("could not create program!");

        let mut shaders = Vec::with_capacity(5);

        if let Some(source) = &self.vert {
            let vshader = gl.create_shader(glow::VERTEX_SHADER).expect("could not create vertex shader!");
            gl.shader_source(vshader, &source);
            gl.compile_shader(vshader);
            if !gl.get_shader_compile_status(vshader) {
                panic!("error compiling {}: {}", &self.name, gl.get_shader_info_log(vshader));
            }
            gl.attach_shader(program, vshader);
            shaders.push(vshader);
        }

        if let Some(source) = &self.frag {
            let fshader = gl.create_shader(glow::FRAGMENT_SHADER).expect("could not create fragment shader!");
            gl.shader_source(fshader, &source);
            gl.compile_shader(fshader);
            if !gl.get_shader_compile_status(fshader) {
                panic!("error compiling {}: {}", &self.name, gl.get_shader_info_log(fshader));
            }
            gl.attach_shader(program, fshader);
            shaders.push(fshader);
        }

        gl.link_program(program);
        if !gl.get_program_link_status(program) {
            panic!("error compiling {}: {}", &self.name, gl.get_program_info_log(program));
        }

        for shader in shaders {
            gl.detach_shader(program, shader);
            gl.delete_shader(shader);
        }

        self.program = Some(program);
    }

    pub fn from_source(name: String, source: &String) -> Self {
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
                        Stage::Frag => frag = format!("{}\r\n{}", frag, line),
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

        AppShader {
            name,
            vert,
            frag,
            program: None
        }
    }
}

pub struct ShaderCache {
    shaders: HashMap<String, AppShader>,
}

impl ShaderCache {
    pub fn new() -> Self {
        let mut sources = HashMap::<String, String>::new();
        load_sources(&mut sources);

        let shaders: HashMap<String, AppShader> = sources.iter().map(|x| {
            info!("Parsed shader {}", &x.0);
            (x.0.clone(), AppShader::from_source(x.0.clone(), x.1))
        }).collect();

        Self {
            shaders,
        }
    }

    pub unsafe fn init(&mut self, gl: Arc<glow::Context>) {
        info!("Shader cache is initializing");

        for shader in self.shaders.values_mut() {
            shader.compile_shader(&gl);
        }
    }
}