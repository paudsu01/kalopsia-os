use crate::print;
use crate::println;
use alloc::string::String;

pub async fn memdump_command(args: String) {
    if args.is_empty() {
        return;
    }
    // Parse args "0xb8000 64" (addr, length)
    let (addr_str, len_str) = match args.trim().split_once(char::is_whitespace) {
        Some((addr_str, len_str)) => (addr_str, len_str.trim()),
        None => (args.as_str(), "64"),
    };
    let len = len_str.parse().unwrap_or(64);
    let start_addr = parse_hex_u64(addr_str);
    let Some(start_addr) = start_addr else {
        return;
    };
    println!("Dumping {} bytes at {:#x}", len, start_addr);

    let ptr = start_addr as *const u8;

    for i in (0..len).step_by(16) {
        print!("{:08x}  ", start_addr + i as u64); // Address offset

        // Print Hex Bytes
        for j in 0..16 {
            if i + j < len {
                unsafe {
                    print!("{:02x} ", *ptr.add(i + j));
                }
            } else {
                print!("   "); // Padding
            }
        }

        print!(" | ");

        // Print ASCII
        for j in 0..16 {
            if i + j < len {
                let byte: u8 = unsafe { *ptr.add(i + j) };
                // Only print printable ASCII (32-126), else '.'
                if (32..=126).contains(&byte) {
                    print!("{}", byte as char);
                } else {
                    print!(".");
                }
            }
        }
        println!();
    }
}

pub fn parse_hex_u64(s: &str) -> Option<u64> {
    let s = s.strip_prefix("0x").unwrap_or(s);
    let mut value: u64 = 0;
    for c in s.bytes() {
        let digit = match c {
            b'0'..=b'9' => (c - b'0') as u64,
            b'a'..=b'f' => (c - b'a' + 10) as u64,
            b'A'..=b'F' => (c - b'A' + 10) as u64,
            _ => return None, // invalid char
        };
        value = value.checked_mul(16)?.checked_add(digit)?;
    }

    Some(value)
}
