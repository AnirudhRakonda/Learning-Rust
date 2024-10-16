fn main() {
    //general use of if ,else if,else keywords
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    //using if in a let statement
    //if and else should have same d_type
    let condition = true;
    let element = if condition { 5 } else {6};
    println!("The value of element is: {element}");


    //flow control in loops

    let mut counter = 0;
    let res = loop {
        counter += 1;
        if counter == 10{
            break counter*2;
        }
    };
    println!("The result is: {res}");

    //we can lable loops using 'lable_name

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    //while loop
    let mut num = 3;
    while num != 0{
        println!("{num}");
        num -= 1;
    }
    println!("Launch");

    //for in

    for count_down in (1..4).rev(){
        println!("{count_down}")
    }
    println!("Launch");
}