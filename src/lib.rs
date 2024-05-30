#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

#[global_allocator]
static ALLOC: mini_alloc::MiniAlloc = mini_alloc::MiniAlloc::INIT;

use stylus_sdk::{abi::Bytes, alloy_primitives::Address, call::RawCall, prelude::*};

#[solidity_storage]
#[entrypoint]
pub struct CallContract;

#[external]
impl CallContract {
    pub fn execute(&self, address: Address, data: Bytes) -> Bytes {
        let result = RawCall::new().call(address, data.to_vec().as_slice());

        result.unwrap().into()
    }
}
