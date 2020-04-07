pub struct Random(u32, u32, u32, u32);

impl Random {
    pub fn new(seed: u32) -> Random {
        let seed0 = (0x6C078965u64 * ((seed ^ (seed >> 30)) as u64) + 1) as u32;
        let seed1 = (0x6C078965u64 * ((seed0 ^ (seed0 >> 30)) as u64) + 2) as u32;
        let seed2 = (0x6C078965u64 * ((seed1 ^ (seed1 >> 30)) as u64) + 3) as u32;
        let seed3 = (0x6C078965u64 * ((seed2 ^ (seed2 >> 30)) as u64) + 4) as u32;
        Random(seed0, seed1, seed2, seed3)
    }

    pub fn get_u32(&mut self) -> u32 {
        let n = self.0 ^ (self.0 << 11);
        self.0 = self.1;
        self.1 = self.2;
        self.2 = self.3;
        self.3 = n ^ (n >> 8) ^ self.3 ^ (self.3 >> 19);
        return self.3;
    }

    pub fn rand_bool(&mut self) -> bool {
        self.get_u32() & 0x8000_0000 != 0
    }

    pub fn rand_int(&mut self, min: i32, max: i32) -> i32 {
        ((((self.get_u32() as u64) * ((max - min + 1) as u64)) >> 32) + (min as u64)) as i32
    }

    pub fn rand_float(&mut self, a: f32, b: f32) -> f32 {
        let val: u32 = 0x3F800000 | (self.get_u32() >> 9);
        let pval: *const u32 = &val;
        unsafe {
            let fval: f32 = *(pval as *const f32);
            a + ((fval - 1.0f32) * (b - a))
        }
    }
}