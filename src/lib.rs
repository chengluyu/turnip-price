mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

struct Random(u32, u32, u32, u32);

impl Random {
    fn new(seed: u32) -> Random {
        let seed0 = (0x6C078965u64 * ((seed ^ (seed >> 30)) as u64) + 1) as u32;
        let seed1 = (0x6C078965u64 * ((seed0 ^ (seed0 >> 30)) as u64) + 2) as u32;
        let seed2 = (0x6C078965u64 * ((seed1 ^ (seed1 >> 30)) as u64) + 3) as u32;
        let seed3 = (0x6C078965u64 * ((seed2 ^ (seed2 >> 30)) as u64) + 4) as u32;
        Random(seed0, seed1, seed2, seed3)
    }

    fn get_u32(&mut self) -> u32 {
        let n = self.0 ^ (self.0 << 11);
        self.0 = self.1;
        self.1 = self.2;
        self.2 = self.3;
        self.3 = n ^ (n >> 8) ^ self.3 ^ (self.3 >> 19);
        return self.3;
    }

    fn rand_bool(&mut self) -> bool {
        self.get_u32() & 0x8000_0000 != 0
    }

    fn rand_int(&mut self, min: i32, max: i32) -> i32 {
        ((((self.get_u32() as u64) * ((max - min + 1) as u64)) >> 32) + (min as u64)) as i32
    }

    fn rand_float(&mut self, a: f32, b: f32) -> f32 {
        let val: u32 = 0x3F800000 | (self.get_u32() >> 9);
        let pval: *const u32 = &val;
        unsafe {
            let fval: f32 = *(pval as *const f32);
            a + ((fval - 1.0f32) * (b - a))
        }
    }
}

fn ceil(val: f32) -> i32 {
    (val + 0.99999f32) as i32
}

#[wasm_bindgen]
pub struct Prediction {
    pattern: u32,
    prices: [i32; 14]
}

#[wasm_bindgen]
impl Prediction {
    pub fn pattern(&self) -> u32 {
        self.pattern
    }

    pub fn prices(&self) -> *const i32 {
        self.prices.as_ptr()
    }
}

#[wasm_bindgen]
pub fn predict(what_pattern: u32, seed: u32) -> Prediction {
    let mut rng = Random::new(seed);
    let base_price = rng.rand_int(90, 110);
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

    let mut sell_prices: [i32; 14] = [base_price, base_price, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    match next_pattern {
        0 => {
            // Pattern 0: high, decreasing, high, decreasing, high
            let mut work: i32 = 2;
            let dec_phase_len_1: i32 = if rng.rand_bool() { 3 } else { 2 };
            let dec_phase_len_2: i32 = 5 - dec_phase_len_1;

            let hi_phase_len_1: i32 = rng.rand_int(0, 6);
            let hi_phase_len_2_and_3: i32 = 7 - hi_phase_len_1;
            let hi_phase_len_3: i32 = rng.rand_int(0, hi_phase_len_2_and_3 - 1);

            // High phase 1
            for _ in 0..hi_phase_len_1 {
                sell_prices[work as usize] = ceil(rng.rand_float(0.9, 1.4) * (base_price as f32));
                work += 1;
            }

            // Decreasing phase 1
            let mut rate: f32 = rng.rand_float(0.8, 0.6);
            for _ in 0..dec_phase_len_1 {
                sell_prices[work as usize] = ceil(rate * (base_price as f32));
                work += 1;
                rate -= 0.04;
                rate -= rng.rand_float(0.0, 0.06);
            }

            // High phase 2
            for _ in 0..(hi_phase_len_2_and_3 - hi_phase_len_3) {
                sell_prices[work as usize] = ceil(rng.rand_float(0.9, 1.4) * (base_price as f32));
                work += 1;
            }

            // Decreasing phase 2
            rate = rng.rand_float(0.8, 0.6);
            for _ in 0..dec_phase_len_2 {
                sell_prices[work as usize] = ceil(rate * (base_price as f32));
                work += 1;
                rate -= 0.04;
                rate -= rng.rand_float(0.0, 0.06);
            }

            // High phase 3
            for _ in 0..hi_phase_len_3 {
                sell_prices[work as usize] = ceil(rng.rand_float(0.9, 1.4) * (base_price as f32));
                work += 1;
            }
        }
        1 => {
            // Pattern 1: decreasing middle, high spike, random low
            let peak_start: i32 = rng.rand_int(3, 9);
            let mut rate: f32 = rng.rand_float(0.9, 0.85);
            let mut work = 2;
            while work < peak_start {
                sell_prices[work as usize] = ceil(rate * (base_price as f32));
                rate -= 0.03;
                rate -= rng.rand_float(0.0, 0.02);
                work += 1;
            }
            sell_prices[work as usize] = ceil(rng.rand_float(0.9, 1.4) * (base_price as f32));
            work += 1;
            sell_prices[work as usize] = ceil(rng.rand_float(1.4, 2.0) * (base_price as f32));
            work += 1;
            sell_prices[work as usize] = ceil(rng.rand_float(2.0, 6.0) * (base_price as f32));
            work += 1;
            sell_prices[work as usize] = ceil(rng.rand_float(1.4, 2.0) * (base_price as f32));
            work += 1;
            sell_prices[work as usize] = ceil(rng.rand_float(0.9, 1.4) * (base_price as f32));
            work += 1;
            // Fill the remaining days
            while work < 14 {
                sell_prices[work as usize] = ceil(rng.rand_float(0.4, 0.9) * (base_price as f32));
                work += 1;
            }
        }
        2 => {
            // Pattern 2: consistently decreasing
            let mut rate: f32 = 0.9;
            rate -= rng.rand_float(0.0, 0.05);
            for work in 2..14 {
                sell_prices[work] = ceil(rate * (base_price as f32));
                rate -= 0.03;
                rate -= rng.rand_float(0.0, 0.02);
            }
        }
        3 => {
            // Pattern 3: decreasing, spike, decreasing
            let peak_start: i32 = rng.rand_int(2, 9);

            // Decreasing phase before the peak
            let mut rate: f32 = rng.rand_float(0.9, 0.4);

            let mut work: i32 = 2;
            while work < peak_start {
                sell_prices[work as usize] = ceil(rate * (base_price as f32));
                rate -= 0.03;
                rate -= rng.rand_float(0.0, 0.02);
                work += 1;
            }

            sell_prices[work as usize] = ceil(rng.rand_float(0.9, 1.4) * (base_price as f32));
            work += 1;
            sell_prices[work as usize] = ceil(rng.rand_float(0.9, 1.4) * (base_price as f32));
            work += 1;
            rate = rng.rand_float(1.4, 2.0);
            sell_prices[work as usize] = ceil(rng.rand_float(1.4, rate) * (base_price as f32)) - 1;
            work += 1;
            sell_prices[work as usize] = ceil(rate * (base_price as f32));
            work += 1;
            sell_prices[work as usize] = ceil(rng.rand_float(1.4, rate) * (base_price as f32)) - 1;
            work += 1;

            // Decreasing phase after the peak
            if work < 14 {
                rate = rng.rand_float(0.9, 0.4);
                while work < 14 {
                    sell_prices[work as usize] = ceil(rate * (base_price as f32));
                    rate -= 0.03;
                    rate -= rng.rand_float(0.0, 0.02);
                    work += 1;
                }
            }
        }
        _ => {
            for x in sell_prices.iter_mut() {
                *x = -1;
            }
        }
    }

    Prediction { pattern: next_pattern, prices: sell_prices }
}
