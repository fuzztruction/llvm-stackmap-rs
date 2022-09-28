use std::{
    convert::{TryFrom, TryInto},
    io::{self},
    mem::size_of,
    path::Path,
};

use bytes::{Buf, Bytes};
#[cfg(feature = "serde-support")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "from-elf")]
use {goblin::elf, goblin::elf::Elf, std::fs, std::ops::Range};

type Constant = u64;

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde-support", derive(Serialize, Deserialize))]
pub enum LocationType {
    /// Invalid location
    Invalid = 0,
    /// Value is in dwarf_regnum
    Register = 1,
    /// dwarf_regnum + offset_or_constant is the ptr to the value
    Direct = 2,
    /// Ptr to the value is at [dwarf_regnum + offset_or_constant]
    Indirect = 3,
    /// Value is in offset_or_constant
    Constant = 4,
    /// Value is Constants[offset_or_constant]
    /// Constants is an array that is part of the Stackmap.
    ConstIndex = 5,
}

/// By default, an Location is invalid.
impl Default for LocationType {
    fn default() -> Self {
        LocationType::Invalid
    }
}

impl TryFrom<u8> for LocationType {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            _ if value == LocationType::Register as u8 => Ok(LocationType::Register),
            _ if value == LocationType::Direct as u8 => Ok(LocationType::Direct),
            _ if value == LocationType::Indirect as u8 => Ok(LocationType::Indirect),
            _ if value == LocationType::Constant as u8 => Ok(LocationType::Constant),
            _ if value == LocationType::ConstIndex as u8 => Ok(LocationType::ConstIndex),
            _ => Err(format!("Not a valid LocationType variant: {}", value)),
        }
    }
}

#[derive(Debug)]
pub enum ParsingError {
    Malformed(String),
    StackMapSectionNotFound,
    VersionNotSupported(u8),
    IoError(io::Error),
}

impl From<io::Error> for ParsingError {
    fn from(err: io::Error) -> Self {
        ParsingError::IoError(err)
    }
}

#[cfg(feature = "from-elf")]
impl From<goblin::error::Error> for ParsingError {
    fn from(err: goblin::error::Error) -> Self {
        match err {
            goblin::error::Error::IO(io_err) => ParsingError::IoError(io_err),
            err @ _ => ParsingError::Malformed(format!("Error while parsing ELF: {:?}", err)),
        }
    }
}

/// Drain bytes from a Vector and resturn an owned Self.
trait DrainFromBytes {
    /// This will drain size_of::<Self>() from `bytes` and return Self or an error
    /// if the `bytes` is too short.
    #[inline(always)]
    fn drain_from_bytes(bytes: &mut Bytes) -> Result<Self, ParsingError>
    where
        Self: Sized,
    {
        if bytes.len() < size_of::<Self>() {
            // We ran out of input
            return Err(ParsingError::Malformed(
                "Not enough bytes to construct value".to_owned(),
            ));
        }

        let ret = unsafe {
            Ok(std::ptr::read_unaligned(
                bytes.chunk().as_ptr() as *const Self
            ))
        };
        bytes.advance(size_of::<Self>());

        ret
    }
}

impl DrainFromBytes for u8 {}
impl DrainFromBytes for u16 {}
impl DrainFromBytes for u32 {}
impl DrainFromBytes for u64 {}
impl DrainFromBytes for i32 {}
impl DrainFromBytes for i64 {}

#[repr(C)]
#[derive(Default, Debug, Clone, Copy)]
#[cfg_attr(feature = "serde-support", derive(Serialize, Deserialize))]
struct Header {
    /// The LLVM Stackmap version of the following data.
    version: u8,
    reserved_0: u8,
    reserved_1: u16,
}

