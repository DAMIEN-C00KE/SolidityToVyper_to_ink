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
