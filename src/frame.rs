//! [`VideoFrame`] utilities.

use core::slice;
use std::{iter::FusedIterator, marker::PhantomData, ops::Range};

use vapoursynth4_rs::frame::VideoFrame;

use crate::generic::HoldsVideoFormat;

/// [`VideoFrame`] extensions.
pub trait VapoursVideoFrame {
  /// Returns the video frame's data as a slice.
  #[must_use]
  fn as_slice<T: From<u8>>(&self, plane: i32) -> &[T];

  /// Returns the video frame's data as a mutable slice.
  #[must_use]
  fn as_mut_slice<T: From<u8>>(&mut self, plane: i32) -> &mut [T];

  /// Returns an iterator over the planes of the video frame.
  fn planes_iter<T: From<u8>>(&self) -> PlanesIter<'_, T>;

  /// Returns an iterator over the mutable planes of the video frame.
  fn planes_iter_mut<T: From<u8>>(&mut self) -> PlanesIterMut<'_, T>;
}

impl VapoursVideoFrame for VideoFrame {
  #[inline]
  fn as_slice<T: From<u8>>(&self, plane: i32) -> &[T] {
    let stride = self.stride(plane) / size_of::<T>() as isize;
    let ptr: *const T = self.plane(plane).cast::<T>();
    let len = (stride as i32) * self.frame_height(plane);
    unsafe { slice::from_raw_parts(ptr, len as usize) }
  }

  #[inline]
  fn as_mut_slice<T: From<u8>>(&mut self, plane: i32) -> &mut [T] {
    let stride = self.stride(plane) / size_of::<T>() as isize;
    let ptr = self.plane_mut(plane).cast::<T>();
    let len = stride as i32 * self.frame_height(plane);
    unsafe { slice::from_raw_parts_mut(ptr, len as usize) }
  }

  #[inline]
  fn planes_iter<T: From<u8>>(&self) -> PlanesIter<'_, T> {
    PlanesIter::new(self)
  }

  #[inline]
  fn planes_iter_mut<T: From<u8>>(&mut self) -> PlanesIterMut<'_, T> {
    PlanesIterMut::new(self)
  }
}

/// A plane view.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlaneView<'a, T> {
  /// Plane data.
  pub data: &'a [T],

  /// Height in pixels.
  pub height: i32,

  /// Width in pixels.
  pub width: i32,

  /// Distance in bytes between two consecutive lines.
  pub stride: isize,
}

/// An iterator that yields the plane data of a [`VideoFrame`] along with their
/// dimensions.
#[derive(Clone, Debug)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct PlanesIter<'a, T> {
  frame: &'a VideoFrame,
  range: Range<usize>,
  _marker: PhantomData<&'a T>,
}

impl<'a, T> PlanesIter<'a, T> {
  pub(super) fn new(frame: &'a VideoFrame) -> Self {
    Self {
      frame,
      range: 0..frame.video_format().num_planes as usize,
      _marker: PhantomData,
    }
  }
}

impl<'a, T: From<u8>> Iterator for PlanesIter<'a, T> {
  type Item = PlaneView<'a, T>;

  #[inline]
  fn next(&mut self) -> Option<PlaneView<'a, T>> {
    debug_assert!(self.range.start <= self.range.end);
    if self.range.start < self.range.end {
      let plane = PlaneView {
        data: self.frame.as_slice(self.range.start as i32),
        height: self.frame.frame_height(self.range.start as i32),
        width: self.frame.frame_width(self.range.start as i32),
        stride: self.frame.stride(self.range.start as i32),
      };
      self.range.start += 1;
      Some(plane)
    } else {
      None
    }
  }

  #[inline]
  fn size_hint(&self) -> (usize, Option<usize>) {
    let n = self.range.len();
    (n, Some(n))
  }

  #[inline]
  fn count(self) -> usize {
    self.range.len()
  }
}

