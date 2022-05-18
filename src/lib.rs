// use std::collections::HashMap;
// use std::thread::AccessError;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{
    env, near_bindgen, PanicOnDefault,
};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]

pub struct Contract{
    pub rand : Vec<u8>,
}


#[near_bindgen]
impl Contract{

    // This function generates the random number.
    pub fn rand(&mut self) -> Vec<u8> {
       let random_number = env::random_seed();
       random_number
    }

    // This function displays the random number
    fn get_random_number(&self) -> Vec<u8>{
        let num = self.rand.clone();
        num
    }

}
