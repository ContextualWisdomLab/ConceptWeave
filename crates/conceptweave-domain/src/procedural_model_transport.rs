#![forbid(unsafe_code)]
#![deny(missing_docs)]
//! Private Rust transport-admission seam for procedural-model drafts.
//!
//! This module is intentionally not exported while the strict JSON contract is under
//! RED/GREEN development. Successful transport admission does not validate the
//! procedural-model schema, authenticate scope, or grant publication/execution authority.

use std::collections::BTreeSet;
use std::fmt;

/// Maximum accepted UTF-8 transport size before structural parsing.
pub const MAX_PROCEDURAL_TRANSPORT_BYTES: usize = 2 * 1024 * 1024;
/// Maximum nested JSON object/array depth admitted by the transport boundary.
pub const MAX_PROCEDURAL_TRANSPORT_DEPTH: usize = 128;

/// Fixed transport error codes; diagnostics never contain attacker-controlled JSON text.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProceduralTransportError {
    /// Input exceeded the bounded transport size.
    InputTooLarge,
    /// Input was not valid UTF-8 or strict JSON for this admission boundary.
    InvalidJson,
    /// An object repeated the same decoded member name.
    DuplicateMember,
    /// Structural nesting exceeded the parser ceiling.
    DepthLimit,
}

impl fmt::Display for ProceduralTransportError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let code = match self {
            Self::InputTooLarge => "input_too_large",
            Self::InvalidJson => "invalid_json",
            Self::DuplicateMember => "duplicate_member",
            Self::DepthLimit => "depth_limit",
        };
        formatter.write_str(code)
    }
}

impl std::error::Error for ProceduralTransportError {}

struct StrictJsonParser<'a> {
    input: &'a str,
    index: usize,
}

impl<'a> StrictJsonParser<'a> {
    fn new(input: &'a str) -> Self {
        Self { input, index: 0 }
    }

    fn finish(mut self) -> Result<(), ProceduralTransportError> {
        self.skip_whitespace();
        self.parse_value(0)?;
        self.skip_whitespace();
        if self.index == self.input.len() {
            Ok(())
        } else {
            Err(ProceduralTransportError::InvalidJson)
        }
    }

    fn parse_value(&mut self, depth: usize) -> Result<(), ProceduralTransportError> {
        self.skip_whitespace();
        match self.peek_byte() {
            Some(b'{') => self.parse_object(depth),
            Some(b'[') => self.parse_array(depth),
            Some(b'"') => self.parse_string(false).map(|_| ()),
            Some(b't') => self.parse_literal(b"true"),
            Some(b'f') => self.parse_literal(b"false"),
            Some(b'n') => self.parse_literal(b"null"),
            Some(b'-' | b'0'..=b'9') => self.parse_number(),
            _ => Err(ProceduralTransportError::InvalidJson),
        }
    }

    fn parse_object(&mut self, depth: usize) -> Result<(), ProceduralTransportError> {
        if depth >= MAX_PROCEDURAL_TRANSPORT_DEPTH {
            return Err(ProceduralTransportError::DepthLimit);
        }
        self.index += 1;
        self.skip_whitespace();
        if self.consume_if(b'}') {
            return Ok(());
        }

        let mut members = BTreeSet::new();
        loop {
            self.skip_whitespace();
            if self.peek_byte() != Some(b'"') {
                return Err(ProceduralTransportError::InvalidJson);
            }
            let member = self
                .parse_string(true)?
                .ok_or(ProceduralTransportError::InvalidJson)?;
            if !members.insert(member) {
                return Err(ProceduralTransportError::DuplicateMember);
            }
            self.skip_whitespace();
            if !self.consume_if(b':') {
                return Err(ProceduralTransportError::InvalidJson);
            }
            self.parse_value(depth + 1)?;
            self.skip_whitespace();
            if self.consume_if(b'}') {
                return Ok(());
            }
            if !self.consume_if(b',') {
                return Err(ProceduralTransportError::InvalidJson);
            }
        }
    }

