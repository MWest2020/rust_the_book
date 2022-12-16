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
    let mut fib_sequence = vec![0u128, 1u128, 2u128];

    let mut prev  = fib_sequence[1];
    let mut last = fib_sequence[2];
    let mut next : u128;
    
    // will loop as long as fib request 
    let mut counter = 3u128;
    while counter != fib as u128{

        next = prev + last;
        fib_sequence.push(next);
        // set the variables for next iteration
        prev = last;
        last = next;

        counter +=1;
    }

    println!("The {}th number in the Fibonacci sequence is : {:?}", fib, fib_sequence.last().unwrap());
}
