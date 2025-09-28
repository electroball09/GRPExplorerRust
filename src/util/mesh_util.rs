use glam::{Vec3, Mat3, Quat, Vec4};


pub fn snorm16_to_float(v: i16) -> f32 {
    f32::max((v as f32) / 32767.0, -1.0)
}

pub fn snorm8_to_float(v: i8) -> f32 {
    f32::max((v as f32) / 127.0, -1.0)
}

pub fn uvi16_to_float(v: i16) -> f32 {
    f32::from(v) / 1024.0
}

pub fn snorm_10bit_to_float(v: u32) -> f32 {
    let v = ((v << 22) as i32) >> 22;
    f32::max((v as f32) / 512.0, -1.0)
}

pub fn bytes_to_qtangent_tnb(b: &[u8; 8]) -> (Vec3, Vec3, Vec3) {
    let x = i16::from_le_bytes([b[0], b[1]]);
    let y = i16::from_le_bytes([b[2], b[3]]);
    let z = i16::from_le_bytes([b[4], b[5]]);
    let w = i16::from_le_bytes([b[6], b[7]]);

    let snorm = Vec4::new(
        snorm16_to_float(x),
        snorm16_to_float(y),
        snorm16_to_float(z),
        snorm16_to_float(w)
    );
    
    let quat = Quat::from_xyzw(snorm.x, snorm.y, snorm.z, snorm.w);
    let mat = Mat3::from_quat(quat);

    (mat.col(0), mat.col(2), mat.col(1))
}

pub fn bytes_to_short4n_tnb(b: &[u8; 8]) -> (Vec3, Vec3, Vec3) {
    let x1 =i16::from_le_bytes([b[0], b[1]]);
    let y1 =i16::from_le_bytes([b[2], b[3]]);
    let x2 =i16::from_le_bytes([b[4], b[5]]);
    let y2 =i16::from_le_bytes([b[6], b[7]]);

    let x1 = snorm16_to_float(x1);
    let y1 = snorm16_to_float(y1);
    let x2 = snorm16_to_float(x2);
    let y2 = snorm16_to_float(y2);

    let v1 = Vec3::new(x1, y1, 1.0 - (f32::abs(x1) + f32::abs(y1)));
    let v2 = Vec3::new(x2, y2, 1.0 - (f32::abs(x2) + f32::abs(y2)));
    let v3 = Vec3::cross(v1, v2); // tangent x normal ??

    return (v1, v2, v3);
}

pub fn bytes_to_rgb10_a2_tnb_with_a(b: &[u8; 8]) -> (Vec4, Vec4, Vec4) {
    let a1 = u32::from_le_bytes([b[0], b[1], b[2], b[3]]);
    let a2 = u32::from_le_bytes([b[4], b[5], b[6], b[7]]);

    fn u32_to_rgb10a2(v: u32) -> (Vec3, f32) {
        let x = v & 0x3FF;
        let y = (v >> 10) & 0x3FF;
        let z = (v >> 20) & 0x3FF;
        let w = (v >> 30) & 0x3;

        let v = Vec3::new(
            snorm_10bit_to_float(x),
            snorm_10bit_to_float(y),
            snorm_10bit_to_float(z),
        );

        (v, w as f32)
    }

    let (v1, a1) = u32_to_rgb10a2(a1);
    let (v2, a2) = u32_to_rgb10a2(a2);

    let v3 = Vec3::cross(v1, v2);
    (
        Vec4::new(v1.x, v1.y, v1.z, a1),
        Vec4::new(v2.x, v2.y, v2.z, a2),
        Vec4::new(v3.x, v3.y, v3.z, 0.0)
    )
}