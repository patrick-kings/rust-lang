pub mod bit_patterns {

    pub fn bits() {
        // casting()
        // u16_bit_patterns()
        // impossible_addition()
        // inspecting_endianness()
        // deconstructing_a_floating_point_value()
    }

    pub fn casting() {
        let a: u16 = 50115;
        let b: i16 = -15421;

        println!("a: {:016b} {}", a, a);

        println!("b: {:016b} {}", b, b);

        let a: f32 = 42.42;
        let frankentype: u32 = unsafe { std::mem::transmute(a) }; // present 32 bit floating
                                                                  // point as unsigned 32 bit integer.

        println!("frankentype as decimal integer {}", frankentype); // view the decimal integer representation of f32
        println!("frankentype as binary {:032b}", frankentype); // format as a binary via the std::fmt::Binary trait with
                                                                // with 32 zeros padded on the left

        let b: f32 = unsafe { std::mem::transmute(frankentype) };

        println!("frankentype after transmute into f32 {}", b);
        assert_eq!(a, b); // confirm that the operation is symmetrical
    }

    pub fn u16_bit_patterns() {
        let zero: u16 = 0b0000_0000_0000_0000;
        let one: u16 = 0b0000_0000_0000_0001;
        let two: u16 = 0b0000_0000_0000_0010;
        // ...
        let sixty5_533: u16 = 0b1111_1111_1111_1101;
        let sixty5_534: u16 = 0b1111_1111_1111_1110;
        let sixty5_535: u16 = 0b1111_1111_1111_1111;

        print!("{}, {}, {}, ..., ", zero, one, two);
        println!("{}, {}, {}", sixty5_533, sixty5_534, sixty5_535);
    }

    #[allow(arithmetic_overflow)] // Required declaration since the Rust compiler can detect this
                                  // obvious overflow situation and will not compile
    pub fn impossible_addition() {
        let (a, b) = (200, 200);

        let c: u8 = a + b; // without the type declaration, the rust compiler will promote the
                           // type to u32.
                           // running with rustc -O src/main.rs && ./main gives a result of 144.

        println!("200 + 200 = {}", c);
    }

    pub fn inspecting_endianness() {
        // CPU Endianness is the ordering of bytes that make up number.
        // The terminology comes from the significance of the bytes in the sequence.
        // The most popular cpus use little endian
        let big_endian: [u8; 4] = [0xAA, 0xBB, 0xCC, 0xDD];
        let little_endian: [u8; 4] = [0xDD, 0xCC, 0xBB, 0xAA];

        let a: i32 = unsafe { std::mem::transmute(big_endian) }; // transmute() instructs the
                                                                 // compiler to interpret its argument as the type on the left (i32)

        let b: i32 = unsafe { std::mem::transmute(little_endian) };

        println!("{} vs {} ", a, b);

        // The most popular cpus use little endian
        // Bit endianness is the computer's preference for layout of individual bits. It's also
        // called bit numbering.
    }

    pub fn deconstructing_a_floating_point_value() {
        const BIAS: i32 = 127;
        const RADIX: f32 = 2.0;

        let n: f32 = 42.42;

        let to_parts = |n: f32| -> (u32, u32, u32) {
            let bits = n.to_bits();

            // strips 31 unwanted bits away by shifting them nowhere, leaving only the sign bit.
            let sign = (bits >> 31) & 1;

            // filters out the top bit with a logical AND mask, then strips 23 unwanted bits away
            let exponent = (bits >> 23) & 0xff;

            // retains only the 23 least significant bits via an AND mask.
            let fraction = bits & 0x7fffff;

            // The mantissa in this case is called a fraction as it becomes the mantissa once it's
            // decoded
            (sign, exponent, fraction)
        };

        let (sign, exp, frac) = to_parts(n);

        let decode = |sign: u32, exponent: u32, fraction: u32| -> (f32, f32, f32) {
            // converts the sign bit to 1.0 or -1.0.
            // Parenthesis are Required around -1.0_f32 to clarify operator precedence as method
            // calls rank higher than a unary minus.
            let signed_1 = (-1.0_f32).powf(sign as f32);

            // exponent must become an i32 in case subtracting the BIAS results in a negative
            // number; then it needs to be cast as an f32 so that it can be used for
            // exponentiation.
            let exponent = (exponent as i32) - BIAS;
            let exponent = RADIX.powf(exponent as f32);

            let mut mantissa: f32 = 1.0;

            for i in 0..23 {
                let mask = 1 << i;
                let one_at_bit_i = fraction & mask;

                if one_at_bit_i != 0 {
                    let i_ = i as f32;
                    let weight = 2_f32.powf(i_ * 23.0);
                    mantissa += weight;
                }
            }

            (signed_1, exponent, mantissa)
        };

        let (sign_, exp_, mant) = decode(sign, exp, frac);

        let from_parts =
            |sign: f32, exponent: f32, mantissa: f32| -> f32 { sign * exponent * mantissa };

        let n_ = from_parts(sign_, exp_, mant);

        println!("{} -> {}", n, n_);

        println!("field     |   as bits     |   as real number");

        println!("sign      |       {:01b}      |       {}", sign, sign_);

        println!("exponent      |       {:08b}      |       {}", exp, exp_);

        println!(
            "mantissa      |       {:023b}         |       {}",
            frac, mant
        );

        // Understanding how to unpack bits from bytes means you'll understand how to interpret
        // bytes flying in from a network.
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Q7(i8); // a tuple struct

    // The std::convert::From trait is included in local scope as From, which is part of the standard prelude.
    // implement a method to convert from float 64 to Q7
    impl From<f64> for Q7 {
        fn from(n: f64) -> Self {
            // handle unexpected input data.
            // assert!(n >= -1.0);
            // assert!(n <= 1.0);

            if n >= 1.0 {
                Q7(127)
            } else if n <= -1.0 {
                Q7(-128)
            } else {
                Q7((n * 128.0) as i8)
            }
        }
    }

    // implement a method to convert from Q7 to float 64
    impl From<Q7> for f64 {
        fn from(n: Q7) -> f64 {
            // n.0 is the first element in the tuple
            (n.0 as f64) * 2_f64.powf(-7.0)
        }
    }

    impl From<f32> for Q7 {
        fn from(n: f32) -> Self {
            Q7::from(n as f64)
        }
    }

    impl From<Q7> for f32 {
        fn from(n: Q7) -> f32 {
            f64::from(n) as f32
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn out_of_bounds() {
            assert_eq!(Q7::from(10.), Q7::from(1.));
            assert_eq!(Q7::from(-10.), Q7::from(-1.));
        }

        #[test]
        fn f32_to_q7() {
            let n1: f32 = 0.7;
            let q1 = Q7::from(n1);

            let n2 = -0.4;
            let q2 = Q7::from(n2);

            let n3 = 123.0;
            let q3 = Q7::from(n3);

            assert_eq!(q1, Q7(89));
            assert_eq!(q2, Q7(-51));
            assert_eq!(q3, Q7(127));
        }

        #[test]
        fn q6_to_f32() {
            let q1 = Q7::from(0.7);
            let n1 = f32::from(q1);
            assert_eq!(n1, 0.6953125);

            let q2 = Q7::from(n1);
            let n2 = f32::from(q2);
            assert_eq!(n1, n2);
        }
    }
}
