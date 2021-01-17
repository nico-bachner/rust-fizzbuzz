fn main() {
    let mut a = 1;
    while a <= 100 {
        if a % 3 == 0 {
            if a % 5 == 0 {
                println!("FizzBuzz");
            }
            else {
                println!("Fizz");
            }
        }
        else if a % 5 == 0 {
            println!("Buzz");
        }
        else {
            println!("{}", a);
        }
        a += 1;
    }
}
