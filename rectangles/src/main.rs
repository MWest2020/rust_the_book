// fn main() {
//     let width1: u32 = 30;
//     let height1: u32 = 50;

//     println!("The area is {} whatevers, since you didn't specify metrics.", calculate_area(width1, height1));

// }

// fn calculate_area(width: u32, height: u32) -> u32{
//     width * height
// }

//with tuples (comment out above)
// before reading further, my guess is this sucks, because you 
// don't get to know which side is which.

// fn main(){
//     let tuples_rectangle = (30, 50);
    
//     println!("The area is {} whatevers, since you didn't specify metrics.", calculate_area(tuples_rectangle));

// }

// fn calculate_area(dimensions:(u32, u32))-> u32{
//     dimensions.0 * dimensions.1
// }

// with structs (lemme try)
struct Rectangle {
    width: u32,
    height: u32
}
fn main(){

    let rectangle1 = Rectangle{
        width: 30,
        height: 50
    };


    println!("The area is {} whatevers, since you didn't specify metrics.", calculate_area(&rectangle1));
}
fn calculate_area(rectangle: &Rectangle)-> u32{
    rectangle.width *  rectangle.height
}