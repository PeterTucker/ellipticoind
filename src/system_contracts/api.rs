use crate::{
    state::State,
    system_contracts::{self, is_system_contract},
    transaction::Transaction,
};
use ellipticoin::Address;
use serde::de::DeserializeOwned;
use serde_cbor::value::Value;
#[cfg(test)]
use std::collections::BTreeMap;
#[cfg(test)]
use std::convert::TryInto;

pub struct NativeAPI<'a> {
    pub state: &'a mut State,
    pub transaction: Transaction,
    pub address: ([u8; 32], String),
    pub sender: [u8; 32],
    pub caller: Address,
}

impl<'a> ellipticoin::MemoryAPI for NativeAPI<'a> {
    fn get(&mut self, key: &[u8]) -> Vec<u8> {
        self.state.get_memory(&self.address, key)
    }

    fn set(&mut self, key: &[u8], value: &[u8]) {
        self.state.set_memory(&self.address, key, value)
    }
}

impl<'a> ellipticoin::StorageAPI for NativeAPI<'a> {
    fn get(&mut self, key: &[u8]) -> Vec<u8> {
        self.state.get_storage(&self.address, key)
    }

    fn set(&mut self, key: &[u8], value: &[u8]) {
        self.state.set_storage(&self.address, key, value)
    }
}

impl<'a> ellipticoin::API for NativeAPI<'a> {
    fn contract_address(&self) -> ([u8; 32], String) {
        self.address.clone()
    }
    fn sender(&self) -> [u8; 32] {
        self.sender.clone()
    }
    fn caller(&self) -> Address {
        self.caller.clone()
    }
    fn call<D: DeserializeOwned>(
        &mut self,
        legislator: [u8; 32],
        contract_name: &str,
        function_name: &str,
        arguments: Vec<Value>,
    ) -> Result<D, Box<ellipticoin::wasm_rpc::error::Error>> {
        let mut transaction = self.transaction.clone();
        // self.caller = Address::Contract(SYSTEM_ADDRESS.to_vec(), "Ellipticoin".to_string());
        transaction.contract_address = (legislator, contract_name.to_string());
        transaction.arguments = arguments;
        transaction.function = function_name.to_string();
        let return_value: serde_cbor::Value = if is_system_contract(&transaction) {
            system_contracts::run2(self, transaction).into()
        } else {
            // transaction.complete((CONTRACT_NOT_FOUND.clone()).into(), transaction.gas_limit).into()
            panic!();
        };
        Ok(serde_cbor::from_slice(&serde_cbor::to_vec(&return_value).unwrap()).unwrap())
    }
}
#[cfg(test)]
pub struct TestState {
    pub memory: BTreeMap<Vec<u8>, Vec<u8>>,
    pub storage: BTreeMap<Vec<u8>, Vec<u8>>,
    pub memory_changeset: BTreeMap<Vec<u8>, Vec<u8>>,
    pub storage_changeset: BTreeMap<Vec<u8>, Vec<u8>>,
}
#[cfg(test)]
impl TestState {
    pub fn new() -> Self {
        Self {
            memory: BTreeMap::new(),
            storage: BTreeMap::new(),
            storage_changeset: BTreeMap::new(),
            memory_changeset: BTreeMap::new(),
        }
    }
}
#[cfg(test)]
pub struct TestAPI<'a> {
    pub state: &'a mut TestState,
    pub address: ([u8; 32], String),
    pub transaction: Transaction,
    pub sender: [u8; 32],
    pub caller: Address,
}

#[cfg(test)]
impl<'a> TestAPI<'a> {
    pub fn new(state: &'a mut TestState, sender: [u8; 32], address: ([u8; 32], String)) -> Self {
        let transaction = Transaction {
            sender,
            ..Default::default()
        };
        Self {
            state,
            address,
            transaction: transaction.clone(),
            caller: Address::PublicKey(transaction.sender),
            sender: transaction.sender.try_into().unwrap(),
        }
    }
}

#[cfg(test)]
impl<'a> ellipticoin::MemoryAPI for TestAPI<'a> {
    fn get(&mut self, key: &[u8]) -> Vec<u8> {
        self.state.memory.get(key).unwrap_or(&vec![]).to_vec()
    }

    fn set(&mut self, key: &[u8], value: &[u8]) {
        self.state.memory.insert(key.to_vec(), value.to_vec());
    }
}

#[cfg(test)]
impl<'a> ellipticoin::StorageAPI for TestAPI<'a> {
    fn get(&mut self, key: &[u8]) -> Vec<u8> {
        self.state.storage.get(key).unwrap_or(&vec![]).to_vec()
    }

    fn set(&mut self, key: &[u8], value: &[u8]) {
        self.state.storage.insert(key.to_vec(), value.to_vec());
    }
}

#[cfg(test)]
impl<'a> ellipticoin::API for TestAPI<'a> {
    fn contract_address(&self) -> ([u8; 32], String) {
        self.address.clone()
    }
    fn sender(&self) -> [u8; 32] {
        self.sender
    }
    fn caller(&self) -> Address {
        self.caller.clone()
    }
    fn call<D: DeserializeOwned>(
        &mut self,
        legislator: [u8; 32],
        contract_name: &str,
        function_name: &str,
        arguments: Vec<ellipticoin::wasm_rpc::serde_cbor::Value>,
    ) -> Result<D, Box<ellipticoin::wasm_rpc::error::Error>> {
        let mut transaction = self.transaction.clone();
        transaction.contract_address = (legislator, contract_name.to_string());
        transaction.arguments = arguments;
        transaction.function = function_name.to_string();
        let mut api = TestAPI {
            state: &mut self.state,
            address: (legislator, contract_name.to_string()),
            caller: Address::Contract(self.address.0.clone(), self.address.1.clone()),
            sender: self.sender,
            transaction: transaction.clone(),
        };
        let return_value: serde_cbor::Value = if is_system_contract(&transaction) {
            system_contracts::run2(&mut api, transaction).into()
        } else {
            // transaction.complete((CONTRACT_NOT_FOUND.clone()).into(), transaction.gas_limit).into()
            panic!();
        };
        Ok(serde_cbor::from_slice(&serde_cbor::to_vec(&return_value).unwrap()).unwrap())
    }
}