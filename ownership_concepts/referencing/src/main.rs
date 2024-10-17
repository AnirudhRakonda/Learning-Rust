fn main() {
    let mut s1 = String::from("Hello!");
    let len = cal_len(&s1);
    //&s1 --> references to location of tring s1 and cal_len uses that reference to find len
    println!("Length of '{s1}' is: {len}");
    change(&mut s1);
    println!("New s1 is: {s1}");

    //we cannot borrow s as mutable(&mut s1) more than once at a time.use scope if needed
    //to avoid data race
    let r1 = &s1; // no problem
    let r2 = &s1; // no problem
    //let r3 = &mut s; // BIG PROBLEM

    println!("{}, {}", r1, r2);
    let r3 = &mut s1;
    println!("{r3}")


}
fn cal_len(s: &String) -> usize{
    s.len()
}
fn change(some_str: &mut String){
    some_str.push_str(", world");
}
