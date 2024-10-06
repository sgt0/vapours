//! Traits that apply to multiple types.

use num_traits::ToPrimitive;
use vapoursynth4_rs::{
  ffi::VSColorFamily,
  frame::{VideoFormat, VideoFrame},
  node::VideoNode,
  SampleType, VideoInfo,
};

use crate::enums::ColorRange;

/// A trait for types that hold a video format.
pub trait HoldsVideoFormat: Sized {
  /// Get the video format.
  #[must_use]
  fn video_format(&self) -> &VideoFormat;

  /// Get the color family of this clip or format.
  #[must_use]
  fn color_family(&self) -> VSColorFamily {
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
    let chroma = chroma.unwrap_or(false);
    let range_in = range_in.unwrap_or(ColorRange::Full);

    if self.sample_type() == SampleType::Float {
      return if chroma { -0.5 } else { 0.0 };
    }

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
    let chroma = chroma.unwrap_or(false);
    let range_in = range_in.unwrap_or(ColorRange::Full);

    if self.sample_type() == SampleType::Float {
      return if chroma { 0.5 } else { 1.0 };
    }

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
