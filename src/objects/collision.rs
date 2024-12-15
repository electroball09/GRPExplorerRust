use glam::Vec3;

use super::*;

#[derive(Default)]
pub struct CollisionObject {
    pub positions: Vec<Vec3>,
    pub indices: Vec<[u16; 4]>,
}

impl CollisionObject {
    pub fn bounding_box(&self) -> (Vec3, Vec3) {
        let mut min = Vec3::new(0.0, 0.0, 0.0);
        let mut max = Vec3::new(0.0, 0.0, 0.0);

        for pos in self.positions.iter() {
            for i in 0..3 {
                min[i] = f32::min(min[i], pos[i]);
                max[i] = f32::max(max[i], pos[i]);
            }
        }

        (min, max)
    }
}

impl ArchetypeImpl for CollisionObject {
    fn load_from_buf(&mut self, buf: &[u8]) -> Result<(), YetiIOError> {
        let mut cursor = Cursor::new(buf);

        let num_pos = cursor.read_u16::<LittleEndian>()?;
        for _ in 0..num_pos {
            self.positions.push(Vec3::new(
                cursor.read_f32::<LittleEndian>()?,
                cursor.read_f32::<LittleEndian>()?,
                cursor.read_f32::<LittleEndian>()?,
            ));
        }

        let num_ind = cursor.read_u16::<LittleEndian>()?;
        for _ in 0..num_ind {
            let b = [
                cursor.read_u16::<LittleEndian>()?,
                cursor.read_u16::<LittleEndian>()?,
                cursor.read_u16::<LittleEndian>()?,
                cursor.read_u16::<LittleEndian>()?,
            ];
            // if b[0] != 1 {
            //     return Err(format!("weird indices number? {}", b[0]).into());
            // }
            self.indices.push(b);
        }

        Ok(())
    }

    fn unload(&mut self) {
        *self = Default::default()
    }
}

#[derive(Default)]
pub struct CollisionObjectTable {
    pub num_collisions: u32,
}

impl ArchetypeImpl for CollisionObjectTable {
    fn load_from_buf(&mut self, buf: &[u8]) -> Result<(), YetiIOError> {
        self.num_collisions = Cursor::new(buf).read_u32::<LittleEndian>()?;

        Ok(())
    }

    fn unload(&mut self) {
        *self = Default::default()
    }
}