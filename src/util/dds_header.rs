use std::io::{Read, Write};

use bitflags::bitflags;
use byteorder::{ReadBytesExt, LittleEndian, WriteBytesExt};

bitflags! {
    pub struct DdsFlags: u32 {
        const DDSD_CAPS        = 0x1;
        const DDSD_HEIGHT      = 0x2;
        const DDSD_WIDTH       = 0x4;
        const DDSD_PITCH       = 0x8;
        const DDSD_PIXELFORMAT = 0x1000;
        const DDSD_MIPMAPCOUNT = 0x20000;
        const DDSD_LINEARSIZE  = 0x80000;
        const DDSD_DEPTH       = 0x800000;
    }
}

impl Default for DdsFlags {
    fn default() -> Self {
        DdsFlags::empty() | DdsFlags::DDSD_CAPS | DdsFlags::DDSD_HEIGHT | DdsFlags::DDSD_WIDTH | DdsFlags::DDSD_PIXELFORMAT
    }
}

#[allow(non_snake_case)]
pub struct DdsHeader {
    pub dwMagic: u32,
    pub dwSize: u32,
    pub dwFlags: DdsFlags,
    pub dwHeight: u32,
    pub dwWidth: u32,
    pub dwPitchOrLinearSize: u32,
    pub dwDepth: u32,
    pub dwMipMapCount: u32,
    pub dwReserved: [u8; 44],
    pub ddspf: DdsPixelformat,
    pub dwCaps: u32,
    pub dwCaps2: u32,
    pub dwCaps3: u32,
    pub dwCaps4: u32,
    pub dwReserved2: u32,
}

impl Default for DdsHeader {
    fn default() -> Self {
        Self {
            dwMagic: u32::from_le_bytes([b'D', b'D', b'S', b' ']),
            dwSize: 124,
            dwFlags: DdsFlags::default(),
            dwHeight: 0,
            dwWidth: 0,
            dwPitchOrLinearSize: 0,
            dwDepth: 0,
            dwMipMapCount: 0,
            dwReserved: [0; 44],
            ddspf: DdsPixelformat::default(),
            dwCaps: 0x1000,
            dwCaps2: 0,
            dwCaps3: 0,
            dwCaps4: 0,
            dwReserved2: 0
        }
    }
}

impl DdsHeader {
    pub fn dxt1(height: u32, width: u32) -> Self {
        Self {
            dwHeight: height,
            dwWidth: width,
            ddspf: DdsPixelformat::dxt1(),
            ..Default::default()
        }
    }

    pub fn dxt5(height: u32, width: u32) -> Self {
        Self {
            dwHeight: height,
            dwWidth: width,
            ddspf: DdsPixelformat::dxt5(),
            ..Default::default()
        }
    }

    pub fn read_from(reader: &mut impl Read) -> Result<Self, std::io::Error> {
        fn read_reserved(reader: &mut impl Read) -> [u8; 44] {
            let mut buf: [u8; 44] = [0; 44];
            let mut i = 0;
            while i < 11 {
                reader.read(&mut buf[i * 4..i * 4 + 4]).unwrap();
                i += 1;
            }
            buf
        }

        Ok(Self {
            dwMagic: reader.read_u32::<LittleEndian>()?,
            dwSize: reader.read_u32::<LittleEndian>()?,
            dwFlags: DdsFlags::from_bits(reader.read_u32::<LittleEndian>()?).unwrap(),
            dwHeight: reader.read_u32::<LittleEndian>()?,
            dwWidth: reader.read_u32::<LittleEndian>()?,
            dwPitchOrLinearSize: reader.read_u32::<LittleEndian>()?,
            dwDepth: reader.read_u32::<LittleEndian>()?,
            dwMipMapCount: reader.read_u32::<LittleEndian>()?,
            dwReserved: read_reserved(reader),
            ddspf: DdsPixelformat::read_from(reader)?,
            dwCaps: reader.read_u32::<LittleEndian>()?,
            dwCaps2: reader.read_u32::<LittleEndian>()?,
            dwCaps3: reader.read_u32::<LittleEndian>()?,
            dwCaps4: reader.read_u32::<LittleEndian>()?,
            dwReserved2: reader.read_u32::<LittleEndian>()?
        })
    }

