use std::io;
fn main() {
    loop{
        println!("Hi enter your nth number to get it from the Fibonacci Sequence:");
        let mut n = String::new(); 
        io::stdin()
            .read_line(&mut n)
            .expect("Failed to read line");
        let n: u32 = n.trim().parse().expect("Please Enter a Number");
        let number = fibbonaci(n);
        println!("The number at {} is: {}", n, number);
        let mut choice = String::new(); 
        println!("Would you like to find another number(y/n): ");
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
        let choice: char = choice.trim().parse().expect("Enter y or n"); 
        if choice == 'n'{
            break
        }
    }
    println!("Program Exit");
   
}
fn fibbonaci(n: u32) -> u32{
    if n <= 1 {
        return n
    }
    else{
        return fibbonaci(n-1) + fibbonaci(n-2);
    }
}
