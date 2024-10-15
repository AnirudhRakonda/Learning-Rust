fn main() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("const is immutable {THREE_HOURS_IN_SECONDS}");

    //shadowing

    let x = 7;
    println!("x value: {x}");
    let x = x + 1;
    {
        let x = x*2;
        println!("Inner scope value of x: {x}");
    }
    println!("Outer scope value of x: {x}");

}