    fn parse_array(&mut self, depth: usize) -> Result<(), ProceduralTransportError> {
        if depth >= MAX_PROCEDURAL_TRANSPORT_DEPTH {
            return Err(ProceduralTransportError::DepthLimit);
        }
        self.index += 1;
        self.skip_whitespace();
        if self.consume_if(b']') {
            return Ok(());
        }

        loop {
            self.parse_value(depth + 1)?;
            self.skip_whitespace();
            if self.consume_if(b']') {
                return Ok(());
            }
            if !self.consume_if(b',') {
                return Err(ProceduralTransportError::InvalidJson);
            }
        }
    }

    fn parse_string(&mut self, capture: bool) -> Result<Option<String>, ProceduralTransportError> {
        if !self.consume_if(b'"') {
            return Err(ProceduralTransportError::InvalidJson);
        }
        let mut output = if capture { Some(String::new()) } else { None };

        loop {
            let byte = self
                .peek_byte()
                .ok_or(ProceduralTransportError::InvalidJson)?;
            match byte {
                b'"' => {
                    self.index += 1;
                    return Ok(output);
                }
                b'\\' => {
                    self.index += 1;
                    let escape = self
                        .peek_byte()
                        .ok_or(ProceduralTransportError::InvalidJson)?;
                    self.index += 1;
                    let decoded = match escape {
                        b'"' => Some('"'),
                        b'\\' => Some('\\'),
                        b'/' => Some('/'),
                        b'b' => Some('\u{0008}'),
                        b'f' => Some('\u{000c}'),
                        b'n' => Some('\n'),
                        b'r' => Some('\r'),
                        b't' => Some('\t'),
                        b'u' => {
                            let first = self.parse_hex_quad()?;
                            if (0xd800..=0xdbff).contains(&first) {
                                if !self.consume_if(b'\\') || !self.consume_if(b'u') {
                                    return Err(ProceduralTransportError::InvalidJson);
                                }
                                let second = self.parse_hex_quad()?;
                                if !(0xdc00..=0xdfff).contains(&second) {
                                    return Err(ProceduralTransportError::InvalidJson);
                                }
                                let scalar = 0x1_0000
                                    + (((first as u32) - 0xd800) << 10)
                                    + ((second as u32) - 0xdc00);
                                Some(char::from_u32(scalar).ok_or(
                                    ProceduralTransportError::InvalidJson,
                                )?)
                            } else if (0xdc00..=0xdfff).contains(&first) {
                                return Err(ProceduralTransportError::InvalidJson);
                            } else {
                                Some(char::from_u32(first as u32).ok_or(
                                    ProceduralTransportError::InvalidJson,
                                )?)
                            }
                        }
                        _ => return Err(ProceduralTransportError::InvalidJson),
                    };
                    if let (Some(output), Some(decoded)) = (output.as_mut(), decoded) {
                        output.push(decoded);
                    }
                }
                0x00..=0x1f => return Err(ProceduralTransportError::InvalidJson),
                _ => {
                    let character = self.input[self.index..]
                        .chars()
                        .next()
                        .ok_or(ProceduralTransportError::InvalidJson)?;
                    self.index += character.len_utf8();
                    if let Some(output) = output.as_mut() {
                        output.push(character);
                    }
                }
            }
        }
    }

    fn parse_hex_quad(&mut self) -> Result<u16, ProceduralTransportError> {
        let mut value = 0u16;
        for _ in 0..4 {
            let byte = self
                .peek_byte()
                .ok_or(ProceduralTransportError::InvalidJson)?;
            self.index += 1;
            value = value
                .checked_mul(16)
                .and_then(|accumulator| hex_value(byte).map(|digit| accumulator + digit))
                .ok_or(ProceduralTransportError::InvalidJson)?;
        }
        Ok(value)
    }

