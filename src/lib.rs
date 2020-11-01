pub extern crate image;
pub extern crate vecmath;

use image::Rgb;
use vecmath::Vector3;

pub type Vecf = Vector3<f32>;
pub type Color = Rgb<u8>;
pub mod scene;
pub mod view;
