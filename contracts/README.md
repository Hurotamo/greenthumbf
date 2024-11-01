# GreenThumb Smart Contracts

This directory contains the smart contracts for the GreenThumb decentralized application, built using Rust and deployed on the Solana blockchain. Contracts include managing tokens, subscriptions, referrals, and marketplace features.

Instructions to Deploy the Smart Contracts
Set up your Solana environment: Ensure Solana CLI is installed and configured.

Build the Smart Contracts:

bash
Copy code
# Navigate to the contracts directory
cd contracts

# Build the contracts
cargo build-bpf --manifest-path=./Cargo.toml --bpf-out-dir=./target/deploy
Deploy the Smart Contracts:

bash
Copy code
# Use Solana CLI to deploy the smart contract
solana program deploy ./target/deploy/greenthumb_contracts.so
