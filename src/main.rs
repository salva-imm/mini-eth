use std::collections::HashMap;
use std::sync::{LazyLock, RwLock};

struct Transaction {
    from: String,
    to: String,
    nonce: u64,
    value: u64,
    gas_limit: u64,
    gas_price: u64,
    data: Vec<u8>,
    sig: Vec<u8>,
}

struct Receipt {
    status: bool,
    gas_used: u64,
    logs: Vec<String>,
}

struct Block {
    header: String,
    transactions: Vec<Transaction>,
    receipts: Vec<Receipt>,
}

struct BlockHeader {
    parent_hash: String,
    number: u64,
    state_root: String,
    tx_root: String,
    timestamp: u64,
    beneficiary: String,
    gas_limit: u64,
    gas_used: u64,
    extra_data: Vec<u8>,
}

struct Account {
    nonce: u64,
    balance: u64,
    code_hash: String,
    storage_root: String,
}

// Genesis block and state container

static STATE: LazyLock<RwLock<HashMap<String, Block>>> = LazyLock::new(|| {
    let mut m = HashMap::new();
    m.insert("0x00".to_string(), Block {
        header: "genesis_header".to_string(),
        transactions: vec![],
        receipts: vec![],
    });
    RwLock::new(m)
});

fn produce_block(){
    todo!()
}

fn validate_block(){
    todo!()
}

fn execute_transaction(){
    todo!()
}

fn update_state(){
    todo!()
}

fn store_block(){
    todo!()
}

fn query_blockchain(){
    todo!()
}

fn deploy_smart_contract(){
    todo!()
}

fn consensus_mechanism(){
    todo!()
}

fn main() {
    let state = STATE.read().unwrap();
    println!("Hello, world! {:#?}", state.keys());
    let new_block = Block {
        header: "new_block_header".to_string(),
        transactions: vec![],
        receipts: vec![],
    };
    drop(state); // Release read lock before acquiring write lock
    let mut new_state = STATE.write().unwrap();
    new_state.insert("0x01".to_string(), new_block);
    drop(new_state);
    let mut new_block2 = STATE.write().unwrap();
    new_block2.insert("0x02".to_string(), Block {
        header: "another_block_header".to_string(),
        transactions: vec![],
        receipts: vec![],
    });
    drop(new_block2);
    let state = STATE.read().unwrap();
    println!("Hello, world! {:#?}", state.keys());
}
