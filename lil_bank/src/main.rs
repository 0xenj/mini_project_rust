use std::io;
use std::io::Write;

#[derive(Debug)]
struct Coin {
  value: i32,   // value in cents
  amount: i32   // number
}

impl Coin {
    // &self - instance method
    fn total(&self) -> i32 {
      self.value * self.amount
    }
}

fn get_input() -> i32 {
    let mut input_text = String::new();
    io::stdin()
      .read_line(&mut input_text)
      .expect("Failed to read input");
    let num: i32 = input_text.trim().parse().ok().expect("That's no number!");
  
    num
}

fn print_line(text: &str) {
    print!("{}", text);
    io::stdout().flush().unwrap();
}

fn cents_to_euro(cents: i32) -> String {
    format!("{:.2}€", cents as f32 / 100f32)
}

fn get_total(coins: Vec<Coin>) -> String {
    let mut total: i32 = 0;
  
    for coin in coins {
      total = total + coin.total();
    }
  
    cents_to_euro(total)
}

fn main() {
    let mut coins: Vec<Coin> = Vec::new();
    println!("Enter number of coins: ");
    print_line("2€: ");
    coins.push(Coin { value: 200, amount: get_input() });
  
    print_line("1€: ");
    coins.push(Coin { value: 100, amount: get_input() });
  
    print_line("50c: ");
    coins.push(Coin { value: 50, amount: get_input() });
  
    print_line("20c: ");
    coins.push(Coin { value: 20, amount: get_input() });
  
    print_line("10c: ");
    coins.push(Coin { value: 10, amount: get_input() });
  
    print_line("5c: ");
    coins.push(Coin { value: 5, amount: get_input() });
  
    println!("\nTotal: {}", get_total(coins));
  }

  
  #[test]
  fn test_coins() {
    let mut coins: Vec<Coin> = Vec::new();
  
    coins.push(Coin { value: 200, amount: 1 });
    coins.push(Coin { value: 100, amount: 3 });
    coins.push(Coin { value: 50, amount: 5 });
    coins.push(Coin { value: 20, amount: 2 });
    coins.push(Coin { value: 10, amount: 1 });
    coins.push(Coin { value: 5, amount: 15 });
  
    assert!(get_total(coins) == "8.75€");
  }

