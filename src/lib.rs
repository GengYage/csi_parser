//! This is a simple CSI parser that only provides parsing for a subset of common CSI.
//! If you have additional requirements, feel free to submit a PR (Pull Request)
//!
//!
//! Please refer to the definition of [CSI](https://en.wikipedia.org/wiki/ANSI_escape_code#CSI_(Control_Sequence_Introducer)_sequences)
//!
//! See the [rs docs.](https://docs.rs/csi_parser/)
//! Look at progress and contribute on [github.](https://github.com/YageGeng/csi_parser)
pub mod parser;
pub mod enums;

// ESC is 0x1B
pub(crate) const CSI: &str = "\u{1B}[";
pub(crate) const SEPARATOR: &str = ";";

/// CSI final byte `0x40–0x7E`
/// [CSI](https://en.wikipedia.org/wiki/ANSI_escape_code#CSI_(Control_Sequence_Introducer)_sequences)
#[inline(always)]
pub fn terminated_byte(byte: u8) -> bool {
    (0x40..=0x7e).contains(&byte)
}

/// CSI parameter bytes `0–9:;<=>?`
#[inline(always)]
pub fn parameter_byte(byte: u8) -> bool {
    (0x30..=0x3f).contains(&byte)
}

/// CSI intermediate bytes `0x20–0x2F`
#[inline(always)]
pub fn intermediate_byte(byte: u8) -> bool {
    (0x20..=0x2f).contains(&byte)
}
