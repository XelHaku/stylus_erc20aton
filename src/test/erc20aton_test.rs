// tests/erc20aton_test.rs

#[cfg(test)]
mod tests {
    use crate::{test::constants::env_vars::get_env_vars, Erc20Aton};
    use stylus_sdk::{
        alloy_primitives::{address, Address, U256},msg,
        prelude::*,
    };
    // If you are not actually using these two, comment them out:
    // use crate::test::constants::env_vars::{get_env_vars, EnvVars};
    const VAULT_ADDRESS: &str = "0x7e32B54800705876D3B5CfBC7d9C226A211F7C1A";

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


//         #[motsu::test]
//     fn mint_aton(contract: Erc20Aton) {
// let  val = msg::value();
// print!("Value: {}", val);
//         assert!(contract.mint_aton());
//         contract.mint_aton();

//     }


    // Uncomment below tests if you are actually using them in your Erc20Aton:
    /*
    #[motsu::test]
    fn can_set_number(contract: Erc20Aton) {
        let new_number = U256::from(10);
        contract.set_number(new_number);
        let number = contract.number();
        assert_eq!(number, new_number);
    }

    #[motsu::test]
    fn can_increment_number(contract: Erc20Aton) {
        contract.set_number(U256::from(5));
        contract.increment();
        let number = contract.number();
        assert_eq!(number, U256::from(6));
    }

    #[motsu::test]
    fn can_add_number(contract: Erc20Aton) {
        contract.set_number(U256::from(5));
        contract.add_number(U256::from(3));
        let number = contract.number();
        assert_eq!(number, U256::from(8));
    }

    #[motsu::test]
    fn can_mul_number(contract: Erc20Aton) {
        contract.set_number(U256::from(5));
        contract.mul_number(U256::from(2));
        let number = contract.number();
        assert_eq!(number, U256::from(10));
    }
    */
}
