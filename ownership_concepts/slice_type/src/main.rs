fn main() {
    let mut s = String::from("Hello world");
    let n = s.len();
    let first_word = first_word(&s);
    println!("First word of '{s}' is : '{first_word}'");
    let second_word = second_word(&s,n);
    println!("Second word of '{s}' is : '{second_word}'");
    s.clear();
}
fn first_word(s: &String) -> &str{
    let bytes = s.as_bytes();
    for(i,&item) in bytes.iter().enumerate(){
        if item == b' '{
            //slicing(..i)of string using reference or indices in general
            return &s[..i];
        }
    }
    &s[..]
}
fn second_word(s: &String,n: usize) -> &str{
    let bytes = s.as_bytes();
    for(i,&item) in bytes.iter().enumerate(){
        if item == b' '{
            return &s[(i+1)..n];
        }
    }
    &s[..]
}