mod random;
mod utils;

use wasm_bindgen::prelude::*;
use random::Random;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn ceil(val: f32) -> i32 {
    (val + 0.99999f32) as i32
}

#[wasm_bindgen]
pub struct TurnipPrice {
    pattern: u32,
    buying_price: i32,
    selling_prices: [i32; 12],
}

#[wasm_bindgen]
impl TurnipPrice {
    pub fn pattern(&self) -> u32 {
        self.pattern
    }

    pub fn buying_price(&self) -> i32 {
        self.buying_price
    }

    pub fn selling_prices(&self) -> *const i32 {
        self.selling_prices.as_ptr()
    }
}

#[wasm_bindgen]
pub fn calculate(what_pattern: u32, seed: u32) -> TurnipPrice {
    let mut rng = Random::new(seed);
    let base_price = rng.rand_int(90, 110) as f32;
    let chance = rng.rand_int(0, 99);
    let next_pattern: u32 = match what_pattern {
        0 => match chance {
            0..=19 => 0,
            20..=49 => 1,
            50..=64 => 2,
            _ => 3,
        },
        1 => match chance {
            0..=49 => 0,
            50..=54 => 1,
            55..=74 => 2,
            _ => 3,
        },
        2 => match chance {
            0..=24 => 0,
            25..=69 => 1,
            70..=74 => 2,
            _ => 3,
        },
        3 => match chance {
            0..=44 => 0,
            45..=69 => 1,
            70..=84 => 2,
            _ => 3,
        },
        _ => 2,
    };

    let mut sell_prices: [i32; 12] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    match next_pattern {
        0 => {
            // Pattern 0: high, decreasing, high, decreasing, high
            let mut day: usize = 0;
            let dec_phase_len_1: i32 = if rng.rand_bool() { 3 } else { 2 };
            let dec_phase_len_2: i32 = 5 - dec_phase_len_1;

            let hi_phase_len_1: i32 = rng.rand_int(0, 6);
            let hi_phase_len_2_and_3: i32 = 7 - hi_phase_len_1;
            let hi_phase_len_3: i32 = rng.rand_int(0, hi_phase_len_2_and_3 - 1);

            // High phase 1
            for _ in 0..hi_phase_len_1 {
                sell_prices[day] = ceil(rng.rand_float(0.9, 1.4) * base_price);
                day += 1;
            }

            // Decreasing phase 1
            let mut rate: f32 = rng.rand_float(0.8, 0.6);
            for _ in 0..dec_phase_len_1 {
                sell_prices[day] = ceil(rate * base_price);
                day += 1;
                rate -= 0.04;
                rate -= rng.rand_float(0.0, 0.06);
            }

            // High phase 2
            for _ in 0..(hi_phase_len_2_and_3 - hi_phase_len_3) {
                sell_prices[day] = ceil(rng.rand_float(0.9, 1.4) * base_price);
                day += 1;
            }

            // Decreasing phase 2
            rate = rng.rand_float(0.8, 0.6);
            for _ in 0..dec_phase_len_2 {
                sell_prices[day] = ceil(rate * base_price);
                day += 1;
                rate -= 0.04;
                rate -= rng.rand_float(0.0, 0.06);
            }

            // High phase 3
            for _ in 0..hi_phase_len_3 {
                sell_prices[day] = ceil(rng.rand_float(0.9, 1.4) * base_price);
                day += 1;
            }
        }
        1 => {
            // Pattern 1: decreasing middle, high spike, random low
            let peak_start = rng.rand_int(1, 7) as usize;
            let mut rate: f32 = rng.rand_float(0.9, 0.85);
            let mut day: usize = 0;
            while day < peak_start {
                sell_prices[day] = ceil(rate * base_price);
                rate -= 0.03;
                rate -= rng.rand_float(0.0, 0.02);
                day += 1;
            }
            sell_prices[day] = ceil(rng.rand_float(0.9, 1.4) * base_price);
            day += 1;
            sell_prices[day] = ceil(rng.rand_float(1.4, 2.0) * base_price);
            day += 1;
            sell_prices[day] = ceil(rng.rand_float(2.0, 6.0) * base_price);
            day += 1;
            sell_prices[day] = ceil(rng.rand_float(1.4, 2.0) * base_price);
            day += 1;
            sell_prices[day] = ceil(rng.rand_float(0.9, 1.4) * base_price);
            day += 1;
            // Fill the remaining days
            while day < 12 {
                sell_prices[day] = ceil(rng.rand_float(0.4, 0.9) * base_price);
                day += 1;
            }
        }
        2 => {
            // Pattern 2: consistently decreasing
            let mut rate: f32 = 0.9 - rng.rand_float(0.0, 0.05);
            for price in sell_prices.iter_mut() {
                *price = ceil(rate * base_price);
                rate = rate - 0.03 - rng.rand_float(0.0, 0.02);
            }
        }
        3 => {
            // Pattern 3: decreasing, spike, decreasing
            let peak_start = rng.rand_int(0, 7) as usize;

            // Decreasing phase before the peak
            let mut rate: f32 = rng.rand_float(0.9, 0.4);

            let mut day: usize = 0;
            while day < peak_start {
                sell_prices[day] = ceil(rate * base_price);
                rate -= 0.03;
                rate -= rng.rand_float(0.0, 0.02);
                day += 1;
            }

            sell_prices[day] = ceil(rng.rand_float(0.9, 1.4) * base_price);
            day += 1;
            sell_prices[day] = ceil(rng.rand_float(0.9, 1.4) * base_price);
            day += 1;
            rate = rng.rand_float(1.4, 2.0);
            sell_prices[day] = ceil(rng.rand_float(1.4, rate) * base_price) - 1;
            day += 1;
            sell_prices[day] = ceil(rate * base_price);
            day += 1;
            sell_prices[day] = ceil(rng.rand_float(1.4, rate) * base_price) - 1;
            day += 1;

            // Decreasing phase after the peak
            if day < 12 {
                rate = rng.rand_float(0.9, 0.4);
                while day < 12 {
                    sell_prices[day] = ceil(rate * base_price);
                    rate -= 0.03;
                    rate -= rng.rand_float(0.0, 0.02);
                    day += 1;
                }
            }
        }
        _ => {
            for x in sell_prices.iter_mut() {
                *x = -1;
            }
        }
    }

    TurnipPrice {
        pattern: next_pattern,
        buying_price: base_price as i32,
        selling_prices: sell_prices,
    }
}
