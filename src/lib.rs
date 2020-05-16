extern crate rand;

pub mod analyzer;
pub mod collector;
pub mod detector;
pub mod input;
pub mod output;
pub mod parser;

pub trait Data {
    fn get_numeric_value(&self) -> f64;
}

impl Data for i32 {
    fn get_numeric_value(&self) -> f64 {
        *self as f64
    }
}
impl Data for f64 {
    fn get_numeric_value(&self) -> f64 {
        *self as f64
    }
}

impl Data for f32 {
    fn get_numeric_value(&self) -> f64 {
        *self as f64
    }
}
