mod data_type;
mod wallet;
mod blockchain;

fn main() {
    println!("Starting Rust Practice Project");

    data_type::run();
    wallet::run();
    blockchain::run();

    println!("\nAll exercises completed!");
}