impl DrainFromBytes for Header {
    fn drain_from_bytes(bytes: &mut Bytes) -> Result<Self, ParsingError>
    where
        Self: Sized,
    {
        let version = u8::drain_from_bytes(bytes)?;
        if version != 3 {
            return Err(ParsingError::VersionNotSupported(version));
        }

        let reserved_0 = u8::drain_from_bytes(bytes)?;
        let reserved_1 = u16::drain_from_bytes(bytes)?;

        if reserved_0 != 0 || reserved_1 != 0 {
            return Err(ParsingError::Malformed(
                "Reversed bytes in header are not zero".to_owned(),
            ));
        }

        Ok(Header {
            version,
            reserved_0,
            reserved_1,
        })
    }
}

/// Describes one function of the binary the Stackmap belongs to.
#[repr(C)]
#[derive(Default, Debug, Clone, Copy)]
#[cfg_attr(feature = "serde-support", derive(Serialize, Deserialize))]
pub struct StkSizeRecord {
    /// VMA of this function. This address is relative to the sections base,
    /// if this is a PIC binary.
    pub function_address: u64,
    /// Number of bytes this function allocates on the stack.
    pub stack_size: u64,
    /// Number of StkMapRecords that belong to this function.
    /// The addresses given in the StkMapRecords are relative to the `function_address`.
    /// While iterating through the StkSizeRecords, one must keep the sum of all `record_count`
    /// values seen so far to get an index into the StkMapRecords list for a specific function.
    pub record_count: u64,
}

impl DrainFromBytes for StkSizeRecord {
    fn drain_from_bytes(bytes: &mut Bytes) -> Result<Self, ParsingError>
    where
        Self: Sized,
    {
        let function_address = u64::drain_from_bytes(bytes)?;
        let stack_size = u64::drain_from_bytes(bytes)?;
        let record_count = u64::drain_from_bytes(bytes)?;

        Ok(StkSizeRecord {
            function_address,
            stack_size,
            record_count,
        })
    }
}

#[repr(C)]
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde-support", derive(Serialize, Deserialize))]
pub struct Location {
    /// Describes how the values of the struct must be interpreted to get the
    /// actual recorded value.
    pub loc_type: LocationType,
    pub reserved_0: u8,
    /// The values size in bytes.
    pub loc_size: u16,
    /// Some register that is used according to loc_type.
    pub dwarf_regnum: u16,
    pub reserved_1: u16,
    /// Some offset or constant that is used according to loc_type.
    pub offset_or_constant: i32,
}

impl DrainFromBytes for Location {
    fn drain_from_bytes(bytes: &mut Bytes) -> Result<Self, ParsingError>
    where
        Self: Sized,
    {
        let loc_type = u8::drain_from_bytes(bytes)?
            .try_into()
            .map_err(|err| ParsingError::Malformed(err))?;
        let reserved_0 = u8::drain_from_bytes(bytes)?;
        let loc_size = u16::drain_from_bytes(bytes)?;
        let dwarf_regnum = u16::drain_from_bytes(bytes)?;
        let reserved_1 = u16::drain_from_bytes(bytes)?;
        let offset_or_constant = i32::drain_from_bytes(bytes)?;

        Ok(Location {
            loc_type,
            reserved_0,
            loc_size,
            dwarf_regnum,
            reserved_1,
            offset_or_constant,
        })
    }
}

/// A register that must stay alive (is not allowed to be trashed)
/// during execution of the PatchPoint. Whether a value must be manually saved
/// depends on the calling convention used for a given PatchPoint.
#[repr(C)]
#[derive(Default, Debug, Clone, Copy)]
#[cfg_attr(feature = "serde-support", derive(Serialize, Deserialize))]
pub struct LiveOut {
    /// The register that must stay live.
    dwarf_regnum: u16,
    reserved_0: u8,
    size: u8,
}
impl DrainFromBytes for LiveOut {
    fn drain_from_bytes(bytes: &mut Bytes) -> Result<Self, ParsingError>
    where
        Self: Sized,
    {
        let dwarf_regnum = u16::drain_from_bytes(bytes)?;
        let reserved_0 = u8::drain_from_bytes(bytes)?;
        if reserved_0 != 0 {
            return Err(ParsingError::Malformed(
                "LiveOut reserved field != 0".to_owned(),
            ));
        }

        let size = u8::drain_from_bytes(bytes)?;

        Ok(LiveOut {
            dwarf_regnum,
            reserved_0,
            size,
        })
    }
}

