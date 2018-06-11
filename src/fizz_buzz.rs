fn main() {
  for number in 1..100 {
    match number {
      number if (number % 3 == 0) && (number % 5 == 0) => println!("FizzBuzz"),
      number if number % 3 == 0 => println!("Fizz"),
      number if number % 5 == 0 => println!("Buzz"),
      _ => println!("{}", number)
    }
  }
}