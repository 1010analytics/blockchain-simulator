use std::collections::HashMap;
use std::io;
use std::io::Write;
use sha2::{Sha256, Digest};
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug, Clone)]
struct Transaction {
    sender: String,
    receiver: String,
    amount: f64,
    payload: String, 
}

#[derive(Debug, Clone)]
struct SmartContract {
    code: String, 
}

impl SmartContract {
    fn execute(&self, data: &Transaction) -> f64 {
        
        println!("Executing contract code: {}", self.code);
        data.amount * 2.0 
    }
}

#[derive(Debug, Clone)]
struct Block {
    previous_hash: String,
    transactions: Vec<Transaction>,
    hash: String,
    nonce: u64,
    validator: String,
}

impl Block {
    fn new(transactions: Vec<Transaction>, previous_hash: String, validator: String) -> Block {
        let mut block = Block {
            previous_hash,
            transactions,
            hash: String::new(),
            nonce: 0,
            validator,
        };
        block.hash = block.calculate_hash();
        block
    }

    fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(&self.previous_hash);
        hasher.update(&self.nonce.to_string());
        hasher.update(&self.validator);
        for transaction in &self.transactions {
            hasher.update(&transaction.sender);
            hasher.update(&transaction.receiver);
            hasher.update(&transaction.amount.to_string());
            hasher.update(&transaction.payload);
        }
        format!("{:x}", hasher.finalize())
    }
}

#[derive(Debug)]
struct Blockchain {
    blocks: Vec<Block>,
    stakes: HashMap<String, f64>,
    contracts: Vec<SmartContract>,
}

impl Blockchain {
    fn new() -> Blockchain {
        let genesis_block = Block::new(vec![], "0".to_string(), "genesis_validator".to_string());
        let mut blockchain = Blockchain {
            blocks: vec![genesis_block],
            stakes: HashMap::new(),
            contracts: vec![SmartContract { code: "return input * 2;".to_string() }], 
        };
        blockchain.stakes.insert("genesis_validator".to_string(), 1000.0);
        blockchain
    }

    fn add_block(&mut self, mut new_block: Block) {
        if self.validate_block(&new_block) {
            self.blocks.push(new_block);
        }
    }

    fn validate_block(&self, block: &Block) -> bool {
        self.stakes.get(&block.validator).map_or(false, |&stake| stake > 100.0)
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
    let blockchain = Arc::new(Mutex::new(blockchain));

    loop {
        let blockchain = Arc::clone(&blockchain);
        println!("Enter a new transaction details:");
        let sender = read_input("Enter sender: ");
        let receiver = read_input("Enter receiver: ");
        let amount = read_input("Enter amount: ");
        let payload = read_input("Enter payload or contract call: ");
        let amount: f64 = amount.parse().expect("Expected a number for amount");

        thread::spawn(move || {
            let transaction = Transaction {
                sender,
                receiver,
                amount,
                payload,
            };

            let block = Block::new(vec![transaction], blockchain.lock().unwrap().blocks.last().unwrap().hash.clone(), "some_validator_address".to_string());
            blockchain.lock().unwrap().add_block(block);
        });

        let cont = read_input("Add another transaction? (yes/no): ");
        if cont.to_lowercase() != "yes" {
            break;
        }
    }

    println!("Blockchain: {:?}", *blockchain.lock().unwrap());
}
