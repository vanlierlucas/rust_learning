// program to get the nth number of the fibonacci sequence
use std::io;

fn main() {
    loop {
        println!("Please, enter wich nth of the fibonacci sequence you want");
        let mut nth = String::new();
        io::stdin()
            .read_line(&mut nth)
            .expect("We couldn't read the line.");

        let nth = match nth.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("The programm will be stopped because you didn't enter an integer.");
                continue;
            }
        };

        let f = fibonacci(nth);
        println!("The {nth}th number of the fibonacci sequence is {f}");
    }
}

fn fibonacci(n: u32) -> u128 {
    let (mut a, mut b) = (1, 1);

    if n == 0 || n == 1 {
        return 1; // Not necessary to iterate
    }

    for _number in 1..n - 1 {
        (a, b) = (b, a + b);
    }

    b
}
