//! This example selects a different skin for ligth or
//! dark terminals
//!
//! Run it with
//!     cargo run --example skin

use crossterm::style::{
    Color,
    ContentStyle,
};

struct Skin {
    high_contrast: ContentStyle,
    low_contrast: ContentStyle,
    code: ContentStyle,
}

fn main() {
    let skin = match terminal_light::luma() {
        Ok(luma) if luma > 0.6 => { // light theme
            Skin {
                high_contrast: ContentStyle {
                    foreground_color: Some(Color::Rgb { r: 40, g: 5, b: 0 }),
                    .. Default::default()
                },
                low_contrast: ContentStyle {
                    foreground_color: Some(Color::Rgb { r: 120, g: 120, b: 80 }),
                    .. Default::default()
                },
                code: ContentStyle {
                    foreground_color: Some(Color::Rgb { r: 50, g: 50, b: 50 }),
                    background_color: Some(Color::Rgb { r: 210, g: 210, b: 210 }),
                    .. Default::default()
                },
            }
        }
        _ => { // dark theme
            Skin {
                high_contrast: ContentStyle {
                    foreground_color: Some(Color::Rgb { r: 250, g: 180, b: 0 }),
                    .. Default::default()
                },
                low_contrast: ContentStyle {
                    foreground_color: Some(Color::Rgb { r: 180, g: 150, b: 0 }),
                    .. Default::default()
                },
                code: ContentStyle {
                    foreground_color: Some(Color::Rgb { r: 220, g: 220, b: 220 }),
                    background_color: Some(Color::Rgb { r: 80, g: 80, b: 80 }),
                    .. Default::default()
                },
            }
        }
    };
    println!("\n {}", skin.low_contrast.apply("This line is easy to read but low intensity"));
    println!("\n {}", skin.high_contrast.apply("This line has a much greater contrast"));
    println!("\n {}", skin.code.apply("    this.is_meant_to_be(some_code);"));
    println!();
}
