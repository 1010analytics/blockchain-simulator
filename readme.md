# Blockchain Practice Project

## Overview

This Rust Blockchain Simulator is a practice project designed to introduce me to the world of blockchain technology. It serves as a platform to consolidate my Rust programming skills and to understand blockchain fundamentals. The project is structured in such a way that it can evolve; as I learn more advanced Rust features, I can integrate them to expand the blockchain's capabilities.

## Current Implementation

So far, the project simulates a simple blockchain that:

- Handles basic transactions between a sender and a receiver with a specified amount.
- Stores these transactions in blocks.
- Implements a basic proof-of-work (PoW) system to secure block creation using hashing.
- Allows dynamic addition of blocks to the blockchain during runtime through user input.

### Key Components

- **Transaction:** Represents a transfer of cryptocurrency from a sender to a receiver.
- **Block:** Contains a list of transactions, a reference to the previous block's hash, and its own hash calculated via SHA-256.
- **Blockchain:** Manages a sequence of blocks and ensures integrity through cryptographic hashes.

## How It Works

1. Users can input transaction details including sender, receiver, and amount.
2. Each transaction is placed into a new block.
3. A proof-of-work algorithm secures each block before adding it to the blockchain.
4. Users can interactively add more transactions and observe the blockchain's growth.

## Next Steps

- **Hashing and PoW Enhancement:** Improve the hashing mechanism to include additional block attributes like timestamps and transaction counts.
- **Security Features:** Integrate digital signatures to authenticate transaction origins.
- **Network Capabilities:** Simulate a peer-to-peer network to manage the blockchain across multiple nodes.
- **Persistence:** Implement mechanisms to save and load the blockchain's state from disk.
- **User Interface:** Develop a more sophisticated interface for easier interaction with the blockchain.

## Running the Project

Ensure you have Rust installed and configured. Clone the repository, navigate to the project directory, and run:

```bash
cargo run
```
