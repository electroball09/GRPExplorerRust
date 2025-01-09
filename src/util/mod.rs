use std::io::Read;
use byteorder::ReadBytesExt;
use glam::{Mat4, Vec4};

pub mod dds_header;
pub mod log_config;
pub mod load_util;
pub mod texture_util;

pub fn read_nul_term_string(rdr: &mut impl Read) -> std::io::Result<String> {
    let mut s = String::new();
    let mut b: u8 = rdr.read_u8()?;
    while b != 0 {
        s.push(b.into());
        b = rdr.read_u8()?;
    }
    Ok(s)
}

pub fn transform_yeti_matrix(mat: &Mat4) -> Mat4 {
    // https://stackoverflow.com/questions/1263072/changing-a-matrix-from-right-handed-to-left-handed-coordinate-system
    const TOGGLE_MATRIX: glam::Mat4 = Mat4 {
        x_axis: Vec4::new(-1.0, 0.0, 0.0, 0.0), // flip the x axis
        y_axis: Vec4::new(0.0, 0.0, 1.0, 0.0), // swap y and z axis
        z_axis: Vec4::new(0.0, 1.0, 0.0, 0.0), // swap y and z axis
        w_axis: Vec4::new(0.0, 0.0, 0.0, 1.0),
    };
    TOGGLE_MATRIX * *mat * TOGGLE_MATRIX // this switches y and z coords and flips x coord, same as in exp_mesh.rs
}