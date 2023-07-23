use std::io::Error as IOError;

#[derive(Debug)]
pub enum EmuErr {
    ReadRom(IOError),
    UnrecognizedOpCode(u16),
    UnrecognizedAddressingMode(u16),
    UnrecognizedCondBranchFlag(u16),
}
