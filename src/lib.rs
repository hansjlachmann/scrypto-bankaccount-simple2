use scrypto::prelude::*;

#[blueprint]
mod bankaccount {
    struct BankAccount {
        account_balance: Vault,
        xrd_balance: Vault,
    }

    impl BankAccount {
        pub fn instantiate() -> Global<BankAccount> {
            let initial_balance: Bucket = ResourceBuilder::new_fungible(OwnerRole::None)
            .divisibility(DIVISIBILITY_NONE)
            .metadata(metadata!(
                init {
                    "name" => "Euro", locked;
                    "symbol" => "EUR", locked;
                    "description" => "Euro Coins", locked;
                }
            ))
            .mint_initial_supply(1000)
            .into();
            Self {
                account_balance: Vault::with_bucket(initial_balance),
                xrd_balance:  Vault::new(XRD)  //XRD is a global constant in radix_common
            }
            .instantiate()
            .prepare_to_globalize(OwnerRole::None)
            .globalize()
        }    

        pub fn get_balance(&self)   {
            info!("bank balance is: {}", self.account_balance.amount());
            info!("XRD balance is: {}", self.xrd_balance.amount());
            
        }

        pub fn withdraw(&mut self,withdraw_amount: Decimal) -> Bucket {
            self.account_balance.take(withdraw_amount)                        
        }

        pub fn deposit(&mut self, bucket_deposit: Bucket) {
            if bucket_deposit.resource_address() == XRD {
                self.xrd_balance.put(bucket_deposit);    
            }
            else {
                self.account_balance.put(bucket_deposit);
            }
        }

        pub fn burn_from_vault(&mut self, burn_amount: Decimal) {
            let tokens_to_burn = self.account_balance.take(burn_amount);   
            tokens_to_burn.burn();
        }
    }
}