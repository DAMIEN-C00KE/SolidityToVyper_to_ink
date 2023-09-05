// SOLIDITY
// Contract taken from https://solidity-by-example.org/enum/
/*
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

contract Enum {
    // Enum representing shipping status
    enum Status {
        Pending,
        Shipped,
        Accepted,
        Rejected,
        Canceled
    }

    // Default value is the first element listed in
    // definition of the type, in this case "Pending"
    Status public status;

    // Returns uint
    // Pending  - 0
    // Shipped  - 1
    // Accepted - 2
    // Rejected - 3
    // Canceled - 4
    function get() public view returns (Status) {
        return status;
    }

    // Update status by passing uint into input
    function set(Status _status) public {
        status = _status;
    }

    // You can update to a specific enum like this
    function cancel() public {
        status = Status.Canceled;
    }

    // delete resets the enum to its first value, 0
    function reset() public {
        delete status;
    }
}
*/

// ink! RUST
#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod shipping {

    #[ink(storage)]
    pub struct Shipping {
        status: Status,
    }

    #[derive(Debug, scale::Encode, scale::Decode, PartialEq, Eq, Copy, Clone)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
    pub enum Status {
        Pending,
        Shipped,
        Accepted,
        Rejected,
        Canceled,
    }

    impl Default for Status {
        fn default() -> Self {
            Status::Pending
        }
    }

    impl Shipping {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                status: Status::default(),
            }
        }

        #[ink(message)]
        pub fn get(&self) -> Status {
            self.status
        }

        #[ink(message)]
        pub fn set(&mut self, new_status: Status) {
            self.status = new_status;
        }

        #[ink(message)]
        pub fn cancel(&mut self) {
            self.status = Status::Canceled;
        }

        #[ink(message)]
        pub fn reset(&mut self) {
            self.status = Status::Pending;
        }
    }

}
