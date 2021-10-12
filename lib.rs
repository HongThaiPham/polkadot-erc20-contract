#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod erc20 {
    #[cfg(not(feature = "ink-as-dependency"))]
    #[ink(storage)]
    pub struct Erc20 {
        /// The total supply.
        total_supply: Balance,
        /// The balance of each user.
        balances: ink_storage::collections::HashMap<AccountId, Balance>,
        /// Approval spender on behalf of the message's sender.
        //  ACTION: Add an `allowances` storage item. It should be a
        //         `HashMap` from `(AccountId, AccountId)` to `Balance`
        allowances : ink_storage::collections::HashMap<(AccountId,AccountId), Balance>
    }

    #[ink(event)]
    pub struct Transfer {
    //  ACTION: Create a `Transfer` event with:
    //          * from: Option<AccountId>
    //          * to: Option<AccountId>
    //          * value: Balance
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        value: Balance
    }

    // ACTION: Add an `Approval` event
    //         It should emit the following:
    //         * `owner` as an `AccountId`
    //         * `spender` as an `AccountId`
    //         * `value` as a `Balance`

    #[ink(event)]
    pub struct Approval {
        owner: AccountId,
        spender: AccountId,
        value: Balance
    }

    impl Erc20 {
        #[ink(constructor)]
        pub fn new(initial_supply: Balance) -> Self {
            // ACTION: `set` the total supply to `initial_supply`
            // ACTION: `insert` the `initial_supply` as the `caller` balance
            let _caller = Self::env().caller();
            let mut _balances = ink_storage::collections::HashMap::new();
            _balances.insert(_caller, initial_supply);
           

            // ACTION: Call `Self::env().emit_event` with the `Transfer` event
            //   HINT: Since we use `Option<AccountId>`, you need to wrap accounts in `Some()`
            Self::env().emit_event(
                Transfer {
                    from: None,
                    to: Some(_caller),
                    value: initial_supply
                }
            );
            
            Self {
                total_supply: initial_supply, 
                balances: _balances,
                allowances: ink_storage::collections::HashMap::new(),
            }
        }

        #[ink(message)]
        pub fn total_supply(&self) -> Balance {
            // ACTION: Return the total supply
            self.total_supply
        }

        #[ink(message)]
        pub fn balance_of(&self, owner: AccountId) -> Balance {
            // ACTION: Return the balance of `owner`
            //   HINT: Use `balance_of_or_zero` to get the `owner` balance
            self.balance_of_or_zero(&owner)
        }

        #[ink(message)]
        pub fn approve(&mut self, spender: AccountId, value: Balance) -> bool {
            // ACTION: Get the `self.env().caller()` and store it as the `owner`
            let owner = self.env().caller();
            // ACTION: Insert the new allowance into the `allowances` HashMap
            //   HINT: The key tuple is `(owner, spender)`
            self.allowances.insert((owner, spender), value);
            // ACTION: `emit` the `Approval` event you created using these values
            // ACTION: Return true if everything was successful
            self.env().emit_event(
                Approval {
                    owner: owner,
                    spender: spender,
                    value: value
                }
            );
            true
        }

        #[ink(message)]
        pub fn allowance(&self, owner: AccountId, spender: AccountId) -> Balance {
            // ACTION: Create a getter for the `allowances` HashMap
            //   HINT: Take a look at the getters above if you forget the details
            // ACTION: Return the `allowance` value
            self.allowance_of_or_zero(&owner, &spender)
        }

        #[ink(message)]
        pub fn transfer_from(&mut self, from: AccountId, to: AccountId, value: Balance) -> bool {
            // ACTION: Get the allowance for `(from, self.env().caller())` using `allowance_of_or_zero`
            let _caller = self.env().caller();
            let allowance = self.allowance_of_or_zero(&from, &_caller);
            // ACTION: `if` the `allowance` is less than the `value`, exit early and return `false`
            if allowance < value {
                return false;
            }
            // ACTION: `insert` the new allowance into the map for `(from, self.env().caller())`
            let transfer_result = self.transfer_from_to(from, to, value);
             // Check `transfer_result` because `from` account may not have enough balance
            //   and return false.
            if !transfer_result {
                return false
            }
            // ACTION: Finally, call the `transfer_from_to` for `from` and `to`
            self.allowances.insert((from, _caller), allowance - value);
            // ACTION: Return true if everything was successful
            true
        }

        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, value: Balance) -> bool {
            // ACTION: Call the `transfer_from_to` with `from` as `self.env().caller()`
            self.transfer_from_to(self.env().caller(), to, value)
        }

        fn transfer_from_to(&mut self, from: AccountId, to: AccountId, value: Balance) -> bool {
            // ACTION: Get the balance for `from` and `to`
            //   HINT: Use the `balance_of_or_zero` function to do this
            let _from_balance = self.balance_of_or_zero(&from);
            let _to_balance = self.balance_of_or_zero(&to);
            // ACTION: If `from_balance` is less than `value`, return `false`
            if _from_balance<value{
                return false;
            }
                
            // ACTION: Insert new values for `from` and `to`
            //         * from_balance - value
            //         * to_balance + value
            // self.balances.entry(from).and_modify(|_from_balance| *_from_balance -= value);
            // self.balances.entry(to).and_modify(|_to_balance| *_to_balance += value).or_insert(value);
            self.balances.insert(from, _from_balance - value);
            self.balances.insert(to, _to_balance + value);

            // ACTION: Call `self.env().emit_event` with the `Transfer` event
            //   HINT: Since we use `Option<AccountId>`, you need to wrap accounts in `Some()`
            self.env().emit_event(
                Transfer {
                    from: Some(from),
                    to: Some(to),
                    value: value
                }
            );
            // ACTION: Return `true`
            true
        }

        fn balance_of_or_zero(&self, owner: &AccountId) -> Balance {
            // ACTION: `get` the balance of `owner`, then `unwrap_or` fallback to 0
            // ACTION: Return the balance
            *self.balances.get(owner).unwrap_or(&0)
        }

        fn allowance_of_or_zero(&self, owner: &AccountId, spender: &AccountId) -> Balance {
            // ACTION: `get` the `allowances` of `(owner, spender)` and `unwrap_or` return `0`.
            // If you are new to Rust, you may wonder what's the deal with all the asterisks and
            // ampersends.
            //
            // In brief, using `&` if we want to get the address of a value (aka reference of the
            // value), and using `*` if we have the reference of a value and want to get the value
            // back (aka dereferencing).
            // To read more: https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html
            *self.allowances.get(&(*owner, *spender)).unwrap_or(&0)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        use ink_lang as ink;

        #[ink::test]
        fn new_works() {
            let contract = Erc20::new(777);
            assert_eq!(contract.total_supply(), 777);
        }

        #[ink::test]
        fn balance_works() {
            let contract = Erc20::new(100);
            assert_eq!(contract.total_supply(), 100);
            assert_eq!(contract.balance_of(AccountId::from([0x1; 32])), 100);
            assert_eq!(contract.balance_of(AccountId::from([0x0; 32])), 0);
        }

        #[ink::test]
        fn transfer_works() {
            let mut contract = Erc20::new(100);
            assert_eq!(contract.balance_of(AccountId::from([0x1; 32])), 100);
            assert!(contract.transfer(AccountId::from([0x0; 32]), 10));
            assert_eq!(contract.balance_of(AccountId::from([0x0; 32])), 10);
            assert!(!contract.transfer(AccountId::from([0x0; 32]), 100));
        }

        #[ink::test]
        fn transfer_from_works() {
            let mut contract = Erc20::new(100);
            assert_eq!(contract.balance_of(AccountId::from([0x1; 32])), 100);
            contract.approve(AccountId::from([0x1; 32]), 20);
            contract.transfer_from(AccountId::from([0x1; 32]), AccountId::from([0x0; 32]), 10);
            assert_eq!(contract.balance_of(AccountId::from([0x0; 32])), 10);
        }

        #[ink::test]
        fn allowances_works() {
            let mut contract = Erc20::new(100);
            assert_eq!(contract.balance_of(AccountId::from([0x1; 32])), 100);
            contract.approve(AccountId::from([0x1; 32]), 200);
            assert_eq!(contract.allowance(AccountId::from([0x1; 32]), AccountId::from([0x1; 32])), 200);

            assert!(contract.transfer_from(AccountId::from([0x1; 32]), AccountId::from([0x0; 32]), 50));
            assert_eq!(contract.balance_of(AccountId::from([0x0; 32])), 50);
            assert_eq!(contract.allowance(AccountId::from([0x1; 32]), AccountId::from([0x1; 32])), 150);

            assert!(!contract.transfer_from(AccountId::from([0x1; 32]), AccountId::from([0x0; 32]), 100));
            assert_eq!(contract.balance_of(AccountId::from([0x0; 32])), 50);
            assert_eq!(contract.allowance(AccountId::from([0x1; 32]), AccountId::from([0x1; 32])), 150);
        }
    }
}

