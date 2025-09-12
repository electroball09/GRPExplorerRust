use std::fmt::{Display, UpperHex};
#[allow(dead_code)]

use std::io::Read;
use byteorder::{ReadBytesExt, LittleEndian};
use chrono::{DateTime, Utc};
use enum_as_inner::EnumAsInner;
use strum::{AsRefStr, EnumIter, FromRepr};
use strum_macros::EnumString;

use crate::YetiIOError;

#[allow(unused)]
#[derive(Debug, Default)]
pub struct SegmentHeader {
    pub sig: [u8; 4],
    pub unk01: u8, // always 1
    pub num_segments: u8,
    pub segment: u8,
    pub unk02: u8, // always 0
    pub unk_seg_offset01: u64,
    pub header_offset: u64,
    pub prev_total_data_len: u64,
    pub total_data_len: u64,
    pub last_update: u64,
}

impl Display for SegmentHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,
        "sig: {} | unk01: {:#04X} | num_segments: {:#04X} | segment: {:#04X} | unk02: {:#04X} | unk_seg_offset01: {:#10X} | header_offset: {:#10X} | prev_total_data_len: {:#10X} | total_data_len: {:#10X} | last_update: {:#10X}", 
        self.sig_to_str(), self.unk01, self.num_segments, self.segment, self.unk02, self.unk_seg_offset01, self.header_offset, self.prev_total_data_len, self.total_data_len, self.last_update)
    }
}

impl SegmentHeader {
    pub fn read_from(read: &mut impl Read) -> Result<SegmentHeader, YetiIOError> {
        let mut header = SegmentHeader::default();

        if read.read(&mut header.sig).expect("error reading signature!") != 4 {
            return Err("could not read header correctly!".into());
        }

        header.unk01 = read.read_u8().unwrap();
        header.num_segments = read.read_u8().unwrap();
        header.segment = read.read_u8().unwrap();
        header.unk02 = read.read_u8().unwrap();
        header.unk_seg_offset01 = read.read_u64::<LittleEndian>().unwrap();
        header.header_offset = read.read_u64::<LittleEndian>().unwrap();
        header.prev_total_data_len = read.read_u64::<LittleEndian>().unwrap();
        header.total_data_len = read.read_u64::<LittleEndian>().unwrap();
        header.last_update = read.read_u64::<LittleEndian>().unwrap();

        header.verify_integrity()?;

        Ok(header)
    }

    pub fn sig_to_str(&self) -> &str {
        std::str::from_utf8(&self.sig).unwrap()
    }

    pub fn verify_integrity(&self) -> Result<(), &'static str> {
        if self.sig[0] != b'Y'
            || self.sig[1] != b'B'
            || self.sig[2] != b'I'
            || self.sig[3] != b'G' {
            return Err("signature invalid!");
        }

        if self.num_segments == 0 || self.segment >= self.num_segments {
            return Err("segment counts invalid!");
        }

        if self.header_offset < 48 {
            return Err("header offset too small! (<48)");
        }

        if self.header_offset > 163840 {
            return Err("header offset too large! (>163840)");
        }

        Ok(())
    }
}

#[derive(FromRepr, Debug, Clone, Copy, PartialEq, PartialOrd)]
#[repr(u16)]
pub enum BigfileVersion {
    NONE = 0x0,
    GRO = 0x86,
    GRFS = 0x87,
}

#[derive(Debug)]
pub struct BigfileHeader {
    pub version: BigfileVersion,
    pub num_folders: u16,
    pub num_files: u32,
    pub unk_01: [u8; 72],
    pub load_priority: u32,
    pub auto_activate: bool, 
    pub unk_02: [u8; 3],
    pub data_root: [u8; 40]
}

impl Default for BigfileHeader {
    fn default() -> Self {
        Self {
            version: BigfileVersion::NONE,
            num_folders: 0,
            num_files: 0,
            unk_01: [0; 72],
            load_priority: 0,
            auto_activate: false,
            unk_02: [0; 3],
            data_root: [0; 40]
        }
    }
}

impl Display for BigfileHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,
        "version: {:?} | num_folders: {:#06X} | num_files: {:#010X} | load_prio: {} | auto_active: {} | data_root: {}",
        self.version, self.num_folders, self.num_files, self.load_priority, self.auto_activate, self.data_root_str())
    }
}

