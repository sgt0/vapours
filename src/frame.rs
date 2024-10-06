//! [`VideoFrame`] utilities.

use core::slice;

use vapoursynth4_rs::frame::VideoFrame;

/// [`VideoFrame`] extensions.
pub trait VapoursVideoFrame {
  /// Returns the video frame's data as a slice.
  #[must_use]
  fn as_slice<T: From<u8>>(&self, plane: i32) -> &[T];

  /// Returns the video frame's data as a mutable slice.
  #[must_use]
  fn as_mut_slice<T: From<u8>>(&mut self, plane: i32) -> &mut [T];
}

impl VapoursVideoFrame for VideoFrame {
  fn as_slice<T: From<u8>>(&self, plane: i32) -> &[T] {
    let stride = self.stride(plane) / size_of::<T>() as isize;
    let ptr: *const T = self.plane(plane).cast::<T>();
    let len = (stride as i32) * self.frame_height(plane);
    unsafe { slice::from_raw_parts(ptr, len as usize) }
  }

  fn as_mut_slice<T: From<u8>>(&mut self, plane: i32) -> &mut [T] {
    let stride = self.stride(plane) / size_of::<T>() as isize;
    let ptr = self.plane_mut(plane).cast::<T>();
    let len = stride as i32 * self.frame_height(plane);
    unsafe { slice::from_raw_parts_mut(ptr, len as usize) }
  }
}
