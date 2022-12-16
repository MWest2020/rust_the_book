use std::io;

fn main() {
    println!("Enter the desired fib number");
    
    let mut fib = String::new(); 
    io::stdin()
    .read_line(&mut fib)
    .expect("Failed to read line");

    // from String to u128
    let fib : usize = fib.trim().parse().expect("That's not a number");

    // place to store the fib sequence
    let mut fib_sequence = Vec::new();

    let mut counter = 3u128;
    let start = 0u128;
    let mut prev  = 1u128;
    let mut last = 2u128;
    let mut next : u128 = 0u128;
    let mut current_fib = 0u128;

    fib_sequence.push(start);
    fib_sequence.push(prev);
    fib_sequence.push(last);
    
    while counter != fib as u128{

        println!("{} {} ", prev, last);
        next = prev + last;
        fib_sequence.push(next);
        println!("{:?}", fib_sequence);
        prev = last;

        last = next;
        
        counter +=1;
    }

    println!("The {}th number in the Fibonacci sequence is : {:?}", fib, fib_sequence.last().unwrap());
}
