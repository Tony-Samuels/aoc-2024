use std::{fs::File, io::Write};

static mut LUT: [u8; 256 * 256 * 256] = [u8::MAX; 256 * 256 * 256];

fn main() {
    for num in -99..=-10 {
        let bytes = format!("{num}");
        let bytes = bytes.as_bytes();
        let lookup = ((bytes[0] as usize) << 16) + ((bytes[1] as usize) << 8) + bytes[2] as usize;
        unsafe {
            assert_eq!(
                LUT[lookup],
                u8::MAX,
                "Already written {} for {num}, with lookup {lookup:x}",
                LUT[lookup] as i8
            );
            LUT[lookup] = num as u8;
        }
    }

    for num in -9..=-1 {
        let bytes = format!("{num}");
        let bytes = bytes.as_bytes();
        for third_byte in (0..=u8::MAX).filter(|b| !b.is_ascii_digit()) {
            let lookup =
                ((bytes[0] as usize) << 16) + ((bytes[1] as usize) << 8) + third_byte as usize;
            unsafe {
                assert_eq!(
                    LUT[lookup],
                    u8::MAX,
                    "Already written {} for {num}, with lookup {lookup:x}",
                    LUT[lookup] as i8
                );
                LUT[lookup] = num as u8;
            }
        }
    }

    for num in 100..=i8::MAX {
        let bytes = format!("{num}");
        let bytes = bytes.as_bytes();
        let lookup = ((bytes[0] as usize) << 16) + ((bytes[1] as usize) << 8) + bytes[2] as usize;
        unsafe {
            assert_eq!(
                LUT[lookup],
                u8::MAX,
                "Already written {} for {num}, with lookup {lookup:x}",
                LUT[lookup] as i8
            );
            LUT[lookup] = num as u8;
        }
    }

    for num in 10..=99 {
        let bytes = format!("{num}");
        let bytes = bytes.as_bytes();
        for third_byte in (0..=u8::MAX).filter(|b| !b.is_ascii_digit()) {
            let lookup =
                ((bytes[0] as usize) << 16) + ((bytes[1] as usize) << 8) + third_byte as usize;
            unsafe {
                assert_eq!(
                    LUT[lookup],
                    u8::MAX,
                    "Already written {} for {num}, with lookup {lookup:x}",
                    LUT[lookup] as i8
                );
                LUT[lookup] = num as u8;
            }
        }
    }

    for num in 0..=9 {
        for second_byte in (0..=u8::MAX).filter(|b| !b.is_ascii_digit()) {
            for third_byte in 0..=u8::MAX {
                let lookup = (((num as u8 + b'0') as usize) << 16)
                    + ((second_byte as usize) << 8)
                    + third_byte as usize;
                unsafe {
                    assert_eq!(
                        LUT[lookup],
                        u8::MAX,
                        "Already written {} for {num}, with lookup {lookup:x}",
                        LUT[lookup] as i8
                    );
                    LUT[lookup] = num as u8;
                }
            }
        }
    }

    let output = "src/LUT14.bin";
    #[allow(static_mut_refs)]
    unsafe {
        File::create(output).unwrap().write_all(&LUT).unwrap()
    };
}
