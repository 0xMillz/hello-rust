use std::mem::size_of_val;
fn main() {
    println!("Hello Rust!");
    let mut number_1 = -133; // creates and assigns a mutable variable and infers type to be i32
    println!("number_1: {}", number_1);
    println!("size of number_1: {}", size_of_val(&number_1)); // &<variable_name> is the address in mem of the var
    number_1 = -1;
    println!("now number_1 is: {}", number_1);

    let mut number_2: u16 = 133; // creates and assigns a mutable variable explicitly as unsigned 16 bits (logs warning b/c it is never reassigned)
    println!("number_2: {}", number_2);
    println!("size of number_2: {}", size_of_val(&number_2)); // &<variable_name> is the address in mem of the var
}


