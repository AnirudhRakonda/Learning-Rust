fn main() {
    let mut s1 = String::from("Hello");
    s1.push_str(", world!");
    println!("{s1}");
//{
    //this doent work because in rust right after we duplicate the s value s goes out of scope i.e; drop method is called to free up space
    // let s2 = s;
    // println!("{s} , world");
    //but for i32 it works because it is stored in stack memory where as string is stored in heap as length and capacity are unknown an beginning
//}
    let s2 = s1.clone();
    println!("s1: {s1} , s2: {s2}"); 
    let (s2 , len) =  cal_len(s1);
    println!("The len of '{s2}' is {len} ");

    let _str1 = gives_owenership(); 
    let str2 =  String::from("hello");
    let _str3 = takes_and_gives_back(str2);
}
fn gives_owenership() -> String{
    let some_string = String::from("yours");
    some_string
}
fn takes_and_gives_back(a_string: String) -> String{
    a_string
}
fn cal_len(s:String) -> (String,usize){
    let len = s.len();
    (s,len)
}


