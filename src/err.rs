use std::io::Error as IOError;

#[derive(Debug)]
pub enum EmuErr {
    ReadRom(IOError),
    InvalidRom,
    UnsupportedMapperType,
    UnrecognizedOpCode(u16),
}
