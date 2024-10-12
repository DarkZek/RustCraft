use std::ops::Range;
use crate::helpers::map;

pub fn daylight_amount(t: f32) -> f32 {
    match t {
        0.0..0.5 => {
            1.0
        }
        0.5..0.65 => {
            map(0.5..0.65, 1.0..0.0, t)
        }
        0.65..0.85 => {
            0.0
        }
        0.85..=1.0 => {
            map(0.85..1.0, 0.0..1.0, t)
        }
        _ => panic!("Invalid input provided to daylight_amount")
    }
}
