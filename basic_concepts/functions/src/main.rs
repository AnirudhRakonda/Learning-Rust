fn main() {
    println!("From main function");
    //function calling and passing args
    another_function(6,'i');

    //let x = (let y = 6); this statement does'nt work because (y = 6) does'nt return any value

    let y = {
        let x = five();
        x+8
    };
     //{
    //     let x = 3;
    //     x+1
    // };is an expression
    println!("Value of y is: {y}");
}
fn another_function(x:i32,c:char){
    println!("From another function and arg is: {x} and {c}");
}
fn five() -> i32{
    5
    //i32 in fn sign is return value
}
