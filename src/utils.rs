use const_str::cstr;
use strum_macros::EnumString;
use vapoursynth4_rs::{
  core::Core,
  key,
  map::{AppendMode, Value},
  node::VideoNode,
};

use crate::errors::VapoursError;

const FMTCONV_NAMESPACE: &str = "fmtc";

/// Enum for `zimg_dither_type_e` and fmtconv `dmode`.
#[derive(Clone, Copy, Debug, EnumString, Eq, PartialEq)]
#[strum(ascii_case_insensitive, serialize_all = "snake_case")]
pub enum DitherType {
  /// Choose automatically.
  Auto,

  /// Round to nearest.
  None,

  /// Bayer patterned dither.
  Ordered,

  /// Pseudo-random noise of magnitude 0.5.
  Random,

  /// Floyd-Steinberg error diffusion.
  ErrorDiffusion,

  /// Floyd-Steinberg error diffusion. Modified for serpentine scan (avoids worm
  /// artifacts).
  ErrorDiffusionFmtc,

  /// Another type of error diffusion. Quick and excellent quality, similar to
  /// Floyd-Steinberg.
  #[strum(serialize = "sierra_2_4a")]
  Sierra24a,

  /// Another error diffusion kernel. Preserves delicate edges better but
  /// distorts gradients.
  Stucki,

  /// Another error diffusion kernel. Generates distinct patterns but keeps
  /// clean the flat areas (noise modulation).
  Atkinson,

  /// Another error diffusion kernel. Slow, available only for integer input at
  /// the moment. Avoids usual F-S artifacts.
  Ostromoukhov,

  /// A way to generate blue-noise dither and has a much better visual aspect
  /// than ordered dithering.
  Void,

  /// Dither using quasirandom sequences. Good intermediary between void,
  /// cluster, and error diffusion algorithms.
  Quasirandom,
}

/// [`Core`] extensions.
pub trait VapoursCore {
  /// Bit depth conversion.
  ///
  /// # Errors
  ///
  /// Returns an error if the fmtconv plugin is not found or on any error
  /// accessing frame properties.
  fn depth(&self, clip: VideoNode, bit_depth: u32) -> Result<VideoNode, VapoursError>;
}

impl VapoursCore for Core {
  #[allow(unreachable_code)]
  #[allow(unused_variables)]
  fn depth(&self, clip: VideoNode, bit_depth: u32) -> Result<VideoNode, VapoursError> {
    todo!("Needs configurable dither type, non-fmtc dithering, and probably more.");

    let Some(fmtc_plugin) = self.get_plugin_by_id(cstr!(FMTCONV_NAMESPACE)) else {
      return Err(VapoursError::DependencyNotFoundError(
        FMTCONV_NAMESPACE.to_string(),
      ));
    };

    let mut args = self.create_map();
    args
      .set(key!(c"clip"), Value::VideoNode(clip), AppendMode::Replace)
      .map_err(|_| VapoursError::FramePropertyError("clip".to_string()))?;
    args
      .set(
        key!(c"bitdepth"),
        Value::Int(i64::from(bit_depth)),
        AppendMode::Replace,
      )
      .map_err(|_| VapoursError::FramePropertyError("clip".to_string()))?;
    let ret = fmtc_plugin.invoke(cstr!("bitdepth"), &args);
    ret
      .get_video_node(key!(c"clip"), 0)
      .map_err(|_| VapoursError::FramePropertyError("clip".to_string()))
  }
}
