
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

fn main() {
    println!("Hello, world!");
}
