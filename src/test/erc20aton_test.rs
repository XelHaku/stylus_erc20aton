// tests/erc20aton_test.rs

#[cfg(test)]
mod tests {
    use crate::Erc20Aton;
    use stylus_sdk::{
        alloy_primitives::{address, Address, U256},msg,
        prelude::*,
    };
    // If you are not actually using these two, comment them out:
    // use crate::test::constants::env_vars::{get_env_vars, EnvVars};
    const VAULT_ADDRESS: &str = "0x7e32B54800705876D3B5CfBC7d9C226A211F7C1A";
const ARENATON_ENGINE: &str = "0x7e32B54800705876D3B5CfBC7d9C226A211F7C1A";
    #[motsu::test]
    fn erc20params(contract: Erc20Aton) {
        let name = contract.name();
        let symbol = contract.symbol();
        let decimals = contract.decimals();
        let vault_address = contract.vault_address();
        println!(
            "\n\nName: {}, Symbol: {}, Decimals: {}, Vault Address: {}",
            name, symbol, decimals, vault_address
        );
        assert_eq!(decimals, 18u8);
        assert_eq!(name, "ATON Stylus");
        assert_eq!(symbol, "ATON");
        assert!(vault_address.is_zero());
    }

    #[motsu::test]
    fn initialize(contract: Erc20Aton) {
        assert!(contract.initialize());
        let owner = contract.owner();
        println!("\n\nOwner: {}", owner);
        assert!(!owner.is_zero());
    }
    #[motsu::test]
    fn set_vault(contract: Erc20Aton) {
        // Instead of parse_checksummed, just parse hex ignoring checksums:
        let vault_address = VAULT_ADDRESS;
        let parsed: Address = vault_address
            .parse()
            .expect("Should parse valid hex address");

        // Compare to a known-lowercase literal.
        // No `0x` prefix and all-lowercase for `address!()`
        let expected = address!("7e32b54800705876d3b5cfbc7d9c226a211f7c1a");

        assert_eq!(parsed, expected);

        // If desired, set it in the contract
        contract.set_vault(parsed);
        assert_eq!(contract.vault_address(), parsed);
    }
#[motsu::test]
fn mint_aton_debug_test(contract: Erc20Aton) {
    // Get the address of the sender to check balances and interact with the contract
    let sender = msg::sender();

    // Retrieve the initial total supply of the token
    let mut _total_supply = contract.total_supply();

    // Check the sender's initial balance
    let mut _balance = contract.balances.get(sender);

    // Log the initial total supply to the console for debugging purposes
    println!("\n\nTotal Supply: {}", _total_supply);

    // Assert that the initial total supply is 0 (as expected in a fresh contract)
    assert!(_total_supply == U256::from(0));

    // Assert that the sender's initial balance is also 0
    assert!(_balance == U256::from(0));

    // Call the `mint_aton_debug` function to mint 10 new tokens
    // This function is expected to increase the total supply and update the sender's balance
    assert!(contract.mint_aton_debug(U256::from(10)));

    // Retrieve the updated total supply after minting tokens
    _total_supply = contract.total_supply();

    // Log the updated total supply to the console for debugging purposes
    println!("\n\nTotal Supply2: {}", _total_supply);

    // Assert that the total supply has increased by the expected amount (10 tokens)
    assert!(_total_supply == U256::from(10));

    // Retrieve the sender's updated balance after minting tokens
    _balance = contract.balances.get(sender);

    // Assert that the sender's balance has increased by the minted amount (10 tokens)
    assert!(_balance == U256::from(10));
}



#[motsu::test]
fn update_new_arenaton_engine(contract: Erc20Aton) {
    let  sender = msg::sender();
    assert!(!contract.is_engine(sender));



        assert!(contract.initialize());

    assert!(sender == contract.owner());



assert!(contract.update_engine(sender, true).is_ok());


    assert!(contract.is_engine(sender));
}


}
