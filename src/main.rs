#![no_std]
#![no_main]
#![feature(min_const_generics)]
mod pong_controller;

use psp;
use embedded_graphics::{
    style::{PrimitiveStyleBuilder, TextStyleBuilder},
    primitives::{circle::Circle,rectangle::Rectangle},
    prelude::*,
    pixelcolor::Rgb888,
    fonts::{Font6x8,Text},
    image::Image};
use crate::pong_controller::pong_controller::PongController;

psp::module!("psp pong", 1, 1);

fn psp_main() {
    psp::enable_home_button();
    let mut pong = PongController::new(20);
    pong.run();
}