impl BigfileHeader {
    pub fn read_from(reader: &mut impl Read) -> Result<BigfileHeader, String> {
        let mut header = BigfileHeader::default();
        header.version = BigfileVersion::from_repr(reader.read_u16::<LittleEndian>().unwrap()).unwrap();
        header.num_folders = reader.read_u16::<LittleEndian>().unwrap();
        header.num_files = reader.read_u32::<LittleEndian>().unwrap();
        reader.read(&mut header.unk_01).unwrap();
        header.load_priority = reader.read_u32::<LittleEndian>().unwrap();
        header.auto_activate = reader.read_u8().unwrap() != 0;
        reader.read(&mut header.unk_02).unwrap();
        reader.read(&mut header.data_root).unwrap();
        
        Ok(header)
    }

    pub fn data_root_str(&self) -> &str {
        let idx = self.data_root.iter().position(|b| *b == 0).unwrap();
        std::str::from_utf8(&self.data_root[..idx]).unwrap()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct FolderEntry {
    pub idx: u16,

    pub unk01: u16,
    pub unk02: u16,
    pub unk03: u16,
    pub unk04: u16,
    pub parent_folder: u16,
    pub first_child: u16,
    pub next_folder: u16,
    pub name: [u8; 50]
}

impl Default for FolderEntry {
    fn default() -> Self {
        FolderEntry {
            idx: 0,
            unk01: 0,
            unk02: 0,
            unk03: 0,
            unk04: 0,
            parent_folder: 0,
            first_child: 0,
            next_folder: 0,
            name: [0; 50]
        }
    }
}

impl Display for FolderEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,
        "index: {:#06X} | unk01: {:#06X} | unk02: {:#06X} | unk03: {:#06X} | unk04: {:#06X} | parent: {:#06X} | first_child: {:#06X} | next: {:#06X} | name: {}",
        self.idx, self.unk01, self.unk02, self.unk03, self.unk04, self.parent_folder, self.first_child, self.next_folder, self.get_name())
    }
}

impl FolderEntry {
    pub fn struct_size(version: BigfileVersion) -> usize {
        match version {
            BigfileVersion::GRFS => 64,
            BigfileVersion::GRO => 64,
            BigfileVersion::NONE => panic!("wtf (this is spaghetti code don't mind me)")
        }
    }

    pub fn read_from(reader: &mut impl Read, version: BigfileVersion) -> Result<FolderEntry, String> {
        let mut entry = FolderEntry::default();
        entry.unk01 = reader.read_u16::<LittleEndian>().unwrap();
        entry.unk02 = reader.read_u16::<LittleEndian>().unwrap();
        if version == BigfileVersion::GRO {
            entry.unk03 = reader.read_u16::<LittleEndian>().unwrap();
            entry.unk04 = reader.read_u16::<LittleEndian>().unwrap();
        }
        entry.parent_folder = reader.read_u16::<LittleEndian>().unwrap();
        entry.first_child = reader.read_u16::<LittleEndian>().unwrap();
        entry.next_folder = reader.read_u16::<LittleEndian>().unwrap();
        reader.read(&mut entry.name).unwrap();
        if version == BigfileVersion::GRFS {
            entry.unk03 = reader.read_u16::<LittleEndian>().unwrap();
            entry.unk04 = reader.read_u16::<LittleEndian>().unwrap();
        }

        Ok(entry)
    }

    pub fn get_name(&self) -> &str {
        let idx = self.name.iter().position(|b| *b == 0).unwrap();
        std::str::from_utf8(&self.name[..idx]).unwrap()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct YKey(u32);

impl Default for YKey {
    fn default() -> Self {
        Self(0xFFFFFFFF)
    }
}

impl Display for YKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#010X}", self.0)
    }
}

impl From<u32> for YKey {
    fn from(value: u32) -> Self {
        YKey(value)
    }
}

impl From<YKey> for u32 {
    fn from(value: YKey) -> Self {
        value.0
    }
}

impl UpperHex for YKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        UpperHex::fmt(&self.0, f)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct FileEntry {
    pub offset: u32,
    pub key: YKey,
    pub unk01: i32,
    pub object_type: ObjectType,
    pub parent_folder: u16,
    pub timestamp: DateTime<Utc>,
    pub flags: i32,
    pub unk02: i32,
    pub crc: [u8; 4],
    pub name: [u8; 60],
    pub unk03: i32,
    pub zip: bool,

    tmp_name_buf: [u8; 64]
}

impl Default for FileEntry {
    fn default() -> Self {
        FileEntry {
            offset: 0,
            key: 0.into(),
            unk01: 0,
            object_type: ObjectType::null,
            parent_folder: 0,
            timestamp: Default::default(),
            flags: 0,
            unk02: 0,
            crc: [0; 4],
            name: [0; 60],
            unk03: 0,
            zip: false,

            tmp_name_buf: [0; 64]
        }
    }
}

