// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let speed: f64 = speed as f64;
    if speed < 5.0 {
        speed * 221.0
    }else if speed < 9.0{
        speed * 221.0 * 0.9
    }else {
        speed * 221.0 * 0.77
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    let items_per_hour = production_rate_per_hour(speed) as u32;
    let speed: u32 = speed as u32;
    items_per_hour / 60
}
