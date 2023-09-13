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
