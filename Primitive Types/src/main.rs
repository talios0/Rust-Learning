use std::mem;

fn main() {
    //variables are constant by default (immutable)

    //unsigned (Positive numbers)
    //signed (Negative and Positive numbers)
    let _a:u8 = 123; // 8 bits (1 byte)

    let mut b:i8 = 0; //Signed integer
    println!("b = {}",b);
    b = 25;
    println!("b = {}",b);

    let mut c = 123456789; //Rust will automatically assign the variable as a 32-bit signed integer
    println!("c = {}, size = {} bytes", c, mem::size_of_val(&c));
    c = -123456789;
    println!("c = {}, size = {} bytes", c, mem::size_of_val(&c));

    let z:isize = 123; // Integral data type that corresponds to the size of the address (64-bit system - 64-bit value, 32 bit system - 32-bit value)
    let size_of_z = mem::size_of_val(&z);
    println!("z = {}, takes up {} bytes, {}-bit OS", z, size_of_z, size_of_z * 8); // Multipled by 8 to convert from bytes to bits

    let d = 'x';
    println!("d = {}, size = {} bytes", d, mem::size_of_val(&d));

    let e = 2.5; // Double-precision, 64 bits (f64)
    println!("e = {}, size = {} bytes", e, mem::size_of_val(&e));

    //boolean
    let g = false;
    println!("g = {}, size = {} bytes", g, mem::size_of_val(&g));
    
    let _f = 4>0; // true

}
