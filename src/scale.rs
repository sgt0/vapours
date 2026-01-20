//! Value scaling.

use num_traits::ToPrimitive;
use vapoursynth4_rs::{ColorFamily, SampleType};

use crate::{enums::ColorRange, generic::HoldsVideoFormat};

/// Scale a value from one bit depth to another.
///
/// # Panics
///
/// Will panic if the input value cannot fit in a [`f32`].
#[must_use]
#[allow(clippy::cast_precision_loss)]
pub fn scale_value<T, U, V>(
  value: T,
  format_in: &U,
  format_out: &V,
  range_in: Option<ColorRange>,
  range_out: Option<ColorRange>,
  scale_offsets: Option<bool>,
  chroma: Option<bool>,
) -> f32
where
  T: Copy + ToPrimitive,
  U: HoldsVideoFormat,
  V: HoldsVideoFormat,
{
  let scale_offsets = scale_offsets.unwrap_or(true);

  let is_input_rgb = format_in.color_family() == ColorFamily::RGB;
  let is_output_rgb = format_out.color_family() == ColorFamily::RGB;

  let range_in = range_in.unwrap_or(if is_input_rgb {
    ColorRange::Full
  } else {
    ColorRange::Limited
  });
  let range_out = range_out.unwrap_or(if is_output_rgb {
    ColorRange::Full
  } else {
    ColorRange::Limited
  });

  let mut out_value = value.to_f32().expect("value should fit in a f32");
  if format_in.depth() == format_out.depth()
    && range_in == range_out
    && format_in.sample_type() == format_out.sample_type()
  {
    return out_value;
  }

  let chroma = if is_input_rgb || is_output_rgb {
    false
  } else {
    chroma.unwrap_or(false)
  };

  let input_peak = format_in.peak_value(Some(chroma), Some(range_in));
  let input_lowest = format_in.lowest_value(Some(chroma), Some(range_in));
  let output_peak = format_out.peak_value(Some(chroma), Some(range_out));
  let output_lowest = format_out.lowest_value(Some(chroma), Some(range_out));

  if scale_offsets && format_in.sample_type() == SampleType::Integer {
    if chroma {
      out_value -= (128 << (format_in.depth() - 8)) as f32;
    } else if range_in == ColorRange::Limited {
      out_value -= (16 << (format_in.depth() - 8)) as f32;
    }
  }

  out_value *= (output_peak - output_lowest) / (input_peak - input_lowest);

  if scale_offsets && format_out.sample_type() == SampleType::Integer {
    if chroma {
      out_value += (128 << (format_out.depth() - 8)) as f32;
    } else if range_out == ColorRange::Limited {
      out_value += (16 << (format_out.depth() - 8)) as f32;
    }
  }

  if format_out.sample_type() == SampleType::Integer {
    out_value = out_value
      .round()
      .clamp(0.0, format_out.peak_value(None, Some(ColorRange::Full)));
  }

  out_value
}

#[cfg(test)]
mod tests {
  use approx::assert_relative_eq;
  use rstest::rstest;

  use crate::vs_enums::{GRAY10, GRAY8, YUV444PS};

  use super::*;

  #[rstest]
  #[case(0, 0.0)]
  #[case(24, 24.0)]
  #[case(64, 64.0)]
  #[case(255, 255.0)]
  fn test_scale_value_no_change(#[case] input: u32, #[case] expected: f32) {
    assert_relative_eq!(
      scale_value(input, &GRAY8, &GRAY8, None, None, None, None),
      expected
    );
  }

  #[rstest]
  #[case(0, 0.0)]
  #[case(24, 96.0)]
  #[case(64, 256.0)]
  #[case(255, 1020.0)]
  fn test_scale_value_to_10bit(#[case] input: u32, #[case] expected: f32) {
    assert_relative_eq!(
      scale_value(input, &GRAY8, &GRAY10, None, None, None, None),
      expected
    );
  }

  #[rstest]
  #[case(0, 0.0)]
  #[case(96, 24.0)]
  #[case(256, 64.0)]
  #[case(1020, 255.0)]
  fn test_scale_value_from_10bit(#[case] input: u32, #[case] expected: f32) {
    assert_relative_eq!(
      scale_value(input, &GRAY10, &GRAY8, None, None, None, None),
      expected
    );
  }

  #[rstest]
  #[case(0, -0.073_059_36)]
  #[case(24, 0.036_529_68)]
  #[case(64, 0.219_178_08)]
  #[case(255, 1.091_324_2)]
  fn test_scale_value_to_float(#[case] input: u32, #[case] expected: f32) {
    assert_relative_eq!(
      scale_value(input, &GRAY8, &YUV444PS, None, None, None, None),
      expected
    );
  }

  #[rstest]
  #[case(0.0, 16.0)]
  #[case(0.1, 38.0)]
  #[case(0.25, 71.0)]
  #[case(1.0, 235.0)]
  fn test_scale_value_from_float(#[case] input: f32, #[case] expected: f32) {
    assert_relative_eq!(
      scale_value(input, &YUV444PS, &GRAY8, None, None, None, None),
      expected
    );
  }

  #[rstest]
  #[case(0, 16.0)]
  #[case(24, 37.0)]
  #[case(64, 71.0)]
  #[case(255, 235.0)]
  fn test_scale_value_to_limited(#[case] input: u32, #[case] expected: f32) {
    assert_relative_eq!(
      scale_value(
        input,
        &GRAY8,
        &GRAY8,
        Some(ColorRange::Full),
        Some(ColorRange::Limited),
        None,
        None
      ),
      expected
    );
  }

  #[rstest]
  #[case(0, 0.0)]
  #[case(24, 9.0)]
  #[case(64, 56.0)]
  #[case(255, 255.0)]
  fn test_scale_value_from_limited(#[case] input: u32, #[case] expected: f32) {
    assert_relative_eq!(
      scale_value(
        input,
        &GRAY8,
        &GRAY8,
        Some(ColorRange::Limited),
        Some(ColorRange::Full),
        None,
        None
      ),
      expected
    );
  }
}
