# README for ERC20ATON

## Overview

The `ERC20ATON` contract is a custom ERC20 token implementation named "ATON" built on the Stylus SDK. This token is designed to be used within the Arenaton decentralized sports betting platform. It includes features such as commission handling, minting from ETH, and swapping ATON tokens back to ETH.

## Key Functions

### 1. `initialize()`

- **Description:** Initializes the contract and sets the contract deployer as the owner.
- **Access:** Public
- **Returns:** `bool`

### 2. `mint_aton_from_eth()`

- **Description:** Mints ATON tokens from ETH. Only callable by addresses marked as Arenaton engines.
- **Access:** Public, Payable
- **Returns:** `bool`

### 3. `swap(amount: U256)`

- **Description:** Swaps ATON tokens back to ETH. Ensures that the sender has sufficient balance.
- **Access:** Public
- **Returns:** `Result<bool, ATONError>`

### 4. `transfer(to: Address, amount: U256)`

- **Description:** Transfers ATON tokens from the sender to a specified address, handling commission distribution.
- **Access:** Public
- **Returns:** `Result<bool, ATONError>`

### 5. `approve(spender: Address, value: U256)`

- **Description:** Sets the allowance of a spender to transfer tokens on behalf of the sender.
- **Access:** Public
- **Returns:** `bool`

### 6. `transfer_from(from: Address, to: Address, value: U256)`

- **Description:** Transfers tokens from one address to another using the allowance mechanism.
- **Access:** Public
- **Returns:** `Result<bool, ATONError>`

### 7. `update_engine(account: Address, status: bool)`

- **Description:** Updates the status of an address as an Arenaton engine. Only callable by the owner.
- **Access:** Public
- **Returns:** `Result<(), ATONError>`

## Usage Example

```rust
// Assuming you have a deployed instance of ERC20ATON at `contract_address`
let contract = ERC20ATON::new(contract_address);

// Mint ATON tokens from ETH
let mint_tx = contract.mint_aton_from_eth().call();
assert!(mint_tx.is_ok());

// Approve spending allowance
let approve_tx = contract.approve(spender_address, U256::from(1000)).call();
assert!(approve_tx.is_ok());

// Transfer tokens from one address to another
let transfer_tx = contract.transfer_from(from_address, to_address, U256::from(500)).call();
assert!(transfer_tx.is_ok());
```

---