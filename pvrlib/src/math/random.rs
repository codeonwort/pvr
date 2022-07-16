// Rust prelude does not include even a basic rand()?

// Mersenne Twister RNG forked from:
// http://www.math.sci.hiroshima-u.ac.jp/m-mat/MT/VERSIONS/C-LANG/mt19937-64.c

const NN: usize = 312;
const MM: usize = 156;
const MATRIX_A: u64 = 0xB5026F5AA96619E9;
const UM: u64 = 0xFFFFFFFF80000000; /* Most significant 33 bits */
const LM: u64 = 0x7FFFFFFF; /* Least significant 31 bits */

pub struct MT19937 {
    mt: [u64; NN],
    mti: usize
}

impl MT19937 {
    pub fn new(seed: u64) -> Self {
        let mut result = MT19937 {
            mt: [0; NN],
            mti: NN + 1
        };
        result.mt[0] = seed;
        for mti in 1..NN {
            result.mt[mti] = 6364136223846793005 * (result.mt[mti-1] ^ (result.mt[mti-1] >> 62)) + (mti as u64);
        }

        result
    }

    pub fn rand_int64(&mut self) -> u64 {
        let mut x: u64;
        let mag01: [u64; 2] = [0, MATRIX_A];

        if self.mti >= NN {
            for i in 0..(NN - MM) {
                x = (self.mt[i] & UM) | (self.mt[i+1] & LM);
                self.mt[i] = self.mt[i + MM] ^ (x >> 1) ^ mag01[(x & 1) as usize];
            }
            for i in (NN - MM)..(NN - 1) {
                x = (self.mt[i] & UM) | (self.mt[i+1] & LM);
                self.mt[i] = self.mt[i + MM - NN] ^ (x >> 1) ^ mag01[(x & 1) as usize];
            }
            x = (self.mt[NN - 1] & UM) | (self.mt[0] & LM);
            self.mt[NN - 1] = self.mt[MM - 1] ^ (x >> 1) ^ mag01[(x & 1) as usize];
            self.mti = 0;
        }

        x = self.mt[self.mti];
        self.mti += 1;

        x ^= (x >> 29) & 0x5555555555555555;
        x ^= (x << 17) & 0x71D67FFFEDA60000;
        x ^= (x << 37) & 0xFFF7EEE000000000;
        x ^= x >> 43;

        x
    }

    pub fn rand(&mut self) -> f64 {
        ((self.rand_int64() >> 11) as f64) * (1.0 / 9007199254740991.0)
    }

    pub fn rand_range(&mut self, min: f64, max: f64) -> f64 {
        min + (max - min) * self.rand()
    }
}
