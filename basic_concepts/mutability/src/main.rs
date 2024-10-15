//  error because the variable x is not mutable...
//    let x = 5;
//    println!("The value of x is: {x}");
//    x = 6;
//    println!("The value of x is: {x}");




fn main() {
   let mut x = 5;
   println!("X value(before): {x}");
   x = 6;
   println!("X value(after): {x}");

}
