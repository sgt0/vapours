//! More Rust equivalents to VapourSynth enums.

use seq_macro::seq;
use vapoursynth4_rs::{frame::VideoFormat, ColorFamily, SampleType};

const fn make_video_format(
  color_family: ColorFamily,
  sample_type: SampleType,
  bits_per_sample: i32,
  sub_sampling_w: i32,
  sub_sampling_h: i32,
) -> VideoFormat {
  let mut bytes_per_sample = 1;
  while bytes_per_sample * 8 < bits_per_sample {
    bytes_per_sample *= 2;
  }

  VideoFormat {
    color_family,
    sample_type,
    bits_per_sample,
    bytes_per_sample,
    sub_sampling_w,
    sub_sampling_h,
    num_planes: match color_family {
      ColorFamily::Gray => 1,
      ColorFamily::YUV | ColorFamily::RGB | ColorFamily::Undefined => 3,
    },
  }
}

const GRAY: ColorFamily = ColorFamily::Gray;
const RGB: ColorFamily = ColorFamily::RGB;
const YUV: ColorFamily = ColorFamily::YUV;
const INTEGER: SampleType = SampleType::Integer;
const FLOAT: SampleType = SampleType::Float;

seq!(N in 8..=32 {
  #[doc=concat!("GRAY color family, ", N, " bits per sample.")]
  pub const GRAY~N: VideoFormat = make_video_format(GRAY, INTEGER, N, 0, 0);
});

pub const GRAYH: VideoFormat = make_video_format(GRAY, FLOAT, 16, 0, 0);
pub const GRAYS: VideoFormat = make_video_format(GRAY, FLOAT, 32, 0, 0);

seq!(N in 8..=32 {
  #[doc=concat!("YUV color family, 4:2:0 subsampling, ", N, " bits per sample.")]
  pub const YUV420P~N: VideoFormat = make_video_format(YUV, INTEGER, N, 1, 1);
});

pub const YUV420PH: VideoFormat = make_video_format(YUV, FLOAT, 16, 1, 1);
pub const YUV420PS: VideoFormat = make_video_format(YUV, FLOAT, 32, 1, 1);

seq!(N in 8..=32 {
  #[doc=concat!("YUV color family, 4:4:4 subsampling, ", N, " bits per sample.")]
  pub const YUV444P~N: VideoFormat = make_video_format(YUV, INTEGER, N, 0, 0);
});

pub const YUV444PH: VideoFormat = make_video_format(YUV, FLOAT, 16, 0, 0);
pub const YUV444PS: VideoFormat = make_video_format(YUV, FLOAT, 32, 0, 0);

pub const RGB24: VideoFormat = make_video_format(RGB, INTEGER, 8, 0, 0);
pub const RGBH: VideoFormat = make_video_format(RGB, FLOAT, 16, 0, 0);
pub const RGBS: VideoFormat = make_video_format(RGB, FLOAT, 32, 0, 0);

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_preset_video_formats() {
    assert_eq!(GRAY8.color_family, GRAY);
    assert_eq!(GRAY8.sample_type, INTEGER);
    assert_eq!(GRAY8.bits_per_sample, 8);
    assert_eq!(GRAY8.bytes_per_sample, 1);

    assert_eq!(YUV420P8.color_family, YUV);
    assert_eq!(YUV420P8.sample_type, INTEGER);
    assert_eq!(YUV420P8.bits_per_sample, 8);
    assert_eq!(YUV420P8.bytes_per_sample, 1);

    assert_eq!(YUV420P16.color_family, YUV);
    assert_eq!(YUV420P16.sample_type, INTEGER);
    assert_eq!(YUV420P16.bits_per_sample, 16);
    assert_eq!(YUV420P16.bytes_per_sample, 2);

    assert_eq!(YUV444PS.color_family, YUV);
    assert_eq!(YUV444PS.sample_type, FLOAT);
    assert_eq!(YUV444PS.bits_per_sample, 32);
    assert_eq!(YUV444PS.bytes_per_sample, 4);
  }
}
