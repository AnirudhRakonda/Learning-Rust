use std::io;
fn main() {

    // -(2^(n) - 1) to 2^(n - 1) - 1 =>signed
    //0 to 2^n - 1 => unsigned

    let x = 2.0 ; //f64
    let y : f32 = 3.0; //f32

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    //integer division truncates toward zero to the nearest integer
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;


    //The Tuple Type
    let tup :(i32,f64,u8) = (500,6.4,1);
    //accesing tuple type
    let t1= tup.0;
    let t2= tup.1;
    let t3 = tup.2;
 
    //arrays
    let a = [1,2,3,4,5];
    //to specify type and length
    let b: [u32;5] = [1,2,3,4,5];

    let c = [3;5];
    //this gives c = [3, 3, 3, 3, 3]

    println!("Enter index.");
    let mut index = String::new();

    io::stdin().read_line(&mut index).expect("Failed to read");

    let index: usize = index.trim().parse().expect("Not an integer");

    let elem = a[index];
    println!("The value of the {index} element is: {elem}");


}
