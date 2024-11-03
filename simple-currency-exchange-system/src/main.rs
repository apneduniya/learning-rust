enum Currency {
    USD(f64),
    INR(f64)
}

impl Currency {
    fn balance(&self) -> f64 {
        match *self {
            Currency::USD(amount) => amount,
            Currency::INR(amount) => amount
        }
    }

    fn alt_currency_value(&self, exchange_rate: f64) -> f64 {
        match self {
            Currency::INR(amount) => amount / exchange_rate,
            Currency::USD(amount) => amount * exchange_rate
        }
    }

    fn currency_symbol(&self) -> String {
        match self {
            Currency::INR(_) => String::from("INR"),
            Currency::USD(_) => String::from("USD")
        }
    }

}

fn main() {
    const USD_TO_INR_RATE: f64 = 83.00; // 1 USD = 83 INR

    let user_inr = Currency::INR(300.00);
    let user_usd = Currency::USD(10.00);

    println!("User balance: {} {}", user_inr.balance(), user_inr.currency_symbol());
    println!(
        "Equivalent in USD: {:.2} USD",
        user_inr.alt_currency_value(USD_TO_INR_RATE)
    );
    println!();

    println!("User balance: {} {}", user_usd.balance(), user_usd.currency_symbol());
    println!(
        "Equivalent in INR: {:.2} INR",
        user_usd.alt_currency_value(USD_TO_INR_RATE)
    );
}
