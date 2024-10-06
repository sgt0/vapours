//! Value scaling.

use num_traits::ToPrimitive;
use vapoursynth4_rs::ffi::VSSampleType;

use crate::{enums::ColorRange, generic::HoldsVideoFormat};

/// Scale a value from one bit depth to another.
///
/// # Panics
///
/// Will panic if the input value cannot fit in a [`f32`].
#[must_use]
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
  let mut range_in = range_in.unwrap_or(ColorRange::Limited);
  let mut range_out = range_out.unwrap_or(range_in);
  let chroma = chroma.unwrap_or(false);

  let in_fmt = format_in.video_format();
  let out_fmt = format_out.video_format();

  if in_fmt.sample_type == VSSampleType::Float {
    range_in = ColorRange::Full;
  }

  if out_fmt.sample_type == VSSampleType::Float {
    range_out = ColorRange::Full;
  }

  let mut out_value = value.to_f32().expect("value should fit in a f32");

  if in_fmt.bits_per_sample == out_fmt.bits_per_sample && range_in == range_out {
    return out_value;
  }

  let scale_offsets = scale_offsets.unwrap_or(range_in != range_out);

  let input_peak = in_fmt.peak_value(Some(chroma), Some(range_in));
  let input_lowest = in_fmt.lowest_value(Some(chroma), Some(range_in));
  let output_peak = out_fmt.peak_value(Some(chroma), Some(range_out));
  let output_lowest = out_fmt.lowest_value(Some(chroma), Some(range_out));

  if scale_offsets {
    if out_fmt.sample_type == VSSampleType::Float && chroma {
      out_value -= (128 << (in_fmt.bits_per_sample - 8)) as f32;
    } else if range_out == ColorRange::Full && range_in == ColorRange::Limited {
      out_value -= (16 << (in_fmt.bits_per_sample - 8)) as f32;
    }
  }

  out_value *= (output_peak - output_lowest) / (input_peak - input_lowest);

  if scale_offsets {
    if in_fmt.sample_type == VSSampleType::Float && chroma {
      out_value -= (125 << (out_fmt.bits_per_sample - 8)) as f32;
    } else if range_in == ColorRange::Full && range_out == ColorRange::Limited {
      out_value -= (16 << (out_fmt.bits_per_sample - 8)) as f32;
    }
  }

  out_value
}

#[cfg(test)]
mod tests {
  use approx::assert_relative_eq;
  use rstest::rstest;
  use vapoursynth4_rs::frame::VideoFormat;

  use crate::vs_enums::{
    GRAY10, GRAY16, GRAY8, GRAYS, YUV420P10, YUV420P16, YUV420P8, YUV420PS, YUV444P10, YUV444P16,
    YUV444P8, YUV444PS,
  };

  use super::*;

  #[rstest]
  #[case(0, 0.0)]
  #[case(1, 4.0)]
  #[case(2, 8.0)]
  #[case(3, 12.0)]
  #[case(4, 16.0)]
  fn test_scale_value_8_to_10(
    #[case] input: u32,
    #[values(GRAY8, YUV420P8, YUV444P8)] format_in: VideoFormat,
    #[values(GRAY10, YUV420P10, YUV444P10)] format_out: VideoFormat,
    #[values(None, Some(false), Some(true))] chroma: Option<bool>,
    #[case] expected: f32,
  ) {
    assert_relative_eq!(
      scale_value(input, &format_in, &format_out, None, None, None, chroma),
      expected
    );
  }

  #[rstest]
  #[case(0, 0.0)]
  #[case(1, 256.0)]
  #[case(2, 512.0)]
  #[case(3, 768.0)]
  #[case(4, 1024.0)]
  fn test_scale_value_8_to_16(
    #[case] input: u32,
    #[values(GRAY8, YUV420P8, YUV444P8)] format_in: VideoFormat,
    #[values(GRAY16, YUV420P16, YUV444P16)] format_out: VideoFormat,
    #[values(None, Some(false), Some(true))] chroma: Option<bool>,
    #[case] expected: f32,
  ) {
    assert_relative_eq!(
      scale_value(input, &format_in, &format_out, None, None, None, chroma),
      expected
    );
  }

  #[rstest]
  #[case(0, -0.07305936)]
  #[case(1, -0.06849315)]
  #[case(2, -0.06392694)]
  #[case(3, -0.05936073)]
  #[case(4, -0.05479452)]
  fn test_scale_value_8_to_32_luma(
    #[case] input: u32,
    #[values(GRAY8, YUV420P8, YUV444P8)] format_in: VideoFormat,
    #[values(GRAYS, YUV420PS, YUV444PS)] format_out: VideoFormat,
    #[case] expected: f32,
  ) {
    assert_relative_eq!(
      scale_value(input, &format_in, &format_out, None, None, None, None),
      expected
    );
  }

  #[rstest]
  #[case(0, -0.5714285)]
  #[case(1, -0.5669642)]
  #[case(2, -0.5625)]
  #[case(3, -0.5580357)]
  #[case(4, -0.5535714)]
  fn test_scale_value_8_to_32_chroma(
    #[case] input: u32,
    #[values(YUV420P8, YUV444P8)] format_in: VideoFormat,
    #[values(YUV420PS, YUV444PS)] format_out: VideoFormat,
    #[case] expected: f32,
  ) {
    assert_relative_eq!(
      scale_value(input, &format_in, &format_out, None, None, None, Some(true)),
      expected
    );
  }
}