impl Display for FileEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,
        "offset: {:#010X} | key: {:#010X} | unk01: {:#010X} | type: {:?} | parent_folder: {} | timestamp: {} | flags: {:#010X} | unk02: {:#010X} | unk03: {:#010X} | zip: {} | name: {}",
        self.offset, self.key, self.unk01, self.object_type, self.parent_folder, self.timestamp, self.flags, self.unk02, self.unk03, self.zip, self.get_name())
    }
}

impl FileEntry {
    pub fn struct_size(version: BigfileVersion) -> usize {
        match version {
            BigfileVersion::GRFS => 96,
            BigfileVersion::GRO => 100,
            BigfileVersion::NONE => panic!("wtf")
        }
    }

    pub fn read_from(reader: &mut impl Read, version: BigfileVersion) -> Result<FileEntry, String> {
        let mut entry = FileEntry::default();
        entry.offset = reader.read_u32::<LittleEndian>().unwrap();
        entry.key = reader.read_u32::<LittleEndian>().unwrap().into();
        entry.unk01 = reader.read_i32::<LittleEndian>().unwrap();
        entry.object_type = ObjectType::from_repr(reader.read_u16::<LittleEndian>().unwrap()).unwrap();
        entry.parent_folder = reader.read_u16::<LittleEndian>().unwrap();
        entry.timestamp = DateTime::from_timestamp(reader.read_i32::<LittleEndian>().unwrap() as i64, 0).unwrap(); //NaiveDateTime::from_timestamp(reader.read_i32::<LittleEndian>().unwrap() as i64, 0).unwrap();
        entry.flags = reader.read_i32::<LittleEndian>().unwrap();
        entry.unk02 = reader.read_i32::<LittleEndian>().unwrap();
        reader.read(&mut entry.crc).unwrap();
        reader.read(&mut entry.name).unwrap();
        entry.unk03 = reader.read_i32::<LittleEndian>().unwrap();
        if version == BigfileVersion::GRO {
            entry.zip = reader.read_i32::<LittleEndian>().unwrap() == 1;
        }

        entry.tmp_name_buf[..60].clone_from_slice(&entry.name[..]);
        let idx = entry.tmp_name_buf.iter().position(|b| *b == 0).unwrap();
        let ext = format!("{:?}", entry.object_type);
        entry.tmp_name_buf[idx] = b'.';
        let ext_bytes = ext.as_bytes();
        entry.tmp_name_buf[idx + 1..idx + ext_bytes.len() + 1].copy_from_slice(&ext_bytes);

        Ok(entry)
    }

    pub fn get_name(&self) -> &str {
        let idx = self.name.iter().position(|b| *b == 0).unwrap();
        std::str::from_utf8(&self.name[..idx]).unwrap()
    }

    pub fn get_name_ext(&self) -> &str {
        let idx = self.tmp_name_buf.iter().position(|b| *b == 0).unwrap();
        std::str::from_utf8(&self.tmp_name_buf[..idx]).unwrap()
    }
}

