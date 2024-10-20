//using generic function
// fn main() {
//    let width = 30;
//    let height = 70;
//     println!("The area of rectangle is: {}" ,area(width,height));
// }
// fn  area(width: u32, height: u32)->u32{
//     width*height
// }


//using tuples
// fn main(){
//     let rect = (30,70);
//     println!("The area of rectangle is: {}" ,area(rect));
// }
// fn area(dimensions: (u32,u32)) -> u32{
//     dimensions.0*dimensions.1
// }


//using structs
struct Rectangle {
    width:u32,
    height:u32,
}
fn main(){
    let rect = Rectangle{
        width:50,
        height:70,
    };
    println!("The area of rectangle is: {}" ,area(&rect));
}
fn area(rectangle:&Rectangle) -> u32{
    rectangle.width * rectangle.height
}
