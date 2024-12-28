// Allow `cargo stylus export-abi` to generate a main function.
#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

// Modules and imports

use stylus_sdk::prelude::*;
use stylus_sdk::{
    alloy_primitives::{Address, U256},
    alloy_sol_types::sol,
    call::transfer_eth,
    call::{call, Call},
    contract, evm, msg,
};
// use alloy_sol_macro::sol;
// `Counter` will be the entrypoint.
sol_storage! {
    #[entrypoint]
    pub struct ERC20ATON {


        address owner;
                            /// Maps users to balances
        mapping(address => uint256) balances;
        /// Maps users to a mapping of each spender's allowance
        mapping(address => mapping(address => uint256)) allowances;
        /// The total supply of the token
        uint256 total_supply;

        mapping(address => bool) arenaton_engine;

        address vault_address;



    }





}

sol_interface! {
        interface IVault {

    function playerCommission(address player) external view returns (uint256);

    function clearCommission(address player) external;

        }
}
sol! {


    // ATON
    event CommissionAccumulate(uint256 indexed amount, uint256 indexed newAccPerToken, uint256 indexed totalCommission);
    event EngineUpdated(address indexed account, bool status);
    error Zero(address account);


        // Access Control
    event EngineRoleGranted( address indexed account, address indexed sender);
    event EngineRoleRevoked( address indexed account, address indexed sender);


    // Ownership
    event OwnershipTransferred(address indexed previous_owner, address indexed new_owner);
    error UnauthorizedAccount(address account);

     // ERC20
    event Transfer(address indexed from, address indexed to, uint256 value);
    event Approval(address indexed owner, address indexed spender, uint256 value);
    error InsufficientBalance(address from, uint256 have, uint256 want);
    error InsufficientAllowance(address owner, address spender, uint256 have, uint256 want);

}

/// Represents the ways methods may fail.
#[derive(SolidityError)]
pub enum ATONError {
    InsufficientBalance(InsufficientBalance),
    Zero(Zero),
    InsufficientAllowance(InsufficientAllowance),
    UnauthorizedAccount(UnauthorizedAccount),
}

#[public]
impl ERC20ATON {
    /// Immutable token name
    pub fn name() -> String {
        "ATON Stylus".into()
    }

    /// Immutable token symbol
    pub fn symbol() -> String {
        "ATON".into()
    }

    /// Immutable token decimals
    pub fn decimals() -> u8 {
        18u8
    }

    /// Total supply of tokens
    pub fn total_supply(&self) -> U256 {
        self.total_supply.get()
    }

    /// Balance of `address`
    pub fn balance_of(&self, owner: Address) -> U256 {
        self.balances.get(owner)
    }

    /// Transfers `value` tokens from `from` to `to`
    /// (msg::sender() must be able to spend at least `value` tokens from `from`)
    pub fn transfer_from(
        &mut self,
        from: Address,
        to: Address,
        value: U256,
    ) -> Result<bool, ATONError> {
        // Check msg::sender() allowance
        let mut sender_allowances = self.allowances.setter(from);
        let mut allowance = sender_allowances.setter(msg::sender());
        let old_allowance = allowance.get();
        if old_allowance < value {
            return Err(ATONError::InsufficientAllowance(InsufficientAllowance {
                owner: from,
                spender: msg::sender(),
                have: old_allowance,
                want: value,
            }));
        }

        // Decreases allowance
        allowance.set(old_allowance - value);

        // Calls the internal transfer function
        self._transfer(from, to, value)?;

        Ok(true)
    }

    /// Approves the spenditure of `value` tokens of msg::sender() to `spender`
    pub fn approve(&mut self, spender: Address, value: U256) -> bool {
        self.allowances.setter(msg::sender()).insert(spender, value);
        evm::log(Approval {
            owner: msg::sender(),
            spender,
            value,
        });
        true
    }

    /// Returns the allowance of `spender` on `owner`'s tokens
    pub fn allowance(&self, owner: Address, spender: Address) -> U256 {
        self.allowances.getter(owner).get(spender)
    }

    fn owner(&self) -> Address {
        self.owner.get()
    }

    fn transfer_ownership(&mut self, new_owner: Address) -> Result<(), ATONError> {
        self._only_owner()?;

        if new_owner.is_zero() {
            return Err(ATONError::UnauthorizedAccount(UnauthorizedAccount {
                account: Address::ZERO,
            }));
        }

        self._transfer_ownership(new_owner);

        Ok(())
    }
    pub fn initialize(&mut self) -> bool {
        if self.owner.get() != Address::ZERO {
            // Access the value using .get()
            return false;
        }
        self.owner.set(msg::sender());
        true
    }

    pub fn set_vault(&mut self, vault_address: Address) -> bool {
        if self.vault_address.get() != Address::ZERO {
            // Access the value using .get()
            return false;
        }
        self.vault_address.set(vault_address);
        true
    }

    pub fn vault(&self) -> Address {
        self.vault_address.get()
    }

