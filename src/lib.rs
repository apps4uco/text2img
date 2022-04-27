#![deny(missing_docs)]
//! Creates a png image from text.
//! Useful to protect sensitive information from bots and scrapers,
//! such as email addresses and telephone numbers in web pages,
//! and even passwords sent via email
//!
//! - [x] Png Support 8 bit rgba
//! - [x] Font Weight implemented as Random pixels
//! - [ ] Png Support 1 bit monochrome -  to reduce image size
//! - [ ] Poisson Disk rendering

use fontdue::layout::{CoordinateSystem, Layout, TextStyle};
use raqote::DrawTarget;

const FONT_DATA: &[u8]= include_bytes!("../assets/Roboto-Regular.ttf") as &[u8];

/// Converts a text string into an image
///
/// # Arguments
/// * `text` - a non empty string to render
/// * `weight` - a percentage of pixels to be painted
/// * `font_size` - the size of the font. Note this is not the same as the height of the image
/// # Return
/// the result. DrawTarget on success, an error message on failure
///
/// # Deprecated
/// This should not leak the implementation of DrawTarget from the raqote crate,
/// and will be removed in a future release.
#[must_use]
#[deprecated]
pub fn text2img_internal(text:String, weight:u8, font_size:u8) -> Result<DrawTarget,String> {

    if text.is_empty() {
        return Err("Content can not be empty".to_owned());
    }

    if weight==0 {
        return Err("Weight can not be zero".to_owned());
    }

    let settings = fontdue::FontSettings::default();

    let font=fontdue::Font::from_bytes(FONT_DATA, settings).unwrap();

    let mut layout = Layout::new(CoordinateSystem::PositiveYDown);

    //TODO_maybe this is for centering text in larger image
    // let x=0;
    // let y=0;
    // let width=500;
    // let height=70;
    // layout.reset(&LayoutSettings {
    // 	x: x as f32,
    // 	y: y as f32,
    // 	max_width: Option::Some(width as f32),
    // 	max_height: Option::Some(height as f32),
    // 	horizontal_align: fontdue::layout::HorizontalAlign::Center,
    // 	..LayoutSettings::default()
    // });

    let fonts = &[font.clone()];
    layout.append(fonts, &TextStyle::new(&text, font_size as f32, 0));
    let height=layout.height() as i32;
    let first=layout.glyphs().first().unwrap();
    let last=layout.glyphs().last().unwrap();

    //This calculates the x of the last pixel and adds a margin equivalent to the first glyphs pixel
    let width=(last.x as usize + last.width + first.x as usize) as i32;

    let mut dt = DrawTarget::new(width, height);

    for glyph in layout.glyphs() {
        let glyph=glyph.to_owned();
        let index=glyph.key.glyph_index;
        let px=glyph.key.px;
        let (metrics, coverage) = font.rasterize_indexed(index, px);

        let mut image_data = Vec::with_capacity(coverage.len());

        //TODO split this off into a trait
        for cov in coverage.iter() {
            let pixel = if weight==100 || fastrand::u8(0..=100)<=weight {
                rgb_to_u32(0, 0, 0, *cov)
            } else { 0};
            image_data.push(pixel);
        }
        dt.draw_image_at(
            glyph.x,
            glyph.y,
            &raqote::Image {
                width: metrics.width as i32,
                height: metrics.height as i32,
                data: &image_data,
            },
            &raqote::DrawOptions {
                blend_mode: raqote::BlendMode::Darken,
                alpha: 1.0,
                antialias: raqote::AntialiasMode::Gray,
            },
        );
    }

    Ok(dt)
}

/// Converts a rgba u8 to a u32
/// # Arguments
/// * `red` - red component byte
/// * `green` - green component byte
/// * `blue` - blue component byte
/// * `alpha` - añpha component byte, 255 is opaque, 0 is transparent
///
/// # Return
/// argb as a u32
pub fn rgb_to_u32(red: u8, green: u8, blue: u8, alpha: u8) -> u32 {
    (((alpha as u32) << 24) | ((red as u32) << 16) | ((green as u32) << 8) | (blue as u32)) as u32
}
