fn cpy(to: &mut u16, from: &mut u16){
    use std::arch::asm;

    unsafe{
        asm!(

            "xor eax, eax",
            "add eax, edx",

            inout("eax") *to,
            inout("edx") *from,
        );
    }
}
fn main() {

    let mut port: u16 = 10;
    let mut result: u16 = 0;

    println!("\n\tTO\tFROM\t\n\t{}\t{}", result, port);

    cpy(&mut result, &mut port);

    println!("\t{}\t{}\n", result, port);
}
