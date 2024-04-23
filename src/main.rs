use std::io;
use std::io::Write; 
use sha2::{Sha256, Digest}; 

#[derive(Debug)]
struct Transaction {
    sender: String,
    receiver: String,
    amount: f64,
}

#[derive(Debug)]
struct Block {
    previous_hash: String,
    transactions: Vec<Transaction>,
    hash: String,
    nonce: u64,
}

impl Block {
    fn new(transactions: Vec<Transaction>, previous_hash: String) -> Block {
        let mut block = Block {
            previous_hash,
            transactions,
            hash: String::new(),
            nonce: 0,
        };
        block.hash = block.calculate_hash();
        block
    }

    fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(&self.previous_hash);
        hasher.update(&self.nonce.to_string());
        for transaction in &self.transactions {
            hasher.update(&transaction.sender);
            hasher.update(&transaction.receiver);
            hasher.update(&transaction.amount.to_string());
        }
        format!("{:x}", hasher.finalize())
    }

    fn mine(&mut self, difficulty: usize) {
        let target = std::iter::repeat("0").take(difficulty).collect::<String>();
        while &self.hash[..difficulty] != target {
            self.nonce += 1;
            self.hash = self.calculate_hash();
        }
    }
}

#[derive(Debug)]
struct Blockchain {
    blocks: Vec<Block>,
}

impl Blockchain {
    fn new() -> Blockchain {
        let genesis_block = Block::new(vec![], "0".to_string());
        let mut blockchain = Blockchain {
            blocks: vec![genesis_block],
        };
        blockchain.blocks[0].mine(4); 
        blockchain
    }

    fn add_block(&mut self, mut new_block: Block) {
        new_block.mine(4); 
        self.blocks.push(new_block);
    }
}

fn read_input(prompt: &str) -> String {
    let mut input = String::new();
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

fn main() {
    let mut blockchain = Blockchain::new();

    loop {
        println!("Enter a new transaction details:");
        let sender = read_input("Enter sender: ");
        let receiver = read_input("Enter receiver: ");
        let amount = read_input("Enter amount: ");

        let amount: f64 = amount.parse().expect("Expected a number for amount");

        let transaction = Transaction {
            sender,
            receiver,
            amount,
        };

        let block = Block::new(vec![transaction], blockchain.blocks.last().unwrap().hash.clone());
        blockchain.add_block(block);

        let cont = read_input("Add another transaction? (yes/no): ");
        if cont.to_lowercase() != "yes" {
            break;
        }
    }

    println!("Blockchain: {:?}", blockchain);
}
