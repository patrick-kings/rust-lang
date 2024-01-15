pub mod bit_patterns {

    pub fn bits() {
        // casting()
        // u16_bit_patterns()
        // impossible_addition()
        inspecting_endianness()
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
        let big_endian: [u8; 4] = [0xAA, 0xBB, 0xCC, 0xDD];
        let little_endian: [u8; 4] = [0xDD, 0xCC, 0xBB, 0xAA];

        let a: i32 = unsafe { std::mem::transmute(big_endian) }; // transmute() instructs the
                                                                 // compiler to interpret its argument as the type on the left (i32)

        let b: i32 = unsafe { std::mem::transmute(little_endian) };

        println!("{} vs {} ", a, b);
    }
}
