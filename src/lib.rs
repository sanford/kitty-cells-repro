use image::{DynamicImage, Rgba, RgbaImage};

/// A test picture: a colour gradient with a white border and a diagonal,
/// so cropping or wrong scaling is easy to spot.
pub fn test_image(w: u32, h: u32) -> DynamicImage {
    let img = RgbaImage::from_fn(w, h, |x, y| {
        let border = x < 3 || y < 3 || x >= w - 3 || y >= h - 3;
        let diag = (x * h / w).abs_diff(y) < 2;
        if border || diag {
            Rgba([255, 255, 255, 255])
        } else {
            Rgba([(x * 255 / w) as u8, (y * 255 / h) as u8, 160, 255])
        }
    });
    DynamicImage::ImageRgba8(img)
}