#[repr(C)]
#[derive(Default, Debug, Clone)]
#[cfg_attr(feature = "serde-support", derive(Serialize, Deserialize))]
pub struct StkMapRecord {
    /// Custom ID assigned during compilation.
    pub patch_point_id: u64,
    /// Offset from start of the function this record belongs to.
    pub instruction_offset: u32,
    pub reserved_0: u16,
    /// The number of locations this record contains.
    pub num_locations: u16,
    /// Location of the values this patch point was instructed to record.
    pub locations: Vec<Location>,
    // conditional_padding_0: u32,
    // conditional_padding_1: u16,
    /// The number of live outs in this record.
    pub num_live_outs: u16,
    /// The live out values.
    pub live_outs: Vec<LiveOut>,
    // conditional_padding_2: u32
}

#[repr(C)]
#[derive(Default, Debug, Clone)]
#[cfg_attr(feature = "serde-support", derive(Serialize, Deserialize))]
pub struct StackMap {
    /// The stackmap header.
    header: Header,
    /// Number of entries in stk_size_records.
    pub num_functions: u32,
    /// Number of entries in large_constants.
    pub num_constants: u32,
    /// Number of entries in stk_map_records.
    pub num_records: u32,
    /// One record for each function that contains patch points.
    pub stk_size_records: Vec<StkSizeRecord>,
    /// Used to store constants that where recorded by an Location.
    pub large_constants: Vec<Constant>,
    /// One record for each patch point.
    pub stk_map_records: Vec<StkMapRecord>,
}

impl StkMapRecord {
    fn new(data: &mut Bytes, stream_offset: &mut usize) -> Result<StkMapRecord, ParsingError> {
        let mut sm: StkMapRecord = StkMapRecord::default();
        let old_len = data.len();
        sm.patch_point_id = u64::drain_from_bytes(data)?;
        sm.instruction_offset = u32::drain_from_bytes(data)?;
        sm.reserved_0 = u16::drain_from_bytes(data)?;
        sm.num_locations = u16::drain_from_bytes(data)?;

        let tmp = (0..sm.num_locations)
            .map(|_| Location::drain_from_bytes(data))
            .collect::<Vec<_>>();
        // try does not work in closures.
        for l in tmp.into_iter() {
            sm.locations.push(l?);
        }

        *stream_offset += old_len - data.len();
        let old_len = data.len();
        // optional padding for alignment
        if (*stream_offset % 8) != 0 {
            u32::drain_from_bytes(data)?;
        }
        // padding
        u16::drain_from_bytes(data)?;

        sm.num_live_outs = u16::drain_from_bytes(data)?;
        let tmp = (0..sm.num_live_outs)
            .map(|_| LiveOut::drain_from_bytes(data))
            .collect::<Vec<_>>();
        // try does not work in closures.
        for lo in tmp.into_iter() {
            sm.live_outs.push(lo?);
        }

        *stream_offset += old_len - data.len();
        // optional padding for alignment
        if (*stream_offset % 8) != 0 {
            u32::drain_from_bytes(data)?;
            *stream_offset += 4;
        }

        Ok(sm)
    }
}

impl StackMap {
    /// Parse the given stackmap(s) contained in `data`. If multiple object files
    /// that contain a stackmap are linked, the corresponding stackmaps are concatinated.
    /// Thus, this function might return more than one stackmap.
    pub fn new(data: &mut Vec<u8>) -> Result<Vec<StackMap>, ParsingError> {
        let mut result = Vec::new();
        let mut data = Bytes::copy_from_slice(data);
        while !data.is_empty() {
            // The `parse` call will raise an error if parsing fails, hence this will
            // not end in a endless loop.
            let map = StackMap::parse(&mut data)?;
            result.push(map);
        }
        Ok(result)
    }

