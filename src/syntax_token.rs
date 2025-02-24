/*
    Marty PC Emulator 
    (C)2023 Daniel Balsom
    https://github.com/dbalsom/marty

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.

    syntax_token.rs

    Defines token enums for visual formatting of debugging output 
    including disassembly and memory views. A corresponding egui control
    TokenListView can use these tokens to format output with syntax coloring.
*/

/// A generic enum type that can hold values that are intended to update a 
/// Debug display. A Dirty variant can hold a value that can be dirty or not
/// DirtyAging8 adds an u8 frame age parameter. 
/// DirtyAgging adds a u16 frame age parameter.
/// Aging8 has a u8 frame age parameter.
/// Aging16 has a u16 frame age parameter.

pub const TOKEN_MAX_AGE: u8 = 255;

pub trait SyntaxTokenize {
    fn tokenize(&self) -> Vec<SyntaxToken>;
}

#[derive(Clone)]
pub enum SyntaxToken {

    NullToken,
    // Generic display tokens

    // State string has a 'dirty' flag for displaying state data as new, and a 
    // u8 frame age counter for tracking age of value.
    StateString(String, bool, u8), 

    // Memory viewer tokens
    ErrorString(String),
    MemoryAddressSeg16(u16, u16, String),
    MemoryAddressFlat(u32, String),
    MemoryByteHexValue(u32, u8, String, bool, u8),
    MemoryByteAsciiValue(u32, u8, String, u8),

    // Disassembly tokens
    ErrorText(String),
    InstructionBytes(String),
    Prefix(String),
    Mnemonic(String),
    Text(String),
    Segment(String),
    Colon,
    Comma,
    PlusSign,
    OpenBracket,
    CloseBracket,
    HexValue(String),
    Register(String),
    Displacement(String),
}

impl Default for SyntaxToken {
    fn default() -> Self { SyntaxToken::NullToken }
}
