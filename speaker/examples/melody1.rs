#[macro_use]
extern crate array;

use array::samples::*;
use num::{ratio::*, invert::*};
use wave::*;

use std::{ops::Add, f32::consts::TAU};

macro_rules! s {
    ($x:expr) => {
        (DEFAULT_SAMPLE_RATE as f32 * $x) as usize
    };
}

const NOTE: usize = s!(1.0 / 16.0);
const MELODY: usize = NOTE * 8;
const TIMES: usize = 32;

pub fn gen(sample_rate: f32) -> [f32; MELODY] {
    let factor = 0.5 * TAU * sample_rate.invert();
    let sine = |pitch: f32| move |x: usize|
        (x as f32 * pitch * factor).sin() + 1.0; 
    let c = samples!(NOTE, sine(C));
    let e = samples!(NOTE, sine(E));
    let g = samples!(NOTE, sine(G));
    let a = samples!(NOTE, sine(A));
    let mut data = [0.0; MELODY];
    let fade = fade(in_out, NOTE / 8);
    let mut at = |i, xs| subsume(&mut data[i..], xs, Add::add, &fade);
    at(NOTE * 0, &c);
    at(NOTE * 2, &e);
    at(NOTE * 4, &g);
    at(NOTE * 6, &a);
    data
}
pub fn main() {
    let melody = gen(DEFAULT_SAMPLE_RATE as f32);
    let mut data = [0.0; MELODY * TIMES];
    for i in 0..TIMES {
        let r = f32_ratio(i, (TIMES / 2) - 1);
        let volume = 1.0 + (r * TAU).sin() * 0.5;
        subsume(&mut data[MELODY * i..], &melody, Add::add, |x, _| x * volume);
    }
    speaker::play(DEFAULT_SAMPLE_RATE, &data);
}
