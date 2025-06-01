//! Enums.

/// Pixel range ([ITU-T H.265](https://www.itu.int/rec/T-REC-H.265) Equations
/// E-10 through E-20).
#[derive(Clone, Copy, Debug, Eq, FromPrimitive, PartialEq)]
pub enum ColorRange {
  /// Studio (TV) legal range, 16-235 in 8 bits. This is primarily used with
  /// YUV integer formats.
  Limited = 1,

  /// Full (PC) dynamic range, 0-255 in 8 bits. Note that float clips should
  /// ALWAYS be FULL range! RGB clips will ALWAYS be FULL range!
  Full = 0,
}

#[cfg(test)]
mod tests {
  use num_traits::FromPrimitive;

  use super::*;

  #[test]
  fn test_from_primitive() {
    assert_eq!(ColorRange::from_u8(0), Some(ColorRange::Full));
    assert_eq!(ColorRange::from_u8(1), Some(ColorRange::Limited));
    assert_eq!(ColorRange::from_u8(2), None);
  }
}
