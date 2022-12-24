use std::io;
use std::collections::HashMap;

fn main() {
    
    // declare HashMap
    let mut number_list =  Vec::new();
    
    println!("Welcome to median and mode!");
    println!("How long should the list be?");
    
    let mut counter: u32  = 0;
    let list_length = get_list_length();

    while counter < list_length {
        println!("Enter a number");
        let mut number = String::new();
        
        // get user input
        io::stdin()
        .read_line(&mut number)
        .expect("failed to read line");
                
        let number: u32  = number.trim().parse().expect("That's not a number!");
        number_list.push(number);
        
        counter+= 1;
    }
            
    if number_list.len() % 2 != 0 { 
        println!("The median is: {}", get_odd_median(&mut number_list))
    };

    if number_list.len() % 2  == 0 {
        println!("The median is:{}", get_even_median(&mut number_list));
    }

    println!("The mode is: {} ", get_mode(&mut number_list));
}


fn get_list_length() -> u32 {
    let mut list_length = String::new();
        
        // get user input
        io::stdin()
        .read_line(&mut list_length)
        .expect("failed to read line");
        
        let list_length: u32  = list_length.trim().parse().expect("That's not a number!");
    return list_length;
}

fn get_odd_median( list: &mut Vec<u32>) -> u32 {
    
    list.sort();
    let median  = (&list.len()) / 2;
    return list[median]; 
    
}

fn get_even_median( list: &mut Vec<u32>) -> f32 {
    list.sort();
    let median = 0_f32;
    // need to figure this one out. thinking next literally getting the mid values and divide those
    // let mut median = 0.5 * (&list.len() /2 -1) + list.len() /2; 
    return median;
}

fn get_mode(vec: &mut Vec<u32>) -> u32{
    let mut count_map = HashMap::new();

    for number in vec { 
        // push the items from vec to map
        // whereas entry is the value and or_insert the occurence 
        *count_map.entry(*number).or_insert(0) += 1;
    }

    let mut mode: u32 = 0;
    let mut mode_key: u32 = 0;
    for (key,value) in count_map.iter(){
        if *value > mode {
            mode_key = *key;
            mode = *value;
        }
    }

    println!("{:?}", &count_map);
    return mode_key;
    
}