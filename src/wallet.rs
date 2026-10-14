pub struct Wallet {
    pub address: String,
    pub balance: i64,
}

impl Wallet {
    pub fn new(address: String, balance: i64) -> Self {
        Self { address, balance }
    }

    pub fn display(&self) {
        println!("Wallet address: {}", self.address);
        println!("Balance: {} DEMO coins", self.balance);
    }
}

pub fn run() {
    println!("\n=== Rust Wallet Lab ===");

    let my_wallet = Wallet::new(
        String::from("demo_wallet_001"),
        50000,
    );

    my_wallet.display();
}