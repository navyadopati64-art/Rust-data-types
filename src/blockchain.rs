use sha2::{Digest, Sha256};

#[derive(Debug)]
pub struct Block {
    pub index: u32,
    pub data: String,
    pub previous_hash: String,
    pub hash: String,
}

impl Block {
    pub fn new(
        index: u32,
        data: String,
        previous_hash: String,
    ) -> Self {
        let hash = Self::calculate_hash(
            index,
            &data,
            &previous_hash,
        );

        Self {
            index,
            data,
            previous_hash,
            hash,
        }
    }

    fn calculate_hash(
        index: u32,
        data: &str,
        previous_hash: &str,
    ) -> String {
        let input = format!(
            "{}|{}|{}",
            index, data, previous_hash
        );

        let digest = Sha256::digest(input.as_bytes());

        format!("{:x}", digest)
    }
}

pub struct Blockchain {
    pub blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        let genesis = Block::new(
            0,
            String::from("Genesis block"),
            String::from("0"),
        );

        Self {
            blocks: vec![genesis],
        }
    }

    pub fn add_block(&mut self, data: String) {
        let previous = self.blocks.last().unwrap();

        let new_block = Block::new(
            previous.index + 1,
            data,
            previous.hash.clone(),
        );

        self.blocks.push(new_block);
    }

    pub fn is_valid(&self) -> bool {
        for i in 0..self.blocks.len() {
            let block = &self.blocks[i];

            let expected_hash = Block::calculate_hash(
                block.index,
                &block.data,
                &block.previous_hash,
            );

            // Check whether the block's data was changed.
            if block.hash != expected_hash {
                return false;
            }

            // Check whether this block links to the
            // previous block correctly.
            if i > 0 {
                let previous = &self.blocks[i - 1];

                if block.previous_hash != previous.hash {
                    return false;
                }
            }
        }

        true
    }
}

pub fn run() {
    println!("\n=== Mini Blockchain ===");

    let mut blockchain = Blockchain::new();

    blockchain.add_block(
        String::from("Alice pays Bob 10 demo coins"),
    );

    blockchain.add_block(
        String::from("Bob pays Charlie 3 demo coins"),
    );

    for block in &blockchain.blocks {
        println!("\nBlock index: {}", block.index);
        println!("Data: {}", block.data);
        println!("Previous hash: {}", block.previous_hash);
        println!("Hash: {}", block.hash);
    }

    println!(
        "\nBlockchain valid: {}",
        blockchain.is_valid()
    );

    // Simulate someone changing stored transaction data.
    blockchain.blocks[1].data =
        String::from("Alice pays Bob 1000 demo coins");

    println!(
        "Valid after tampering: {}",
        blockchain.is_valid()
    );
}