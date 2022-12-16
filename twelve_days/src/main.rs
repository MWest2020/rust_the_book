fn main() {

    let lyrics = vec![
        "And a partridge in a pear tree",
        "Two turtle-doves",
        "Three French hens", 
        "Four calling birds",
        "Five golden rings (five golden rings)",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "12 drummers drumming",
        "My true love sent to me"
    ];


    let mut counter = 1u32;

    // run 12 times
    for index in 0..12{
        println!(" ");
        println!("On the {} day of Christmas", get_number_spelled_out(index +1));
        println!("My lover sent to me");
        for number in (0.. counter).rev(){
            println!("{}", lyrics[number as usize]);
        }
        if counter == 12 { println!("{:?}", lyrics[0])};
        counter += 1;
        println!(" ");
    }
}



fn get_number_spelled_out(number: u32) -> &'static str{
  return match number {
      1 => "first",
      2 => "second",
      3 => "third",
      4 => "fourth",
      5 => "fifth",
      6 => "sixth",
      7 => "seventh",
      8 => "eight",
      9 => "ninth",
      10 => "tenth",
      11 => "eleventh",
      12 => "twelveth",
      0_u32 | 13_u32..=u32::MAX => "How in the world?"
  };
  
}