    pub fn transfer(&mut self, to: Address, amount: U256) -> Result<bool, ATONError> {
        let caller = msg::sender();

        // let vault_contract = IVault::new(self.vault_address.get());

        // let mut from_commission = self._player_commission(&vault_contract, caller);
        // let mut to_commission = self._player_commission(&vault_contract, to);

        // let _to = to;
        // let _caller = caller;
        // let _owner = self.owner.get();

        // if caller == contract::address() || to == contract::address() {
        //     let owner_commission = self._player_commission(&vault_contract, to);

        //     if to == contract::address() {
        //         let _to = _owner;
        //         to_commission += owner_commission;
        //     }

        //     if caller == contract::address() {
        //         let _caller = _owner;
        //         from_commission += owner_commission;
        //     }
        // }
        // let vault_address = self.vault_address.get();

        //     let _ = self._perform_transfer(vault_address, _to, to_commission);
        //     let _ = self._perform_transfer(vault_address, _caller, from_commission);

        // let _ = self._clear_commission(&vault_contract, to);
        // let _ = self._clear_commission(&vault_contract, caller);

        // if caller == contract::address() || to == contract::address() {
        //     let _ = self._clear_commission(&vault_contract, contract::address());
        // }

        // Perform the transfer
        self._transfer(caller, to, amount) // 100
            .map(|_| true)
            .map_err(|_| {
                ATONError::InsufficientBalance(InsufficientBalance {
                    from: msg::sender(),
                    want: amount,
                    have: self.balances.get(msg::sender()),
                })
            })
    }

    #[payable]
    pub fn mint_aton_from_eth(&mut self) -> bool {
        // if self.arenaton_engine.get(msg::sender()) == false {
        //     return false;
        // }

        // let _ = self.mint(msg::sender(), msg::value());
        // Increasing balance
        let mut balance = self.balances.setter(msg::sender());
        let new_balance = balance.get() + msg::value();
        balance.set(new_balance);

        // Increasing total supply
        self.total_supply
            .set(self.total_supply.get() + msg::value());

        // Emitting the transfer event
        evm::log(Transfer {
            from: Address::ZERO,
            to: msg::sender(),
            value: msg::value(),
        });

        true
    }

    pub fn swap(&mut self, amount: U256) -> Result<bool, ATONError> {
        let sender = msg::sender();

        let contract_balance = contract::balance();

        if amount == U256::from(0)
            || self.balances.get(sender) < amount
            || contract_balance < amount
        {
            return Err(ATONError::Zero(Zero { account: sender })); // Add the error struct
        }
        let _ = transfer_eth(sender, amount);

        Ok(true)
    }

    /// Allows the owner to update the status of `arenaton_engine` for a specific address.
    pub fn update_engine(&mut self, account: Address, status: bool) -> Result<(), ATONError> {
        // Ensure only the owner can call this function
        self._only_owner()?;

        // Update the `arenaton_engine` mapping
        let mut engine = self.arenaton_engine.setter(account);
        engine.set(status);

        // Emit an event (optional, but recommended for transparency)
        evm::log(EngineUpdated { account, status });

        Ok(())
    }
}

// Private Functions
impl ERC20ATON {
    // Ownable
    pub fn _only_owner(&self) -> Result<(), ATONError> {
        let account = msg::sender();
        if self.owner.get() != account {
            return Err(ATONError::UnauthorizedAccount(UnauthorizedAccount {
                account,
            }));
        }

        Ok(())
    }

    pub fn _transfer_ownership(&mut self, new_owner: Address) {
        let previous_owner = self.owner.get();
        self.owner.set(new_owner);
        evm::log(OwnershipTransferred {
            previous_owner,
            new_owner,
        });
    }

    // Token Management
    pub fn _transfer(&mut self, from: Address, to: Address, value: U256) -> Result<(), ATONError> {
        // Decreasing sender balance
        let mut sender_balance = self.balances.setter(from);
        let old_sender_balance = sender_balance.get();
        if old_sender_balance < value {
            return Err(ATONError::InsufficientBalance(InsufficientBalance {
                from,
                have: old_sender_balance,
                want: value,
            }));
        }
        sender_balance.set(old_sender_balance - value);

        // Increasing receiver balance
        let mut to_balance = self.balances.setter(to);
        let new_to_balance = to_balance.get() + value;
        to_balance.set(new_to_balance);

        // Emitting the transfer event
        evm::log(Transfer { from, to, value });
        Ok(())
    }

    // Helper function to calculate commission
    fn _player_commission(&mut self, vault: &IVault, account: Address) -> U256 {
        vault
            .player_commission(Call::new_in(self), account)
            .map_err(|_| ATONError::Zero(Zero { account }))
            .unwrap_or_default()
    }

    // Helper function to perform a transfer
    pub fn _perform_transfer(
        &mut self,
        from: Address,
        to: Address,
        amount: U256,
    ) -> Result<(), ATONError> {
        self._transfer(from, to, amount).map_err(|_| {
            ATONError::InsufficientBalance(InsufficientBalance {
                from,
                want: amount,
                have: self.balances.get(from),
            })
        })
    }

    // Helper function to clear commission in the vault
     fn _clear_commission(&mut self, vault: &IVault, account: Address) -> Result<(), ATONError> {
        vault
            .clear_commission(Call::new_in(self), account)
            .map_err(|_| ATONError::Zero(Zero { account }))
    }

    pub fn _pay_commissions(&mut self, to: Address, from: Address) -> Result<(), ATONError> {
        let vault_contract = IVault::new(self.vault_address.get());

        let from_commission = self._player_commission(&vault_contract, from);
        let to_commission = self._player_commission(&vault_contract, to);

        let _to = to;
        let _from = from;
        let _owner = self.owner.get();
        let mut owner_commission = U256::from(0);
        if from == contract::address() || to == contract::address() {
            owner_commission = self._player_commission(&vault_contract, to);
        }
        let vault_address = self.vault_address.get();

        let _ = self._perform_transfer(vault_address, _to, to_commission);
        let _ = self._perform_transfer(vault_address, _from, from_commission);

        let _ = self._clear_commission(&vault_contract, to);
        let _ = self._clear_commission(&vault_contract, from);

        if from == contract::address() || to == contract::address() {
            let _ = self._perform_transfer(vault_address, _owner, owner_commission);

            let _ = self._clear_commission(&vault_contract, _owner);
        }
        return Ok(());
    }
}
