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

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle{
    fn calculate_area(&self)-> u32{
        self.width *  self.height
    }
}


fn main(){

    let rectangle1 = Rectangle{
        width: 30,
        height: 50
    };

    println!("The area of {:#?} is {} whatevers, since you didn't specify metrics.", rectangle1, rectangle1.calculate_area());
}


// fn print_rectangle(rectangle: &Rectangle){
//     println!("The area of {:#?} is {} whatevers, since you didn't specify metrics.", rectangle, rectangle.calculate_area(&rectangle));
// }






// use std::io;

// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32
// }
// fn main(){

//     let rectangle1 = Rectangle{
//         width: get_width(),
//         height: get_height()
//     };

//     print_rectangle(&rectangle1);
// }

// fn get_width() -> u32{
    
//     println!("Please enter a width");
    
//     let mut width = String::new();
        
//     io::stdin()
//     .read_line(&mut width)
//     .expect("Failed to get a metric");
    
//     let width: u32 = width.trim().parse().expect("That's not a number");
//     return width;
// }

// fn get_height() -> u32{
    
//     println!("Please enter a height");
    
//     let mut heigth = String::new();
        
//     io::stdin()
//     .read_line(&mut heigth)
//     .expect("Failed to get a metric");
    
//     let heigth: u32 = heigth.trim().parse().expect("That's not a number");
//     return heigth;
// }


// fn calculate_area(rectangle: &Rectangle)-> u32{
//     rectangle.width *  rectangle.height
// }

// fn print_rectangle(rectangle: &Rectangle){
//     println!("The area of {:#?} is {} whatevers, since you didn't specify metrics.", rectangle, calculate_area(&rectangle));
// }