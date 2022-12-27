use std::io;

fn main() {
    
    let input = get_input();
    println!("{:?}" , input_to_oink(input));
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

fn input_to_oink(input_string: String){
    
    let mut multiple_words: Vec<String> = Vec::new();
    const VOWELS: [&str; 5] = ["a", "e", "i", "o", "u"];


    let mut words: Vec<&str> = input_string.split_whitespace().collect();



    println!("{:?}", words);

}