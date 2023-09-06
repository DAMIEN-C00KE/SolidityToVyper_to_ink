// SOLIDITY
// Contract taken from https://solidity-by-example.org/app/ether-wallet/
/*
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

contract EtherWallet {
    address payable public owner;

    constructor() {
        owner = payable(msg.sender);
    }

    receive() external payable {}

    function withdraw(uint _amount) external {
        require(msg.sender == owner, "caller is not owner");
        payable(msg.sender).transfer(_amount);
    }

    function getBalance() external view returns (uint) {
        return address(this).balance;
    }
}
*/

// ink! Rust
#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod ether_wallet {
    
    #[ink(storage)]
    pub struct EtherWallet {
        owner: AccountId,
    }

    impl EtherWallet {
        #[ink(constructor)]
        pub fn new() -> Self {
            let caller = Self::env().caller();
            Self {
                owner: caller,
            }
        }

        #[ink(message, payable)]
        pub fn deposit(&mut self) {}

        #[ink(message)]
        pub fn withdraw(&mut self, amount: Balance) {
            let caller = self.env().caller();
            assert_eq!(caller, self.owner, "caller is not owner");
            self.env().transfer(caller, amount).expect("Failed to transfer");
        }

        #[ink(message)]
        pub fn get_balance(&self) -> Balance {
            self.env().balance()
        }
    }
}
