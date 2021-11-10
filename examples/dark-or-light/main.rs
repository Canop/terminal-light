//! Run this with
//!     cargo run --example dark-or-light

fn main() {
    println!();
    match terminal_light::luma() {
        Ok(luma) if luma > 0.85 => {
            println!(" You're using a light theme (luma={})", luma);
        }
        Ok(luma) if luma < 0.2 => {
            println!(" You're using a dark theme (luma={})", luma);
        }
        Ok(luma) => {
            println!(" You're using a kind of intermediate theme (luma={})", luma);
        }
        Err(e) => {
            println!(" I couldn't determine the background's luma: {}", e);
        }
    }
    println!();
}