impl<T: From<u8>> DoubleEndedIterator for PlanesIter<'_, T> {
  fn next_back(&mut self) -> Option<Self::Item> {
    debug_assert!(self.range.start <= self.range.end);
    if self.range.is_empty() {
      None
    } else {
      self.range.end -= 1;
      debug_assert!(self.range.end < self.frame.video_format().num_planes as usize);
      debug_assert!(self.range.end >= self.range.start);
      Some(PlaneView {
        data: self.frame.as_slice(self.range.end as i32),
        height: self.frame.frame_height(self.range.end as i32),
        width: self.frame.frame_width(self.range.end as i32),
        stride: self.frame.stride(self.range.end as i32),
      })
    }
  }
}

impl<T: From<u8>> ExactSizeIterator for PlanesIter<'_, T> {
  #[inline]
  fn len(&self) -> usize {
    self.range.end - self.range.start
  }
}

impl<T: From<u8>> FusedIterator for PlanesIter<'_, T> {}

/// A mutable plane view.
pub struct PlaneViewMut<'a, T> {
  /// Plane data.
  pub data: &'a mut [T],

  /// Height in pixels.
  pub height: i32,

  /// Width in pixels.
  pub width: i32,

  /// Distance in bytes between two consecutive lines.
  pub stride: isize,
}

/// An iterator that yields the mutable plane data of a [`VideoFrame`] along
/// with their dimensions.
#[derive(Debug)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct PlanesIterMut<'a, T> {
  frame: *mut VideoFrame,
  range: Range<usize>,
  _marker: PhantomData<&'a mut T>,
}

impl<'a, T> PlanesIterMut<'a, T> {
  pub(super) fn new(frame: &'a mut VideoFrame) -> Self {
    Self {
      frame,
      range: 0..frame.video_format().num_planes as usize,
      _marker: PhantomData,
    }
  }
}

impl<'a, T: From<u8>> Iterator for PlanesIterMut<'a, T> {
  type Item = PlaneViewMut<'a, T>;

  #[inline]
  fn next(&mut self) -> Option<PlaneViewMut<'a, T>> {
    debug_assert!(self.range.start <= self.range.end);
    if self.range.start < self.range.end {
      let plane = unsafe {
        PlaneViewMut {
          height: (*self.frame).frame_height(self.range.start as i32),
          width: (*self.frame).frame_width(self.range.start as i32),
          stride: (*self.frame).stride(self.range.start as i32),
          data: (*self.frame).as_mut_slice(self.range.start as i32),
        }
      };
      self.range.start += 1;
      Some(plane)
    } else {
      None
    }
  }

  #[inline]
  fn size_hint(&self) -> (usize, Option<usize>) {
    let n = self.range.len();
    (n, Some(n))
  }

  #[inline]
  fn count(self) -> usize {
    self.range.len()
  }
}

impl<T: From<u8>> DoubleEndedIterator for PlanesIterMut<'_, T> {
  fn next_back(&mut self) -> Option<Self::Item> {
    debug_assert!(self.range.start <= self.range.end);
    if self.range.is_empty() {
      None
    } else {
      self.range.end -= 1;
      unsafe {
        debug_assert!(self.range.end < (*self.frame).video_format().num_planes as usize);
        debug_assert!(self.range.end >= self.range.start);
        Some(PlaneViewMut {
          data: (*self.frame).as_mut_slice(self.range.end as i32),
          height: (*self.frame).frame_height(self.range.end as i32),
          width: (*self.frame).frame_width(self.range.end as i32),
          stride: (*self.frame).stride(self.range.end as i32),
        })
      }
    }
  }
}

impl<T: From<u8>> ExactSizeIterator for PlanesIterMut<'_, T> {
  #[inline]
  fn len(&self) -> usize {
    self.range.end - self.range.start
  }
}

impl<T: From<u8>> FusedIterator for PlanesIterMut<'_, T> {}