    pub fn write_to(&self, writer: &mut impl Write) -> Result<(), std::io::Error> {
        writer.write_u32::<LittleEndian>(self.dwMagic)?;
        writer.write_u32::<LittleEndian>(self.dwSize)?;
        writer.write_u32::<LittleEndian>(self.dwFlags.bits)?;
        writer.write_u32::<LittleEndian>(self.dwHeight)?;
        writer.write_u32::<LittleEndian>(self.dwWidth)?;
        writer.write_u32::<LittleEndian>(self.dwPitchOrLinearSize)?;
        writer.write_u32::<LittleEndian>(self.dwDepth)?;
        writer.write_u32::<LittleEndian>(self.dwMipMapCount)?;
        writer.write(&self.dwReserved)?;
        self.ddspf.write_to(writer)?;
        writer.write_u32::<LittleEndian>(self.dwCaps)?;
        writer.write_u32::<LittleEndian>(self.dwCaps2)?;
        writer.write_u32::<LittleEndian>(self.dwCaps3)?;
        writer.write_u32::<LittleEndian>(self.dwCaps4)?;
        writer.write_u32::<LittleEndian>(self.dwReserved2)?;
        Ok(())
    }
}

bitflags! {
    pub struct DdsPfFlags: u32 {
        const DDPF_ALPHAPIXELS = 0x1;
        const DDPF_ALPHA = 0x2;
        const DDPF_FOURCC = 0x4;
        const DDPF_RGB = 0x40;
        const DDPF_YUV = 0x200;
        const DDPF_LUMINANCE = 0x20000;
    }
}

impl Default for DdsPfFlags {
    fn default() -> Self {
        DdsPfFlags::empty() | DdsPfFlags::DDPF_FOURCC
    }
}

#[allow(non_snake_case)]
pub struct DdsPixelformat {
    pub dwSize: u32,
    pub dwFlags: DdsPfFlags,
    pub dwFourCC: u32,
    pub dwRGBBitCount: u32,
    pub dwRBitMask: u32,
    pub dwGBitMask: u32,
    pub dwBBitMask: u32,
    pub dwABitMask: u32,
}

impl Default for DdsPixelformat {
    fn default() -> Self {
        Self {
            dwSize: 32,
            dwFlags: DdsPfFlags::default(),
            dwFourCC: 0,
            dwRGBBitCount: 0,
            dwRBitMask: 0,
            dwGBitMask: 0,
            dwBBitMask: 0,
            dwABitMask: 0
        }
    }
}

impl DdsPixelformat {
    pub fn dxt1() -> Self {
        Self {
            dwFourCC: u32::from_le_bytes([b'D', b'X', b'T', b'1']),
            dwRGBBitCount: 24,
            dwRBitMask: 0xFF000000,
            dwGBitMask: 0x00FF0000,
            dwBBitMask: 0x0000FF00,
            dwABitMask: 0x00000000,
            ..Default::default()
        }
    }

    pub fn dxt5() -> Self {
        Self {
            dwFourCC: u32::from_le_bytes([b'D', b'X', b'T', b'5']),
            dwRGBBitCount: 32,
            dwRBitMask: 0xFF000000,
            dwGBitMask: 0x00FF0000,
            dwBBitMask: 0x0000FF00,
            dwABitMask: 0x000000FF,
            ..Default::default()
        }
    }

    pub fn read_from(reader: &mut impl Read) -> Result<Self, std::io::Error> {
        Ok(Self {
            dwSize: reader.read_u32::<LittleEndian>()?,
            dwFlags: DdsPfFlags::from_bits(reader.read_u32::<LittleEndian>()?).unwrap(),
            dwFourCC: reader.read_u32::<LittleEndian>()?,
            dwRGBBitCount: reader.read_u32::<LittleEndian>()?,
            dwRBitMask: reader.read_u32::<LittleEndian>()?,
            dwGBitMask: reader.read_u32::<LittleEndian>()?,
            dwBBitMask: reader.read_u32::<LittleEndian>()?,
            dwABitMask: reader.read_u32::<LittleEndian>()?
        })
    }

    pub fn write_to(&self, writer: &mut impl Write) -> Result<(), std::io::Error> {
        writer.write_u32::<LittleEndian>(self.dwSize)?;
        writer.write_u32::<LittleEndian>(self.dwFlags.bits)?;
        writer.write_u32::<LittleEndian>(self.dwFourCC)?;
        writer.write_u32::<LittleEndian>(self.dwRGBBitCount)?;
        writer.write_u32::<LittleEndian>(self.dwRBitMask)?;
        writer.write_u32::<LittleEndian>(self.dwGBitMask)?;
        writer.write_u32::<LittleEndian>(self.dwBBitMask)?;
        writer.write_u32::<LittleEndian>(self.dwABitMask)?;
        Ok(())
    }
}