//! Traits that apply to multiple types.

use num_traits::ToPrimitive;
use vapoursynth4_rs::{
  frame::{VideoFormat, VideoFrame},
  node::VideoNode,
  ColorFamily, SampleType, VideoInfo,
};

use crate::enums::ColorRange;

/// A trait for types that hold a video format.
pub trait HoldsVideoFormat: Sized {
  /// Get the video format.
  #[must_use]
  fn video_format(&self) -> &VideoFormat;

  /// Get the color family of this clip or format.
  #[must_use]
  fn color_family(&self) -> ColorFamily {
    self.video_format().color_family
  }

  /// Get the bit depth of this clip or format.
  #[must_use]
  fn depth(&self) -> i32 {
    self.video_format().bits_per_sample
  }

  /// Get the sample type of this clip or format.
  #[must_use]
  fn sample_type(&self) -> SampleType {
    self.video_format().sample_type
  }

  /// Returns the lowest value for the bit depth of this clip or format.
  #[must_use]
  fn lowest_value(&self, chroma: Option<bool>, range_in: Option<ColorRange>) -> f32 {
    let is_rgb = self.color_family() == ColorFamily::RGB;
    let chroma = if is_rgb {
      false
    } else {
      chroma.unwrap_or(false)
    };

    if self.sample_type() == SampleType::Float {
      return if chroma { -0.5 } else { 0.0 };
    }

    let range_in = range_in.unwrap_or(if is_rgb {
      ColorRange::Full
    } else {
      ColorRange::Limited
    });

    if range_in == ColorRange::Limited {
      return (16 << (self.depth() - 8))
        .to_f32()
        .expect("result should fit in a f32");
    }

    0.0
  }

  /// Returns the midpoint value for the bit depth of this clip or format.
  #[must_use]
  fn neutral_value(&self) -> f32 {
    if self.sample_type() == SampleType::Float {
      return 0.0;
    }

    (1 << (self.depth() - 1))
      .to_f32()
      .expect("result should fit in a f32")
  }

  /// Returns the peak value for the bit depth of this clip or format.
  #[must_use]
  fn peak_value(&self, chroma: Option<bool>, range_in: Option<ColorRange>) -> f32 {
    let is_rgb = self.color_family() == ColorFamily::RGB;
    let chroma = if is_rgb {
      false
    } else {
      chroma.unwrap_or(false)
    };

    if self.sample_type() == SampleType::Float {
      return if chroma { 0.5 } else { 1.0 };
    }

    let range_in = range_in.unwrap_or(if is_rgb {
      ColorRange::Full
    } else {
      ColorRange::Limited
    });

    if range_in == ColorRange::Limited {
      return (if chroma { 240 } else { 235 } << (self.depth() - 8))
        .to_f32()
        .expect("result should fit in a f32");
    }

    ((1 << self.depth()) - 1)
      .to_f32()
      .expect("result should fit in a f32")
  }
}

impl HoldsVideoFormat for VideoFrame {
  fn video_format(&self) -> &VideoFormat {
    self.get_video_format()
  }
}

impl HoldsVideoFormat for VideoFormat {
  fn video_format(&self) -> &VideoFormat {
    self
  }
}

impl HoldsVideoFormat for VideoInfo {
  fn video_format(&self) -> &VideoFormat {
    &self.format
  }
}

impl HoldsVideoFormat for VideoNode {
  fn video_format(&self) -> &VideoFormat {
    self.info().video_format()
  }
}

#[cfg(test)]
mod tests {
  use approx::assert_relative_eq;
  use rstest::rstest;
  use vapoursynth4_rs::frame::VideoFormat;

  use crate::vs_enums::{
    GRAY16, GRAY8, GRAYH, GRAYS, RGB24, RGBH, RGBS, YUV420P16, YUV420P8, YUV420PS, YUV444P16,
    YUV444P8, YUV444PS,
  };

  use super::*;

  #[rstest]
  #[case(GRAY16, ColorFamily::Gray)]
  #[case(RGB24, ColorFamily::RGB)]
  #[case(YUV420P16, ColorFamily::YUV)]
  fn test_color_family(#[case] format: VideoFormat, #[case] expected: ColorFamily) {
    assert_eq!(format.color_family(), expected);
  }

  #[rstest]
  #[case(GRAY8, 16.0)]
  #[case(GRAY16, 4096.0)]
  #[case(RGB24, 0.0)]
  #[case(RGBH, 0.0)]
  #[case(RGBS, 0.0)]
  #[case(YUV420P8, 16.0)]
  #[case(YUV420P16, 4096.0)]
  #[case(YUV420PS, 0.0)]
  #[case(YUV444P8, 16.0)]
  #[case(YUV444P16, 4096.0)]
  #[case(YUV444PS, 0.0)]
  fn test_lowest_value_defaults(#[case] format: VideoFormat, #[case] expected: f32) {
    assert_relative_eq!(format.lowest_value(None, None), expected);
  }

  #[rstest]
  #[case(GRAY8, 16.0)]
  #[case(GRAY16, 4096.0)]
  #[case(RGB24, 0.0)]
  #[case(RGBH, 0.0)]
  #[case(RGBS, 0.0)]
  #[case(YUV420P8, 16.0)]
  #[case(YUV420P16, 4096.0)]
  #[case(YUV420PS, -0.5)]
  #[case(YUV444P8, 16.0)]
  #[case(YUV444P16, 4096.0)]
  #[case(YUV444PS, -0.5)]
  fn test_lowest_value_chroma(#[case] format: VideoFormat, #[case] expected: f32) {
    assert_relative_eq!(format.lowest_value(Some(true), None), expected);
  }

  #[rstest]
  #[case(GRAY8, 128.0)]
  #[case(GRAY16, 32768.0)]
  #[case(GRAYH, 0.0)]
  #[case(GRAYS, 0.0)]
  fn test_neutral_value_defaults(#[case] format: VideoFormat, #[case] expected: f32) {
    assert_relative_eq!(format.neutral_value(), expected);
  }

  #[rstest]
  #[case(GRAY8, 235.0)]
  #[case(GRAY16, 60160.0)]
  #[case(GRAYH, 1.0)]
  #[case(GRAYS, 1.0)]
  #[case(RGB24, 255.0)]
  #[case(RGBH, 1.0)]
  #[case(RGBS, 1.0)]
  fn test_peak_value_defaults(#[case] format: VideoFormat, #[case] expected: f32) {
    assert_relative_eq!(format.peak_value(None, None), expected);
  }
}
