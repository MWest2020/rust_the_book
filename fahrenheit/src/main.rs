use std::io;

fn main() {
    println!("Welcome to the degrees converter!");
    
    let metric = get_metric();
    let degrees =  get_degrees();

    check_metric(metric, degrees);
}


fn get_metric() -> u32{
    
    println!("Please enter a metric");
    println!("1. Celsius");
    println!("2. Fahrenheit");
    
    let mut metric = String::new();
        
    io::stdin()
    .read_line(&mut metric)
    .expect("Failed to get a metric");
    
    let metric: u32 = metric.trim().parse().expect("That's not a number");
    return metric;
}

fn get_degrees() -> f32 {

    println!("Please enter your degrees");

    let mut degrees = String::new();
    
    io::stdin()
    .read_line(&mut degrees)
    .expect("Failed to read line");

    let degrees: f32 = degrees.trim().parse().expect("That's not a number");
    return degrees;
}

fn celsius_to_fahrenheit(degrees: f32) -> f32{
    degrees as f32 * 1.8 + 32.0
}


fn fahrenheit_to_celsius(degrees:f32) -> f32{
    degrees as f32 / 1.8 - 32.0
}

fn check_metric(metric: u32, degrees: f32){
    if metric == 1 { 
        println!("That's {:.1} degrees in fahrenheit", celsius_to_fahrenheit(degrees))
    } else if metric == 2 {
        println!("That's {:.1} degrees in celsius", fahrenheit_to_celsius(degrees))
    }
}
