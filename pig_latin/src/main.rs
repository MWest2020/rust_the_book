use std::io;

fn main() {
    
    let input = get_input();
    println!("{:?}" , input_to_oink(&input));
}


fn get_input() -> String{
    println!("Welcome to pig latin converter. Enter your soon to be oink");
    
    let mut input = String::new();

    io::stdin()
    .read_line(&mut input)
    .expect("Ehm, I didn get that. Failed to read line");

    //need to parse ?

    return input;
}

fn input_to_oink(input_string: &str) -> Option<String>{
    
    if input_string.is_empty(){
        return Some(String::new());
    }
    
    // vowels to byte
    let vowels = b"aeiou";

    //
    let first_byte = input_string.as_bytes()[0];
    if !first_byte.is_ascii_alphabetic(){
        return None;
    }
    
    // let mut multiple_words: Vec<String> = Vec::new();
    // let mut words: Vec<&str> = input_string.split_whitespace().collect();



    // need more imporvement for whole sentences and want any char beyond  vowel to be appended.
    if vowels.contains(&first_byte.to_ascii_lowercase()) {
        return Some(format!("{}-ay", input_string));
    } else { 
        let (first_letter, suffix) = input_string.split_at(1);
        Some(format!("{}-{}ay", suffix, first_letter))
    }

 
}