use std::io;

fn main() {
    
    intro();

    let mut metric = String::new();

    
    io::stdin()
    .read_line(&mut metric)
    .expect("Failed to get a metric");
    
    let metric: u32 = metric.trim().parse().expect("That's not a number");


    println!("Please enter your degrees");
    
    let mut degrees = String::new();
    
    io::stdin()
    .read_line(&mut degrees)
    .expect("Failed to read line");

    let degrees: u32 = degrees.trim().parse().expect("That's not a number");

    println!("Your input is :  {degrees} degrees {metric}");

    check_metric(metric, degrees);


}


fn intro(){
    println!("Welcome to the degrees converter!");
    println!("Please enter a metric");
    println!("1. Celsius");
    println!("2. Fahrenheit");
}


fn celsius_to_fahrenheit(degrees: u32) -> f32{
    degrees as f32 * 1.8 + 32.0
}


fn fahrenheit_to_celsius(degrees :u32) -> f32{
    degrees as f32 / 1.8 - 32.0
}

fn check_metric(metric: u32, degrees: u32){
    if metric == 1 { 
        println!("That's { } in fahrenheit", celsius_to_fahrenheit(degrees))
    } else if metric == 2 {
        println!("That's { } in celsius", fahrenheit_to_celsius(degrees))
    }
}

