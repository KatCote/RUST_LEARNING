fn main() {

    use std::arch::asm;

    let mut port: u8 = 255;
    let mut result: u8 = 1;

    println!("{} {}", &result, &port);

    unsafe{
        asm!(
            "add al, bl;",
            inout("al") result,
            inout("bl") port,
        );
    }

    println!("{} {}", &result, &port);
}