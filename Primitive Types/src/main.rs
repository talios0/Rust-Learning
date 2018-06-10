use std::mem;

mod sh;

const MEANING_OF_LIFE:u8 = 42; // no fixed address
static mut Z:i32 = 123; // This variable is unsafe. There will be a problem if more than a single operation tries to write/read to/from the address at a single time.

fn main() {
    sh::stack_and_heap();
    println!("{}",MEANING_OF_LIFE);
    unsafe { // Tells Rust I know this is unsafe and it will be handled it accordingly
        println!("{}",Z);
    }

    primitive_types();
    operators();
    scope_and_shadowing();
}

fn primitive_types() {
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

fn operators() {
    // arithmetic
    let mut a = 2+3*4;
    println!("{}", a);
    a +=11; //-- and ++ do not exit in Rust
    a -=2;

    println!("Remainder of {} / {} = {}", a, 3, a%3); // % is modulo, takes the remainer of a/3

    let a_cubed = i32::pow(a, 3); // a raised to the third power
    println!("{} cubed is {}", a, a_cubed);

    let b = 2.5;
    let b_cubed = f64::powi(b, 3); // i represents that b raised to the third will be rounded to an integer
    let b_to_pi = f64::powf(b,std::f64::consts::PI);
    println!("{} cubed = {}, {}^pi = {}", b, b_cubed, b, b_to_pi);

    //bitwise operators (integers only)
    let c = 1 | 2; // | or & and ^and ^ exclusive or (xor) ! not
    // 0001  or 0010 = 11 = 3 (decimal)
    println!("1|2 = {}", c);
    let two_to_10 = 1 << 10;
    println!("2^10 = {}", two_to_10);

    //logical operators
    let _pi_less_4 = std::f64::consts::PI < 4.0; // true
}

fn scope_and_shadowing() {
    let a = 123;

    { //scope - variables of the same name can be declared inside the scope. This will overwrite the outside scope while inside the scope. A change to a mutable variable will stay.
        let a = 777;
        let b = 456;
        println!("inside, b = {}", b);
        println!("inside, a = {}", a);
    }
    println!("outside, a = {}", a);
}