#[allow(non_camel_case_types)]
#[derive(FromRepr, Debug, Default, Clone, Copy, EnumString, EnumIter, EnumAsInner, PartialEq, Eq, AsRefStr, Hash)]
#[repr(u16)]
pub enum ObjectType {
    #[default]
    null = 0,
    ini = 0x0001, //yeti - ini file
    t_0x2 = 0x0002,
    dup = 0x0003, //duplicate files
    wor = 0x0004, //world - world
    wot = 0x0005,
    woc = 0x0006, //world - engine config
    gol = 0x0007, //world - game object list
    ioi = 0x0008,
    wal = 0x0009, //world - way list
    lay = 0x000A,
    rec = 0x000B,
    rsy = 0x000C,
    gao = 0x000D, //object - game object
    way = 0x000E, //way - way
    nas = 0x000F,
    cur = 0x0010, //curve - curve
    wel = 0x0011, //way - exernal link
    gtm = 0x0012,
    seq = 0x0013, //sequence - sequence
    cov = 0x0014, //cover -  covers manager
    vgl = 0x0015,
    vgc = 0x0016,
    vgg = 0x0017,
    ssq = 0x0018, //sequence - ???
    got = 0x0019, //object - graphic object table
    t_0x1A = 0x001A,
    msh = 0x001B, //visual - mesh metadata
    vxc = 0x001C, //visual - something about vertex buffer ???
    vxt = 0x001D,
    mat = 0x001E, //visual - material/shader
    sha = 0x001F, //visual - shader
    tga = 0x0020, //visual - texture metadata
    txs = 0x0021,
    t_0x22 = 0x0022,
    ske = 0x0023,
    sfx = 0x0024,
    vid = 0x0025,
    shd = 0x0026, //visual - visual shader (probably a shader graph or something)
    she = 0x0027,
    dst = 0x0028, //visual - dustfx
    cub = 0x0029, //visual - cubemap light
    zc  = 0x002A, //ai - script file
    tes = 0x002B,
    tsc = 0x002C,
    mgm = 0x002D,
    mgb = 0x002E,
    mft = 0x002F,
    acb = 0x0030, //animation - action bank
    act = 0x0031, //animation - action
    ani = 0x0032, //animation - animation
    aev = 0x0033, //animation - animation events
    snk = 0x0034, //sound - sound bank
    t_0x35 = 0x0035,
    t_0x36 = 0x0036,
    t_0x37 = 0x0037,
    t_0x38 = 0x0038,
    t_0x39 = 0x0039,
    t_0x3A = 0x003A,
    t_0x3B = 0x003B,
    end = 0x003C, //?? - enumerable descriptor
    sam = 0x003D, //sound - ambience
    sin = 0x003E, //sound - config
    smx = 0x003F, //sound - sound mix
    svs = 0x0040, //sound - volumetric object
    t_0x41 = 0x0041,
    t_0x42 = 0x0042,
    t_0x43 = 0x0043,
    t_0x44 = 0x0044,
    t_0x45 = 0x0045,
    t_0x46 = 0x0046,
    ai  = 0x0047, //ai - ai model
    aid = 0x0048,
    ste = 0x0049,
    fcf = 0x004A,
    var = 0x004B,
    aiv = 0x004C, //ai - ai variable
    stt = 0x004D,
    zar = 0x004E,
    zon = 0x004F, //zone - zone
    col = 0x0050, //object - collision ?
    cot = 0x0051, //object - collision object table
    gml = 0x0052, //visual - game material list
    gmt = 0x0053, //visual - game material
    phs = 0x0054, //object - physics structure ?
    t_0x55 = 0x0055,
    ccm = 0x0056, //object - cooked collision mesh?
    dbk = 0x0057, //dynamic bank - bank
    dbl = 0x0058, //dynamic bank - bank list
    dbr = 0x0059, //dynamic bank - bank element reference list
    edi = 0x005A,
    hel = 0x005B,
    hsl = 0x005C,
    his = 0x005D,
    mta = 0x005E,
    wil = 0x005F, //world - world include list
    t_0x60 = 0x0060,
    ymf = 0x0061,
    t_0x62 = 0x0062,
    fbx = 0x0063,
    dds = 0x0064,
    png = 0x0065,
    bmp = 0x0066,
    jpg = 0x0067,
    ppm = 0x0068,
    grd = 0x0069,
    dlc = 0x006A, //dlc ?
    ymt = 0x006B,
    par = 0x006C,
    ard = 0x006D,
    med = 0x006E,
    lab = 0x006F, //animation - list action-bank
    feu = 0x0070, //fire - fire package
    ffd = 0x0071, //fire - fire font package
    top = 0x0072, //world - datastreaming entity topography
    msd = 0x0073, //visual - mesh data
    nav = 0x0074, //world - nav data
    skp = 0x0075, //skeleton - procedural data ?
    ago = 0x0076, //visual - dust fx ago ??
    afx = 0x0077, //visual - ago FX ???
    abk = 0x0078, //visual - ago FX bank
    cst = 0x0079, //ai - const list
    syw = 0x007A, //synapse - ???
    sym = 0x007B,
    aer = 0x007C,
    ghk = 0x007D, //game hook ?
    txd = 0x007E, //visual - texture data
    fbk = 0x007F, //fire - fire bank
    pfx = 0x0080,
    eps = 0x0081, //ai - dll editable param struct
    psh = 0x0082,
    epl = 0x0083, //ai - dll editable param list
    lgr = 0x0084,
    acp = 0x0085,
    ask = 0x0086,
    swl = 0x0087, //synapse - synapse world list
    ano = 0x0088, //animation - ano ???
    ann = 0x0089,
    rsf = 0x008A,
    led = 0x008B,
    shg = 0x008C, //??? shader - "SH_cl_List"
    cld = 0x008D, //cover - covers LD
    dtb = 0x008E, //data table - data table
    otf = 0x008F,
    ttf = 0x0090,
    adf = 0x0091,
    pco = 0x0092,
    t_0x93 = 0x0093,
    t_0x94 = 0x0094
}