// Contracts taken from ink! documentation
// https://use.ink/ink-vs-solidity
// 
// - Updated pub fn set_bool
// 
// BEFORE:
/*  pub fn set_bool(&mut self, new_bool: bool) -> bool {
       let bool_changed = true;

    if self.the_bool == new_bool{
       bool_changed = false;
    } else {
       bool_changed = true;
    }

    self.the_bool = new_bool;

    self.env().emit_event(UpdatedBool {
        the_bool: new_bool
    });

    // return
    bool_changed
 }
*/
// Updated due to bool_changed being declared as mutable but then
// reassigned with immutable variables.
//
#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod basic_contract {
    #[ink(storage)]
    pub struct BasicContract {
        the_bool: bool, // class members become struct fields
    }

    #[ink(event)]
    pub struct UpdatedBool {
        #[ink(topic)] // -> indexed
        the_bool: bool,
    }

    impl BasicContract {
        #[ink(constructor)]
        pub fn new(the_bool: bool) -> Self {
            assert!(the_bool == true, "the_bool must start as true");
            Self { the_bool }
        }

        #[ink(message)] // functions become struct implementations
        pub fn set_bool(&mut self, new_bool: bool) -> bool {
            let bool_changed = if self.the_bool == new_bool {
                false
            } else {
                true
            };

            self.the_bool = new_bool;

            self.env().emit_event(UpdatedBool {
                the_bool: new_bool
            });

            // return
            bool_changed
        }
    }
}
