//! vapours is a collection of utilities surrounding [vapoursynth4-rs](https://github.com/inflation/vapoursynth4-rs).
//! Generally these aid in VapourSynth plugin development. vapours can also be seen
//! as a Rust equivalent to [vs-tools](https://github.com/Jaded-Encoding-Thaumaturgy/vs-tools)
//! to some extent.
//!
//! For example, the classic invert filter goes from this:
//!
//! ```rust
//! # use vapoursynth4_rs::frame::{VideoFormat, VideoFrame};
//! # fn invert(fi: &VideoFormat, src: &VideoFrame, dst: &mut VideoFrame) {
//! for plane in 0..fi.num_planes {
//!     let mut src_p = src.plane(plane);
//!     let src_stride = src.stride(plane);
//!     let mut dst_p = dst.plane_mut(plane);
//!     let dst_stride = dst.stride(plane);
//!
//!     let h = src.frame_height(plane);
//!     let w = src.frame_width(plane);
//!
//!     for _ in 0..h {
//!         for x in 0..w as usize {
//!             unsafe { *dst_p.wrapping_add(x) = !*src_p.wrapping_add(x) };
//!         }
//!
//!         src_p = src_p.wrapping_offset(src_stride);
//!         dst_p = dst_p.wrapping_offset(dst_stride);
//!     }
//! }
//! # }
//! ```
//!
//! To this:
//!
//! ```rust
//! // Bring in extensions on `VideoFrame` like `rows()` and `rows_mut()`.
//! use vapours::frame::VapoursVideoFrame;
//!
//! // ...
//!
//! # use vapoursynth4_rs::frame::{VideoFormat, VideoFrame};
//! # fn invert(fi: &VideoFormat, src: &VideoFrame, dst: &mut VideoFrame) {
//! for plane in 0..fi.num_planes {
//!     for (src_row, dst_row) in src.rows::<u8>(plane).zip(dst.rows_mut::<u8>(plane)) {
//!         for (src_pixel, dst_pixel) in src_row.iter().zip(dst_row) {
//!             *dst_pixel = !*src_pixel;
//!         }
//!     }
//! }
//! # }
//! ```

#[macro_use]
extern crate num_derive;

pub mod enums;
pub mod errors;
pub mod frame;
pub mod generic;
pub mod scale;
pub mod utils;
pub mod vs_enums;
