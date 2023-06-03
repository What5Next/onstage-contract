use near_sdk::{env, near_bindgen, require, Promise, Balance};
use near_sdk::{AccountId};
use near_sdk::collections::{UnorderedMap};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::U128;

const CODE : &[u8] = include_bytes!("./../../target/wasm32-unknown-unknown/release/character_nft.wasm");

const INITIAL_BALANCE: Balance = 3_000_000_000_000_000_000_000_000;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct NFTFactory {
    nfts: UnorderedMap<u64, AccountId>,
    owner: AccountId
}

impl Default for NFTFactory{
    fn default()-> Self {
        Self{
            nfts: UnorderedMap::new(b"d"),
            owner: env::predecessor_account_id()
        }
    }
}

#[near_bindgen]
impl NFTFactory{
    pub fn create_new_nft_contract(&self, prefix: AccountId){
        let signer = env::predecessor_account_id();
        require!(self.owner == signer, "Owner only create new nft");

        let sub_account_id = AccountId::new_unchecked(
            format!("{}.{}", prefix, env::current_account_id())
        );

        Promise::new(sub_account_id.clone())
            .create_account()
            .add_full_access_key(env::signer_account_pk())
            .transfer(INITIAL_BALANCE)
            .deploy_contract(CODE.to_vec())
            .then(
                Self::ext(env::current_account_id())
                .after_create_new_nft_contract(sub_account_id.clone())
            );
    }

    #[private]
    pub fn after_create_new_nft_contract(&mut self, sub_account_id: AccountId){
        let current_length = self.nfts.len();
        self.nfts.insert(&current_length, &sub_account_id);
    }

    pub fn get_nfts_contract_accounts(&self)-> Vec<AccountId>{
        let nfts_vec = self.nfts.to_vec();
        nfts_vec.into_iter().map(
            |nft| {
                let (_index, account) = nft;
                return account;
            })
            .collect()
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod test{
    use super::*;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::{testing_env, VMContext};

    fn get_context(is_view: bool) -> VMContext{
        VMContextBuilder::new()
            .signer_account_id("alice_near".parse().unwrap())
            .is_view(is_view)
            .build()
    }

    #[test]
    pub fn deploy_new_nft_contract(){
        let context = get_context(true);
        testing_env!(context);

        let mut nftFactory = NFTFactory::default();
        let sub_account_id: AccountId = "alice_near".parse().unwrap();

        let before_len = nftFactory.nfts.len();
        println!("Before Len: {}", before_len);
        
        nftFactory.create_new_nft_contract("sub".parse().unwrap());
        
        // let after_len = nftFactory.nfts.len();
        // println!("After Len: {}", after_len);
        
        // assert_eq!(1,1);
    }
}