    /// Parse one stackmap contained in `data`. The passed `data` must be retrived
    /// from the .llvm_stackmaps section of a binary that was compiled with a llvm
    /// stackmap. Since a stackmap section possibly contains multiple stackmaps,
    /// `data.len()` might be != 0 after successfully parising a stackmap.
    fn parse(data: &mut Bytes) -> Result<StackMap, ParsingError> {
        let start_size = data.len();
        let mut stack_map: StackMap = StackMap::default();
        stack_map.header = Header::drain_from_bytes(data)?;
        stack_map.num_functions = u32::drain_from_bytes(data)?;
        stack_map.num_constants = u32::drain_from_bytes(data)?;
        stack_map.num_records = u32::drain_from_bytes(data)?;

        let tmp = (0..stack_map.num_functions)
            .map(|_| StkSizeRecord::drain_from_bytes(data))
            .collect::<Vec<_>>();
        for record in tmp.into_iter() {
            stack_map.stk_size_records.push(record?);
        }

        let tmp = (0..stack_map.num_constants)
            .map(|_| Constant::drain_from_bytes(data))
            .collect::<Vec<_>>();
        for constant in tmp.into_iter() {
            stack_map.large_constants.push(constant?);
        }

        let mut stream_offset = start_size - data.len();
        for _ in 0..stack_map.num_records {
            let record = StkMapRecord::new(data, &mut stream_offset)?;
            stack_map.stk_map_records.push(record);
        }

        Ok(stack_map)
    }

    /// Get the byte range of the binary that contains the bytes of section `section_name`.
    /// Thus `file_bytes[range.start..range.end]` yields the content of the section.
    /// If the ELF does not contain a section with the given `section_name`, None is returned.
    #[cfg(feature = "from-elf")]
    fn get_section_byte_range(elf: &Elf, section_name: &str) -> Option<Range<usize>> {
        let section_headers = &elf.section_headers;
        // Indices into the section header string table.
        let section_name_ids = section_headers
            .iter()
            .map(|section| section.sh_name)
            .collect::<Vec<_>>();

        for (idx, offset) in section_name_ids.into_iter().enumerate() {
            if elf
                .shdr_strtab
                .get_at(offset)
                .map_or(false, |e| e == section_name)
            {
                // Index must be valid since we generated section_name_ids by mapping
                // the entries of elf.section_headers.
                return section_headers.get(idx).unwrap().file_range();
            }
        }
        None
    }

    /// Relocates the function addresses contained in the stack map section.
    #[cfg(feature = "from-elf")]
    fn relocate_stackmap_section(
        elf: &Elf,
        stack_map_section_range: Range<usize>,
        stack_map_section: &mut [u8],
    ) -> Result<(), ParsingError> {
        let relas = &elf.dynrelas;
        for rela in relas.iter() {
            let file_offset = rela.r_offset as usize;

            // Skip relocs for other sections then the stack map.
            if !stack_map_section_range.contains(&file_offset) {
                continue;
            }

            let offset = file_offset - stack_map_section_range.start;
            match rela.r_type {
                elf::reloc::R_X86_64_RELATIVE => {
                    let val = rela.r_addend.unwrap().to_ne_bytes().to_vec();
                    stack_map_section[offset..(offset + 8)].copy_from_slice(&val);
                }
                elf::reloc::R_X86_64_64 => {
                    let dynsym_idx = rela.r_sym;
                    let sym = elf.dynsyms.get(dynsym_idx);
                    let sym_val = sym
                        .ok_or(ParsingError::Malformed(
                            "Failed to get symbol for relocation from dynsmy".to_owned(),
                        ))
                        .map(|s| s.st_value)?;
                    let sym_val = sym_val.to_ne_bytes().to_vec();
                    stack_map_section[offset..(offset + 8)].copy_from_slice(&sym_val);
                }
                _ => {
                    if stack_map_section_range.contains(&offset) {
                        todo!("Unsupported relocation for stack map: {:#?}", rela);
                    }
                }
            }
        }
        Ok(())
    }

