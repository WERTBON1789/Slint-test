#![allow(unused)]
#![windows_subsystem = "windows"]

use rand::{Rng, thread_rng};
use slint::Weak;



slint::include_modules!();
fn main() {
    let mut a = Test::new();

    let mut rng = thread_rng();

    let b = a.as_weak();

    let mut change_color = move || {
        b.unwrap().global::<Colors>().set_background1(slint::Color::from_rgb_u8(rng.gen_range(0..=255),rng.gen_range(0..=255),rng.gen_range(0..=255)));
        b.unwrap().global::<Colors>().set_background2(slint::Color::from_rgb_u8(rng.gen_range(0..=255),rng.gen_range(0..=255),rng.gen_range(0..=255)));
        b.unwrap().global::<Colors>().set_background3(slint::Color::from_rgb_u8(rng.gen_range(0..=255),rng.gen_range(0..=255),rng.gen_range(0..=255)));
        b.unwrap().global::<Colors>().set_background4(slint::Color::from_rgb_u8(rng.gen_range(0..=255),rng.gen_range(0..=255),rng.gen_range(0..=255)));
    };

    a.on_change_color(change_color);

    a.run();
}

