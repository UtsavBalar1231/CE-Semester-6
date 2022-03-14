#[allow(dead_code)]
enum Payment {
    Cash(f64),
    Card(String, u8),
    Crypto { currency: String, amount: f64 },
    UPI(UpiData),
}

struct UpiData {
    upi_id: String,
    phone_number: String,
}

fn main() {
    let payment = Payment::Crypto {
        currency: String::from("Bitcoin"),
        amount: 0.00000012,
    };

    match payment {
        Payment::Cash(amount) => println!("Cash Payment: {}", amount),
        Payment::Card(name, amount) => {
            println!("Card Owner: {}, Card Payment: {}", name, amount)
        }
        Payment::Crypto { currency, amount } => {
            println!("Crypto Currency: {}, Amount: {}", currency, amount)
        }
        Payment::UPI(data) => println!(
            "UPI ID: {}, Phone Number: {}",
            data.upi_id, data.phone_number
        ),
    }
}
