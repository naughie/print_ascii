use std::fs::File;
use std::io::Read;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn print(mut buf: impl Read) {
    let byte = &mut [0u8];
    while let Ok(n) = buf.read(byte) {
        if n == 0 {
            break;
        } else {
            print!("0x{:02x} ", byte[0]);
            if byte[0] == 0x0a || byte[0] == 0x0d {
                println!();
            }
        }
    }
    println!();
}

fn print_file(fname: &str) -> Result<()> {
    let f = File::open(fname)?;
    print(f);
    Ok(())
}

fn print_stdin() -> Result<()> {
    print(std::io::stdin());
    Ok(())
}

fn main() -> Result<()> {
    let mut args = std::env::args();
    args.next();
    if args.size_hint() == (0, Some(0)) {
        print_stdin()
    } else {
        for arg in args {
            println!("{}:", &arg);
            print_file(&arg)?;
        }
        Ok(())
    }
}
