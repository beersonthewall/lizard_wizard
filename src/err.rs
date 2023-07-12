#[derive(Debug)]
pub enum EmuErr {
    UnrecognizedOpCode(u16),
    UnrecognizedAddressingMode(u16),
    UnrecognizedCondBranchFlag(u16),
}
