use std::ffi::CStr;
use std::path::Path;

use anyhow::Result;

use elf::ElfBytes;
use elf::endian::AnyEndian;
use elf::note::{Note, NoteAny};
use elf::section::SectionHeader;

#[repr(C, packed)]
#[derive(Debug)]
struct AvrDeviceInfoDesc {
    flash_start: u32,
    flash_size: u32,
    sram_start: u32, 
    sram_size: u32, 
    eeprom_start: u32, 
    eeprom_size: u32, 
    offset_table_size: u32,
    offset_table: [u32; 1],
    strtab: [u8; 8]
}


pub fn device_name_from_binary(binary: impl AsRef<Path>) -> Result<String> {
    let file_data = std::fs::read(binary)?;
    let slice = file_data.as_slice();
    let file = ElfBytes::<AnyEndian>::minimal_parse(slice)?;

    let avr_device_info_header: SectionHeader = file.section_header_by_name(".note.gnu.avr.deviceinfo")?
        .ok_or_else(|| anyhow::anyhow!("AVR device info section not found"))?;

    let device_info_note: NoteAny = file
        .section_data_as_notes(&avr_device_info_header)?
        .map(|note| match note {
            Note::Unknown(note) => if note.n_type == 1 && note.name == "AVR" { Some(note) } else {None},
            _ => None
        })
        .flatten()
        .nth(0).ok_or_else(|| anyhow::anyhow!("AVR device info note not found"))?;

    

    let device_info = unsafe { &*(device_info_note.desc.as_ptr() as *mut AvrDeviceInfoDesc) };
    let device_name = unsafe{CStr::from_ptr((&device_info.strtab as *const u8 as *const i8).offset(device_info.offset_table[0] as isize))};

    Ok(device_name.to_str()?.to_string())
}