    fn parse_literal(&mut self, literal: &[u8]) -> Result<(), ProceduralTransportError> {
        let remaining = &self.input.as_bytes()[self.index..];
        if remaining.starts_with(literal) {
            self.index += literal.len();
            Ok(())
        } else {
            Err(ProceduralTransportError::InvalidJson)
        }
    }

    fn parse_number(&mut self) -> Result<(), ProceduralTransportError> {
        self.consume_if(b'-');
        match self.peek_byte() {
            Some(b'0') => {
                self.index += 1;
                if matches!(self.peek_byte(), Some(b'0'..=b'9')) {
                    return Err(ProceduralTransportError::InvalidJson);
                }
            }
            Some(b'1'..=b'9') => {
                self.index += 1;
                self.consume_digits();
            }
            _ => return Err(ProceduralTransportError::InvalidJson),
        }

        if self.consume_if(b'.') {
            if !matches!(self.peek_byte(), Some(b'0'..=b'9')) {
                return Err(ProceduralTransportError::InvalidJson);
            }
            self.consume_digits();
        }

        if matches!(self.peek_byte(), Some(b'e' | b'E')) {
            self.index += 1;
            if matches!(self.peek_byte(), Some(b'+' | b'-')) {
                self.index += 1;
            }
            if !matches!(self.peek_byte(), Some(b'0'..=b'9')) {
                return Err(ProceduralTransportError::InvalidJson);
            }
            self.consume_digits();
        }
        Ok(())
    }

    fn consume_digits(&mut self) {
        while matches!(self.peek_byte(), Some(b'0'..=b'9')) {
            self.index += 1;
        }
    }

    fn skip_whitespace(&mut self) {
        while matches!(self.peek_byte(), Some(b' ' | b'\n' | b'\r' | b'\t')) {
            self.index += 1;
        }
    }

    fn consume_if(&mut self, expected: u8) -> bool {
        if self.peek_byte() == Some(expected) {
            self.index += 1;
            true
        } else {
            false
        }
    }

    fn peek_byte(&self) -> Option<u8> {
        self.input.as_bytes().get(self.index).copied()
    }
}

fn hex_value(byte: u8) -> Option<u16> {
    match byte {
        b'0'..=b'9' => Some((byte - b'0') as u16),
        b'a'..=b'f' => Some((byte - b'a' + 10) as u16),
        b'A'..=b'F' => Some((byte - b'A' + 10) as u16),
        _ => None,
    }
}

/// Admits one bounded UTF-8 JSON byte transport before schema/domain projection.
///
/// The byte ceiling is enforced before UTF-8 validation so callers do not need to
/// allocate or lossily decode an oversized payload before admission. Invalid UTF-8
/// maps to the fixed `invalid_json` diagnostic. Object-member uniqueness is evaluated
/// after JSON escape decoding, so differently encoded spellings of one logical member
/// fail closed. The parser validates JSON grammar only; Draft 2020-12 mapping and
/// authenticated scope admission are later gates.
pub fn admit_procedural_json_transport_bytes(
    input: &[u8],
) -> Result<(), ProceduralTransportError> {
    if input.len() > MAX_PROCEDURAL_TRANSPORT_BYTES {
        return Err(ProceduralTransportError::InputTooLarge);
    }
    let input = std::str::from_utf8(input).map_err(|_| ProceduralTransportError::InvalidJson)?;
    StrictJsonParser::new(input).finish()
}

/// Admits one already decoded UTF-8 JSON transport before schema/domain projection.
///
/// This convenience wrapper preserves the existing borrowed-string call surface while
/// routing through the byte-first transport boundary. Callers receiving raw network,
/// file or message bytes should use [`admit_procedural_json_transport_bytes`] directly.
pub fn admit_procedural_json_transport(input: &str) -> Result<(), ProceduralTransportError> {
    admit_procedural_json_transport_bytes(input.as_bytes())
}
