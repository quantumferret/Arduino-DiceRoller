/// Credit for the original C++ implementation goes to member EternityForest of
/// https://www.electro-tech-online.com, and the thread containing his implementation and the
/// discussion around it can be found at this link:
/// https://www.electro-tech-online.com/threads/ultra-fast-pseudorandom-number-generator-for-8-bit.124249/
///
/// I've modified it to improve the entropy in higher bits, and implemented an efficient function
/// for generating an unsigned (8-bit) number in the range [0,n),
/// where n is a given upper bound. EternityForest's original comments are below:
///
///
/// "X ABC Algorithm Random Number Generator for 8-Bit Devices:
/// This is a small PRNG, experimentally verified to have at least a 50 million byte period
/// by generating 50 million bytes and observing that there were no overapping sequences and repeats.
/// This generator passes serial correlation, entropy , Monte Carlo Pi value, arithmetic mean,
/// And many other statistical tests. This generator may have a period of up to 2^32, but this has
/// not been verified.
///
/// By XORing 3 bytes into the a,b, and c registers, you can add in entropy from
/// an external source easily.
///
/// This generator is free to use, but is not suitable for cryptography due to its short period(by //cryptographic standards) and simple construction. No attempt was made to make this generator
/// suitable for cryptographic use.
///
/// Due to the use of a constant counter, the generator should be resistant to latching up.
/// A significant performance gain is had in that the x variable is nly ever incremented.
///
/// Only 4 bytes of ram are needed for the internal state, and generating a byte requires 3 XORs , //2 ADDs, one bit shift right , and one increment. Difficult or slow operations like multiply, etc
/// were avoided for maximum speed on ultra low power devices."
use core::num::Wrapping as w;
/* Using Wrapping<u8> to make the code look a little cleaner. Without it, as an example, adding two
u8 values, a and b, with wrapping (as in C/C++ with unsigned numbers), would look like so:
    a.wrapping_add(b);
With Wrapping<u8>, we instead make a and b not u8 values, but Wrapping<u8>, e.g.
    let a = Wrapping(some_u8);
    let b = Wrapping(some_other_u8);
    let c: Wrapping<u8> = a + b;
    let value_wrapped_in_c: u8 = c.0;

By default, in Rust unsigned integer overflow is handled like so:
        In debug mode, arithmetic on signed *and* unsigned primitive integers is checked for overflow,
    panicking if it occurs (this prevents unexpected overflow slipping under the radar, which is
    a common source of bugs in C/C++),
            and
        In release mode, overflow is not checked and is specified to wrap as two's complement.

In practice, most of the time when running Rust on embedded hardware, one will usually use `cargo run
--release`. So the operations here would likely have wrapped by default, thus providing
the behavior one would expect if using unsigned numbers as one would in C/C++.
However, I felt it best to be explicit about the behavior expected here, as well as to avoid
panics that would occur if Wrapping<u8> wasn't used and someone ran the program in debug
mode with different (or default) settings specified.
*/

pub struct Rng {
    x: w<u8>,
    a: w<u8>,
    b: w<u8>,
    c: w<u8>,
}

impl Rng {
    pub fn new(seed: u32) -> Self {
        let mut rng = Rng {
            x: w(0),
            a: w(0),
            b: w(0),
            c: w(0),
        };
        rng.reseed(seed);
        rng
    }

    pub fn reseed(&mut self, seed: u32) {
        let s: [u8; 4] = seed.to_ne_bytes();
        self.init(w(s[0]), w(s[1]), w(s[2]), w(s[3]));
    }

    #[inline]
    fn init(&mut self, s0: w<u8>, s1: w<u8>, s2: w<u8>, s3: w<u8>) {
        self.a ^= s1;
        self.b ^= s2;
        self.c ^= s3;
        self.x = forward_s_box(self.x + s0 + w(1));
        self.a = self.a ^ self.c ^ self.x;
        self.b = self.b + self.a;
        self.c = self.c + inverse_s_box(self.b);
    }

    #[inline]
    pub fn random(&mut self) -> u8 {
        self.x = forward_s_box(self.x + w(1));
        self.a = self.a ^ self.c ^ self.x;
        self.b = self.b + self.a;
        self.c = self.c + inverse_s_box(self.b);
        self.c.0
    }

    /// Based on Daniel Lemire's optimization for generating a random value within a range,
    /// with an extra tweak to prevent unnecessary modulo operations.
    pub fn rand_range(&mut self, bound: u8) -> u8 {
        let mut x = self.random();
        let mut m = (x as u16).wrapping_mul(bound as u16);
        let mut l = m as u8;
        if l < bound {
            let mut t = bound.wrapping_neg();
            if t >= bound {
                t -= bound;
                if t >= bound {
                    t %= bound;
                }
            }
            while l < t {
                x = self.random();
                m = (x as u16).wrapping_mul(bound as u16);
                l = m as u8;
            }
        }
        (m >> 8) as u8
    }

    pub fn rand_u32(&mut self) -> u32 {
        let a: [u8; 4] = [self.random(), self.random(), self.random(), self.random()];
        u32::from_ne_bytes(a)
    }
}

/// Maps an 8-bit input to an 8-bit output, similar to the substitution box used
/// in the Rijndael cipher.
#[inline]
fn forward_s_box(x: w<u8>) -> w<u8> {
    w(x.0.rotate_left(1) ^ x.0.rotate_left(2) ^ x.0.rotate_left(3) ^ x.0.rotate_left(4) ^ 0x63)
}

/// Simply the inverse of the Forward S-box
#[inline]
fn inverse_s_box(x: w<u8>) -> w<u8> {
    w(x.0.rotate_left(1) ^ x.0.rotate_left(3) ^ x.0.rotate_left(6) ^ 0x05)
}
