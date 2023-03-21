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

    let mut is_correct = true; // creates and assigns a mutable variable and infers type to be bool
    println!("is_correct: {}", is_correct);
    is_correct = !is_correct;
    println!("now is_correct is: {}", is_correct);

    // char Unicode character, 32 bits wide ⌛️ (U+231B)
    let mut martini_glass = '\u{1F378}'; // char
    println!("martini_glass: {}", martini_glass);
    println!("size of martini_glass: {}", size_of_val(&martini_glass));

    // arrays
    let arr1 = [1,2,3,4,5]; // inferred as [i32; 5]
    println!("arr1 index 1 is: {}", arr1[1]);

    let mut arr2: [u8; 3];
    arr2 = [1,2,3];
    println!("size of arr2 is: {}", size_of_val(&arr2));
    // iterate with .iter()
    for n in arr2.iter() {
        println!("arr2 contains: {}", n);
    }
    arr2[1] = 9; // needs to be mutable (mut) to change elements
    // another way to iterate
    for i in 0..arr2.len()  {
        println!("Now, arr2[{}] contains: {}", i, arr2[i]);
    }
}


