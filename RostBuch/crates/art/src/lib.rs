//! # Art
//!
//! A library for modeling artistic concepts.

pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
  /// The primary colors according to the RYB color model.
  #[derive(Debug)]
  pub enum PrimaryColor {
    Red,
    Yellow,
    Blue,
  }

  /// The secondary colors according to the RYB color model.
  #[derive(Debug)]
  pub enum SecondaryColor {
    Orange,
    Green,
    Purple,
  }
}

pub mod utils {
  use crate::kinds::*;
  /// Combines two primary colors in equal amounts to create
  /// a secondary color.
  pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> Result<SecondaryColor, String> {
    println!("Color 1: {:?}\nColor 2: {:?}", c1, c2);
    match (c1, c2) {
      (PrimaryColor::Red, PrimaryColor::Yellow) | (PrimaryColor::Yellow, PrimaryColor::Red) => {
        Ok(SecondaryColor::Orange)
      }
      (PrimaryColor::Red, PrimaryColor::Blue) | (PrimaryColor::Blue, PrimaryColor::Red) => {
        Ok(SecondaryColor::Purple)
      }
      (PrimaryColor::Blue, PrimaryColor::Yellow) | (PrimaryColor::Yellow, PrimaryColor::Blue) => {
        Ok(SecondaryColor::Green)
      }
      _ => Err("Not two different primary colors".to_string())
    }
  }
}
