use std::io::Cursor;
use byteorder::{LittleEndian, ReadBytesExt};
use crate::util::read_nul_term_string;

use super::{ArchetypeImpl, LoadError};

#[derive(Default)]
pub struct DataTable {
    pub columns: Vec<DataTableColumn>,
    pub rows: Vec<DataTableRow>,
}

#[derive(Default, Debug)]
pub struct DataTableColumn {
    pub name: String,
    pub data: ColumnData,
}

#[derive(Default, Debug)]
pub struct DataTableRow {
    pub data: Vec<ColumnData>,
}

#[derive(Debug)]
pub enum ColumnData {
    Int(i32),
    Float(f32),
    String(String),
    Asset(u32)
}

impl Default for ColumnData {
    fn default() -> Self {
        Self::Int(0)
    }
}

impl std::fmt::Display for ColumnData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            ColumnData::Int(v) => write!(f, "{}", v),
            ColumnData::Float(v) => write!(f, "{}", v),
            ColumnData::String(ref v) => write!(f, "{}", v),
            ColumnData::Asset(v) => write!(f, "{:#010X}", v)
        }
    }
}

impl ArchetypeImpl for DataTable {
    fn load_from_buf(&mut self, buf: &[u8]) -> Result<(), LoadError> {
        let mut cursor = Cursor::new(buf);

        let num_cols = cursor.read_i32::<LittleEndian>()?;
        let num_rows = cursor.read_i32::<LittleEndian>()?;
        
        let mut idx = 0;
        while idx < num_cols {
            let col = DataTableColumn {
                name: read_nul_term_string(&mut cursor)?,
                data: match cursor.read_i32::<LittleEndian>()? {
                    1 => ColumnData::Int(cursor.read_i32::<LittleEndian>()?),
                    2 => ColumnData::Float(cursor.read_f32::<LittleEndian>()?),
                    3 => ColumnData::String(read_nul_term_string(&mut cursor)?),
                    4 => ColumnData::Asset(cursor.read_u32::<LittleEndian>()?),
                    v => {
                        return Err(format!("unknown datatable column type: {}", v).into());
                    }
                }
            };
            self.columns.push(col);
            idx += 1;
        };

        for _ in 0..num_rows {
            let mut row = DataTableRow::default();
            for col in &self.columns {
                let value = match col.data {
                    ColumnData::Int(_)      => ColumnData::Int(cursor.read_i32::<LittleEndian>()?),
                    ColumnData::Float(_)    => ColumnData::Float(cursor.read_f32::<LittleEndian>()?),
                    ColumnData::String(_)   => ColumnData::String(read_nul_term_string(&mut cursor)?),
                    ColumnData::Asset(_)    => ColumnData::Asset(cursor.read_u32::<LittleEndian>()?),
                };

                row.data.push(value);
            }
            self.rows.push(row);
        };

        Ok(())
    }

    fn unload(&mut self) {
        *self = Self::default();
    }
}