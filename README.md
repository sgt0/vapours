# vapours

vapours is a collection of utilities surrounding [vapoursynth4-rs](https://github.com/inflation/vapoursynth4-rs).
Generally these aid in VapourSynth plugin development. vapours can also be seen
as a Rust equivalent to [vs-tools](https://github.com/Jaded-Encoding-Thaumaturgy/vs-tools)
to some extent.

For example, the classic invert filter goes from this:

```rust
for plane in 0..fi.num_planes {
    let mut src_p = src.plane(plane);
    let src_stride = src.stride(plane);
    let mut dst_p = dst.plane_mut(plane);
    let dst_stride = dst.stride(plane);

    let h = src.frame_height(plane);
    let w = src.frame_width(plane);

    for _ in 0..h {
        for x in 0..w as usize {
            unsafe { *dst_p.wrapping_add(x) = !*src_p.wrapping_add(x) };
        }

        src_p = src_p.wrapping_offset(src_stride);
        dst_p = dst_p.wrapping_offset(dst_stride);
    }
}
```

To this:

```rust
// Bring in extensions on `VideoFrame` like `as_slice()` and `as_mut_slice()`.
use vapours::frame::VapoursVideoFrame;

// ...

for plane in 0..fi.num_planes {
    let src_slice = src.as_slice::<u8>(plane);
    let dst_slice = dst.as_mut_slice::<u8>(plane);

    for (src_pixel, dst_pixel) in zip(src_slice, dst_slice) {
        *dst_pixel = !*src_pixel;
    }
}
```
