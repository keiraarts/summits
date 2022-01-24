#[derive(Debug)]
enum Country {
    Canada,
}

enum Coin {
    Penny,
    Nickel,
    Quarter(Country),
}

fn main() {
    println!("Hello, world!");
    value_in_cents(Coin::Quarter(Country::Canada));

    let default_coin = Coin::Quarter(Country::Canada);
    if let Coin::Quarter(state) = default_coin {
        println!("The coin is a quarter {:#?}", state)
    }

    stats(Some(5));
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn stats(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(t) => Some(t + 1),
    }
}
