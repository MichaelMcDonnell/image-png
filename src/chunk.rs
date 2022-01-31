//! Chunk types and functions
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
use core::fmt;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChunkType(pub [u8; 4]);

// -- Critical chunks --

/// Image header
pub const IHDR: ChunkType = ChunkType([b'I', b'H', b'D', b'R']);
/// Palette
pub const PLTE: ChunkType = ChunkType([b'P', b'L', b'T', b'E']);
/// Image data
pub const IDAT: ChunkType = ChunkType([b'I', b'D', b'A', b'T']);
/// Image trailer
pub const IEND: ChunkType = ChunkType([b'I', b'E', b'N', b'D']);

// -- Ancillary chunks --

/// Transparency
pub const tRNS: ChunkType = ChunkType([b't', b'R', b'N', b'S']);
/// Background colour
pub const bKGD: ChunkType = ChunkType([b'b', b'K', b'G', b'D']);
/// Image last-modification time
pub const tIME: ChunkType = ChunkType([b't', b'I', b'M', b'E']);
/// Physical pixel dimensions
pub const pHYs: ChunkType = ChunkType([b'p', b'H', b'Y', b's']);
/// Source system's pixel chromaticities
pub const cHRM: ChunkType = ChunkType([b'c', b'H', b'R', b'M']);
/// Source system's gamma value
pub const gAMA: ChunkType = ChunkType([b'g', b'A', b'M', b'A']);
/// sRGB color space chunk
pub const sRGB: ChunkType = ChunkType([b's', b'R', b'G', b'B']);
/// ICC profile chunk
pub const iCCP: ChunkType = ChunkType([b'i', b'C', b'C', b'P']);
/// Latin-1 uncompressed textual data
pub const tEXt: ChunkType = ChunkType([b't', b'E', b'X', b't']);
/// Latin-1 compressed textual data
pub const zTXt: ChunkType = ChunkType([b'z', b'T', b'X', b't']);
/// UTF-8 textual data
pub const iTXt: ChunkType = ChunkType([b'i', b'T', b'X', b't']);

// -- Extension chunks --

/// Animation control
pub const acTL: ChunkType = ChunkType([b'a', b'c', b'T', b'L']);
/// Frame control
pub const fcTL: ChunkType = ChunkType([b'f', b'c', b'T', b'L']);
/// Frame data
pub const fdAT: ChunkType = ChunkType([b'f', b'd', b'A', b'T']);

// -- Chunk type determination --

/// Returns true if the chunk is critical.
pub fn is_critical(ChunkType(type_): ChunkType) -> bool {
    type_[0] & 32 == 0
}

/// Returns true if the chunk is private.
pub fn is_private(ChunkType(type_): ChunkType) -> bool {
    type_[1] & 32 != 0
}

/// Checks whether the reserved bit of the chunk name is set.
/// If it is set the chunk name is invalid.
pub fn reserved_set(ChunkType(type_): ChunkType) -> bool {
    type_[2] & 32 != 0
}

/// Returns true if the chunk is safe to copy if unknown.
pub fn safe_to_copy(ChunkType(type_): ChunkType) -> bool {
    type_[3] & 32 != 0
}

impl fmt::Debug for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        struct DebugType([u8; 4]);

        impl fmt::Debug for DebugType {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                for &c in &self.0[..] {
                    write!(f, "{}", char::from(c).escape_debug())?;
                }
                Ok(())
            }
        }

        f.debug_struct("ChunkType")
            .field("type", &DebugType(self.0))
            .field("critical", &is_critical(*self))
            .field("private", &is_private(*self))
            .field("reserved", &reserved_set(*self))
            .field("safecopy", &safe_to_copy(*self))
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const CRITICAL_CHUNKS: [ChunkType; 4] = [IHDR, PLTE, IDAT, IEND];
    const ANCILLARY_CHUNKS: [ChunkType; 11] = [
        tRNS, bKGD, tIME, pHYs, cHRM, gAMA, sRGB, iCCP, tEXt, zTXt, iTXt,
    ];
    const EXTENSION_CHUNKS: [ChunkType; 3] = [acTL, fcTL, fdAT];

    #[test]
    fn test_is_critical() {
        for type_ in CRITICAL_CHUNKS {
            assert!(is_critical(type_));
        }
        for type_ in ANCILLARY_CHUNKS {
            assert!(!is_critical(type_));
        }
        for type_ in EXTENSION_CHUNKS {
            assert!(!is_critical(type_));
        }
    }

    #[test]
    fn test_is_private() {
        for type_ in CRITICAL_CHUNKS {
            assert!(!is_private(type_));
        }
        for type_ in ANCILLARY_CHUNKS {
            assert!(!is_private(type_));
        }
        // The extension chunks are private and not part of PNG specification
        for type_ in EXTENSION_CHUNKS {
            assert!(is_private(type_));
        }
    }

    #[test]
    fn test_reserved_set() {
        // The reserved bit is reserved for later use so it should not be set
        // for the known types.
        for type_ in CRITICAL_CHUNKS {
            assert!(!reserved_set(type_));
        }
        for type_ in ANCILLARY_CHUNKS {
            assert!(!reserved_set(type_));
        }
        for type_ in EXTENSION_CHUNKS {
            assert!(!reserved_set(type_));
        }
    }

    #[test]
    fn test_safe_to_copy() {
        for type_ in CRITICAL_CHUNKS {
            assert!(!safe_to_copy(type_));
        }
        // Only some of the ancillary chunks are safe to copy
        for type_ in [tRNS, bKGD, tIME, cHRM, gAMA, sRGB, iCCP] {
            assert!(!safe_to_copy(type_));
        }
        for type_ in [pHYs, tEXt, zTXt, iTXt] {
            assert!(safe_to_copy(type_));
        }
        for type_ in EXTENSION_CHUNKS {
            assert!(!safe_to_copy(type_));
        }
    }

    #[test]
    fn test_debug_fmt() {
        let s = format!("{:?}", IHDR);
        assert_eq!(s, "ChunkType { type: IHDR, critical: true, private: false, reserved: false, safecopy: false }");
    }
}