    /// Check whether `path` points to a binary that contains a stackmap.
    /// This will also return false if the path does not exist.
    #[cfg(feature = "from-elf")]
    pub fn has_stackmap<T: AsRef<Path>>(path: T) -> bool {
        let bytes = fs::read(path);
        if let Ok(bytes) = bytes {
            let elf = Elf::parse(&bytes);
            return elf.map_or(false, |elf| {
                StackMap::get_section_byte_range(&elf, ".llvm_stackmaps").is_some()
            });
        }
        false
    }

    /// Parse the stackmap(s) of the binary `path` points to.
    #[cfg(feature = "from-elf")]
    pub fn from_path<T: AsRef<Path>>(path: T) -> Result<Vec<StackMap>, ParsingError> {
        let bytes = fs::read(path.as_ref())?;
        let elf = Elf::parse(&bytes)?;

        let stackmap_byte_range = StackMap::get_section_byte_range(&elf, ".llvm_stackmaps");
        if let Some(section_range) = stackmap_byte_range {
            let mut bytes = fs::read(path)?;
            let section_bytes = &mut bytes[section_range.clone()];
            StackMap::relocate_stackmap_section(&elf, section_range, section_bytes)?;
            return Ok(StackMap::new(&mut section_bytes.to_owned())?);
        }

        Err(ParsingError::StackMapSectionNotFound)
    }

    /// Pretty print the stackmap using the same notation as llvm-readobj --stackmap.
    pub fn pretty_print(&self) -> () {
        println!("LLVM StackMap Version: {}", self.header.version);
        println!("Num Functions: {}", self.num_functions);
        for f in &self.stk_size_records {
            println!(
                "  Function address: {}, stack size: {}, callsite record count: {}",
                f.function_address, f.stack_size, f.record_count
            );
        }
        println!("Num Constants: {}", self.num_constants);
        for (i, c) in self.large_constants.iter().enumerate() {
            println!("  #{}: {}", i + 1, c);
        }
        println!("Num Records: {}", self.num_records);
        for r in self.stk_map_records.iter() {
            println!(
                "  Record ID: {}, instruction offset: {}",
                r.patch_point_id, r.instruction_offset
            );
            println!("    {} locations:", r.num_locations);
            for (i, l) in r.locations.iter().enumerate() {
                let type_str = match l.loc_type {
                    LocationType::Register => format!("Register R#{}", l.dwarf_regnum as u32),
                    LocationType::Direct => format!(
                        "Direct R#{} + {}",
                        l.dwarf_regnum as u32, l.offset_or_constant
                    ),
                    LocationType::Indirect => format!(
                        "Indirect [ R#{} + {}]",
                        l.dwarf_regnum as u32, l.offset_or_constant
                    ),
                    LocationType::Constant => format!("Constant {}", l.offset_or_constant),
                    LocationType::ConstIndex => format!(
                        "ConstantIndex #{} ({})",
                        l.offset_or_constant, self.large_constants[l.offset_or_constant as usize]
                    ),
                    _ => {
                        unreachable!("Unexpected Location type");
                    }
                };
                println!("      #{}: {}, size: {}", i + 1, type_str, l.loc_size);
            }
            let live_out_str: String = r
                .live_outs
                .iter()
                .map(|lo| format!("R#{} ({}-bytes) ", lo.dwarf_regnum as u32, lo.size))
                .collect();
            println!("    {} live-outs: [ {}]", r.num_live_outs, live_out_str);
        }
    }
}
